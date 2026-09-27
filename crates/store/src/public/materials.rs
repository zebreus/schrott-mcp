//! Materials: the price catalog (Kupfer, Messing, Stahlschrott, …).
//!
//! `category` is one of: `eisen`, `nichteisen`, `edelstahl`, `kabel`,
//! `elektronik`, `sonstige`.
//! `unit` is the default quotation unit (`EUR/kg`, `EUR/t`, `EUR/Stk`);
//! individual price observations may deviate and always carry their own unit.

use rusqlite::{params, OptionalExtension as _};

use super::PublicDb;
use crate::error::StoreError;

pub(super) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS materials (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    name_de TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'sonstige',
    unit TEXT NOT NULL DEFAULT 'EUR/kg',
    description TEXT NOT NULL DEFAULT '',
    extra_json TEXT NOT NULL DEFAULT '{}',
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_materials_category ON materials(category);
";

/// One material row, as stored.
#[derive(Debug, Clone)]
pub struct MaterialRow {
    pub id: i64,
    pub slug: String,
    pub name_de: String,
    pub category: String,
    pub unit: String,
    pub description: String,
    pub extra_json: String,
    pub updated_at: String,
}

/// Fields for [`PublicDb::upsert_material`].
pub struct NewMaterial<'a> {
    pub slug: &'a str,
    pub name_de: &'a str,
    pub category: &'a str,
    pub unit: &'a str,
    pub description: &'a str,
    pub updated_at: &'a str,
}

impl PublicDb {
    /// Insert a material or refresh a known one (matched by `slug`).
    pub fn upsert_material(&self, m: &NewMaterial<'_>) -> Result<i64, StoreError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO materials (slug, name_de, category, unit, description, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(slug) DO UPDATE SET
              name_de = excluded.name_de, category = excluded.category,
              unit = excluded.unit, description = excluded.description,
              updated_at = excluded.updated_at",
            params![
                m.slug,
                m.name_de,
                m.category,
                m.unit,
                m.description,
                m.updated_at
            ],
        )?;
        let id: i64 = conn.query_row(
            "SELECT id FROM materials WHERE slug = ?1",
            params![m.slug],
            |r| r.get(0),
        )?;
        Ok(id)
    }

    /// Internal id for a material slug, if known.
    pub fn find_material_id(&self, slug: &str) -> Result<Option<i64>, StoreError> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT id FROM materials WHERE slug = ?1",
            params![slug],
            |r| r.get(0),
        )
        .optional()
        .map_err(StoreError::from)
    }

    /// Whole catalog, ordered by category then German name.
    pub fn list_materials(&self) -> Result<Vec<MaterialRow>, StoreError> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, slug, name_de, category, unit, description, extra_json, updated_at
             FROM materials ORDER BY category, name_de",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(MaterialRow {
                    id: r.get(0)?,
                    slug: r.get(1)?,
                    name_de: r.get(2)?,
                    category: r.get(3)?,
                    unit: r.get(4)?,
                    description: r.get(5)?,
                    extra_json: r.get(6)?,
                    updated_at: r.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::{NewMaterial, PublicDb};

    #[test]
    fn upsert_and_list() {
        let dir = std::env::temp_dir().join(format!("schrott-materials-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let db = PublicDb::open(&dir).expect("test db opens");
        let now = "2026-09-27T00:00:00Z";
        db.upsert_material(&NewMaterial {
            slug: "kupfer-millberry",
            name_de: "Kupfer Millberry",
            category: "nichteisen",
            unit: "EUR/kg",
            description: "Blanker Kupferdraht",
            updated_at: now,
        })
        .expect("insert");
        db.upsert_material(&NewMaterial {
            slug: "kupfer-millberry",
            name_de: "Kupfer Millberry (blank)",
            category: "nichteisen",
            unit: "EUR/kg",
            description: "",
            updated_at: now,
        })
        .expect("update");
        let all = db.list_materials().expect("list");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].name_de, "Kupfer Millberry (blank)");
    }
}
