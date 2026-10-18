// Included only inside recording.rs's existing #[cfg(test)] tests module.
#[tokio::test]
async fn historical_typed_60_to_real_61_preserves_evidence_and_ownership() {
    use rusqlite::{params, params_from_iter, types::Value as SqlValue};
    use serde_json::Value;

    const TABLES: [&str; 5] = [
        "prices",
        "current_prices",
        "trader_materials",
        "source_article_snapshots",
        "article_acceptance_owners",
    ];
    let exported: Value = serde_json::from_str(include_str!(
        "../fixtures/mkm-historical-typed60-harness-20261011.json"
    ))
    .unwrap();
    let old = super::super::overview::parse(HTML, IMP, 200).unwrap();
    assert_eq!(
        (
            old.articles.len(),
            old.outcome.prices.len(),
            old.outcome.acceptances.len()
        ),
        (60, 55, 5)
    );
    assert_eq!(
        old.diagnostics,
        vec!["MKM-MS-01: missing known article; retirement NOT authorized"]
    );
    let oracle: Value = serde_json::from_str(include_str!(
        "../fixtures/mkm-reviewed-mapping-oracle-20261011.json"
    ))
    .unwrap();
    let mut old_articles: Vec<Value> = old
        .articles
        .iter()
        .map(|a| {
            assert_eq!(a.material, oracle["expected"][&a.article][0].as_str());
            assert_eq!(
                a.variant,
                oracle["expected"][&a.article][1].as_str().unwrap()
            );
            serde_json::to_value(a).unwrap()
        })
        .collect();
    old_articles.sort_by_key(|a| a["article"].as_str().unwrap().to_owned());
    let now = DateTime::parse_from_rfc3339("2026-10-18T18:02:00+00:00")
        .unwrap()
        .with_timezone(&Utc);

    // Two independent DBs: a real old-owned transition, and a conflicting
    // pre-existing brass owner. Neither path reconstructs the old writer.
    for conflicting_owner in [false, true] {
        let (dir, db, internal, _) = setup("mkm-real-historical-transition");
        let tid = db.find_trader_id(SLUG).unwrap().unwrap();
        let brass = db.find_material_id("messing").unwrap().unwrap();
        let foreign_mid = db.find_material_id("gold").unwrap().unwrap();
        assert!(old.articles.iter().all(|a| a.material != Some("gold")));
        assert!(
            db.query_sql("SELECT * FROM prices")
                .unwrap()
                .rows
                .is_empty()
        );
        let mut ids = std::collections::BTreeMap::new();
        for (old_id, slug) in exported["materials"].as_object().unwrap() {
            ids.insert(
                old_id.parse::<i64>().unwrap(),
                db.find_material_id(slug.as_str().unwrap())
                    .unwrap()
                    .unwrap(),
            );
        }
        {
            // Logical replay of an actual isolated historical writer's
            // five tables. No production 60-row exception, trigger, mock,
            // or copy of record_articles/Store reconciliation logic.
            let mut conn = rusqlite::Connection::open(dir.path().join("public.db")).unwrap();
            conn.execute_batch("PRAGMA foreign_keys=ON").unwrap();
            let tx = conn.transaction().unwrap();
            for table in TABLES {
                let data = &exported["tables"][table];
                let columns: Vec<&str> = data["columns"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|c| c.as_str().unwrap())
                    .collect();
                let placeholders = vec!["?"; columns.len()].join(",");
                let verb = if table == "trader_materials" {
                    "INSERT OR REPLACE"
                } else {
                    "INSERT"
                };
                let sql = format!(
                    "{verb} INTO {table}({}) VALUES({placeholders})",
                    columns.join(",")
                );
                for source in data["rows"].as_array().unwrap() {
                    let mut row = source.as_array().unwrap().clone();
                    for (i, col) in columns.iter().enumerate() {
                        match *col {
                            "trader_id" => row[i] = serde_json::json!(tid),
                            "material_id" => {
                                row[i] = serde_json::json!(ids[&row[i].as_i64().unwrap()])
                            }
                            "manifest_json" => {
                                let mut manifest: Value =
                                    serde_json::from_str(row[i].as_str().unwrap()).unwrap();
                                assert_eq!(manifest["articles"], serde_json::json!(old_articles));
                                // Only DB-local trader IDs may need remapping.
                                // Otherwise preserve the captured string bytes.
                                if manifest["trader_id"] != tid {
                                    manifest["trader_id"] = serde_json::json!(tid);
                                    row[i] = Value::String(manifest.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                    let values: Vec<SqlValue> = row
                        .iter()
                        .map(|v| match v {
                            Value::Null => SqlValue::Null,
                            Value::String(s) => SqlValue::Text(s.clone()),
                            Value::Number(n) => match n.as_i64() {
                                Some(i) => SqlValue::Integer(i),
                                None => SqlValue::Real(n.as_f64().unwrap()),
                            },
                            _ => panic!("unexpected exported SQLite cell"),
                        })
                        .collect();
                    tx.execute(&sql, params_from_iter(values)).unwrap();
                }
            }
            tx.commit().unwrap();
        }
        assert_eq!(db.current_prices_from(tid, URL).unwrap().len(), 55);
        assert!(current(&db, "messing", "Messing 58 blank neu").is_none());
        for a in &old.articles {
            if !a.request_only {
                let p = current(&db, a.material.unwrap(), &a.variant).unwrap();
                let extra: Value = serde_json::from_str(&p.extra_json).unwrap();
                let mut semantic = serde_json::to_value(a).unwrap();
                semantic
                    .as_object_mut()
                    .unwrap()
                    .remove("product_change_text");
                assert_eq!(extra["snapshot_owner"], OWNER);
                assert_eq!(extra["article"], semantic);
            }
        }
        let old_members: Vec<Value> =
            serde_json::from_str(&db.existing_acceptance(tid, brass).unwrap().unwrap().1).unwrap();
        assert_eq!(old_members.len(), 5);

        // Independent same-trader foreign-source/producer evidence must
        // survive the complete MKM commit, not merely exist on another DB.
        let at = exported["observed_at"].as_str().unwrap();
        let foreign_unit = db.material_unit(foreign_mid).unwrap().unwrap();
        db.record_price(&NewPrice {
            trader_id: tid,
            material_id: foreign_mid,
            variant: "independent foreign",
            price: 70.0,
            currency: "EUR",
            unit: &foreign_unit,
            price_kind: "upto",
            price_min: None,
            price_max: Some(70.0),
            confidence: Some(0.5),
            source_type: "haendler_angabe",
            published: true,
            source_url: "https://example.invalid/independent-source",
            observed_at: at,
            published_at: None,
            valid_from: None,
            valid_to: None,
            notes: "isolated foreign ownership control",
            extra_json: "{\"snapshot_owner\":\"independent-test-owner\"}",
            ingested_at: at,
        })
        .unwrap();
        db.set_acceptance(
            tid,
            foreign_mid,
            true,
            "independent conditions",
            None,
            None,
            at,
            at,
        )
        .unwrap();
        {
            let conn = rusqlite::Connection::open(dir.path().join("public.db")).unwrap();
            conn.execute("INSERT INTO article_acceptance_owners(trader_id,material_id,source_url,owner,conditions,observed_at) VALUES(?1,?2,?3,?4,?5,?6)",
                    params![tid, foreign_mid, "https://example.invalid/independent-source", "independent-test-owner", "independent conditions", at]).unwrap();
            if conflicting_owner {
                assert_eq!(conn.execute("UPDATE article_acceptance_owners SET source_url=?1,owner=?2 WHERE trader_id=?3 AND material_id=?4",
                        params!["https://example.invalid/brass-owner", "independent-brass-owner", tid, brass]).unwrap(), 1);
            }
        }
        let evidence = || {
            TABLES
                .iter()
                .map(|t| {
                    db.query_sql(&format!("SELECT * FROM {t} ORDER BY rowid"))
                        .unwrap()
                        .rows
                })
                .collect::<Vec<_>>()
        };
        let foreign = || {
            ["prices", "current_prices", "trader_materials", "article_acceptance_owners"].iter().map(|t| {
                db.query_sql(&format!("SELECT * FROM {t} WHERE trader_id={tid} AND material_id={foreign_mid} ORDER BY rowid")).unwrap().rows
            }).collect::<Vec<_>>()
        };
        let before = evidence();
        let independent_before = foreign();
        assert_eq!(before[0].len(), 56);
        assert_eq!(before[1].len(), 56);
        assert_eq!(before[3].len(), 1);
        let baseline: Value = serde_json::from_str(before[3][0][5].as_str().unwrap()).unwrap();
        assert_eq!(baseline["articles"], serde_json::json!(old_articles));
        // Verify the exported old acceptance and ownership evidence agree
        // for every historical material before invoking the new writer.
        for (source_id, slug) in exported["materials"].as_object().unwrap() {
            let mid = ids[&source_id.parse::<i64>().unwrap()];
            let rows = db.query_sql(&format!("SELECT tm.conditions,o.conditions,tm.observed_at,o.observed_at,o.source_url,o.owner FROM trader_materials tm JOIN article_acceptance_owners o USING(trader_id,material_id) WHERE tm.trader_id={tid} AND tm.material_id={mid}")).unwrap().rows;
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0][0], rows[0][1]);
            assert_eq!(rows[0][2], rows[0][3]);
            if !conflicting_owner || mid != brass {
                assert_eq!(rows[0][4], URL);
                assert_eq!(rows[0][5], OWNER);
            }
            let expected: Vec<Value> = old_articles
                .iter()
                .filter(|a| a["material"] == *slug)
                .map(|a| {
                    let mut a = a.clone();
                    a.as_object_mut().unwrap().remove("product_change_text");
                    a
                })
                .collect();
            assert_eq!(
                serde_json::from_str::<Value>(rows[0][0].as_str().unwrap()).unwrap(),
                serde_json::json!(expected)
            );
        }

        let next = super::super::overview::parse(
            include_str!("../fixtures/mkm-overview-20261018-sanitized.html"),
            include_str!("../fixtures/mkm-impressum-20261018-sanitized.html"),
            200,
        )
        .unwrap();
        assert!(next.diagnostics.is_empty());
        let result = commit(&db, &internal, next, now).await;
        if conflicting_owner {
            let error = result.unwrap_err().to_string();
            assert!(
                error.contains("foreign acceptance producer"),
                "must reach ownership guard, not fail on cardinality: {error}"
            );
            assert_eq!(
                evidence(),
                before,
                "all fields of all five tables must remain unchanged"
            );
            continue;
        }
        let summary =
            result.expect("historical typed 60 -> actual 61 must commit through record_articles");
        assert_eq!((summary.recorded, summary.accepted), (56, 5));
        let after = evidence();
        for row in &before[0] {
            assert!(
                after[0].contains(row),
                "every pre-existing historical field is immutable"
            );
        }
        assert_eq!(after[3].len(), 2);
        assert_eq!(
            after[3][0], before[3][0],
            "old manifest string and ledger fields remain byte-identical"
        );
        assert_eq!(foreign(), independent_before);
        assert_eq!(db.current_prices_from(tid, URL).unwrap().len(), 56);
        assert_eq!(after[1].len(), 57);
        let added = current(&db, "messing", "Messing 58 blank neu").unwrap();
        let added_extra: Value = serde_json::from_str(&added.extra_json).unwrap();
        assert_eq!(added_extra["article"]["article"], "MKM-MS-01");
        assert_eq!(added.price, 6.51);
        assert_eq!(added.price_max, Some(6.51));
        assert_eq!(added.price_min, None);
        assert_eq!(
            (added.published_at, added.valid_from, added.valid_to),
            (None, None, None)
        );
        for member in &old_members {
            let p = current(&db, "messing", member["variant"].as_str().unwrap()).unwrap();
            let extra: Value = serde_json::from_str(&p.extra_json).unwrap();
            assert_eq!(extra["article"]["article"], member["article"]);
            assert_ne!(p.id, added.id);
        }
        let new_manifest: Value = serde_json::from_str(after[3][1][5].as_str().unwrap()).unwrap();
        assert_eq!(new_manifest["articles"].as_array().unwrap().len(), 61);
        for article in new_manifest["articles"].as_array().unwrap() {
            let material = article["material"].as_str().unwrap();
            let variant = article["variant"].as_str().unwrap();
            let p = current(&db, material, variant);
            if article["request_only"] == true {
                assert!(p.is_none());
            } else {
                let p = p.unwrap();
                let mut semantic = article.clone();
                semantic
                    .as_object_mut()
                    .unwrap()
                    .remove("product_change_text");
                let extra: Value = serde_json::from_str(&p.extra_json).unwrap();
                assert_eq!(extra["article"], semantic);
                assert_eq!(extra["snapshot_owner"], OWNER);
                assert_eq!(p.source_url, URL);
            }
        }
        assert_eq!(after[2].len(), before[2].len());
        assert_eq!(after[4].len(), before[4].len());
        for (old_id, slug) in exported["materials"].as_object().unwrap() {
            let mid = ids[&old_id.parse::<i64>().unwrap()];
            let expected: Vec<Value> = new_manifest["articles"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|a| a["material"] == *slug)
                .map(|a| {
                    let mut a = a.clone();
                    a.as_object_mut().unwrap().remove("product_change_text");
                    a
                })
                .collect();
            let rows = db.query_sql(&format!("SELECT tm.accepts,tm.conditions,tm.observed_at,tm.updated_at,tm.valid_from,tm.valid_to,o.source_url,o.owner,o.conditions,o.observed_at FROM trader_materials tm JOIN article_acceptance_owners o USING(trader_id,material_id) WHERE tm.trader_id={tid} AND tm.material_id={mid}")).unwrap().rows;
            assert_eq!(rows.len(), 1);
            let r = &rows[0];
            assert_eq!(r[0], 1);
            assert_eq!(
                serde_json::from_str::<Value>(r[1].as_str().unwrap()).unwrap(),
                serde_json::json!(expected)
            );
            assert_eq!(r[1], r[8]);
            assert_eq!(r[2], new_manifest["observed_at"]);
            assert_eq!(r[3], r[2]);
            assert_eq!(r[9], r[2]);
            assert!(r[4].is_null() && r[5].is_null());
            assert_eq!(r[6], URL);
            assert_eq!(r[7], OWNER);
            if mid == brass {
                assert_eq!(expected.len(), 6);
            }
        }
        // The genuine old source is still rejected by today's writer;
        // replaying it must not undo the successful expansion.
        let completed = evidence();
        let stale = super::super::overview::parse(HTML, IMP, 200).unwrap();
        let error = commit(&db, &internal, stale, now + chrono::Duration::minutes(1))
            .await
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("incomplete or invalid MKM source contract")
        );
        assert_eq!(evidence(), completed);
    }
}
