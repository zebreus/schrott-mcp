//! Server-rendered pages: marketing site, auth forms, consent, dashboard.
//! No frontend build step — one inline stylesheet, system fonts, dark theme.
//! Alles auf Deutsch, im Schrott-Look (Rostorange + Stahl).
//!
//! Conventions: typed input is echoed back on errors (passwords never),
//! one-time secrets render from server-side flash state, never from URLs.

use schrott_mcp_core::{url_encode, Stats};
use schrott_mcp_store::internal::{ApiTokenView, RunRow};

use super::style::CSS;

/// Shared form panel: title, subtitle, optional error banner, then content.
pub fn panel(title: &str, sub: &str, err: Option<&str>, inner: &str) -> String {
    let err_html = err.map_or(String::new(), |e| {
        format!("<div class=\"err\" role=\"alert\">{}</div>", esc(e))
    });
    let sub_html = if sub.is_empty() {
        String::new()
    } else {
        format!("<p class=\"sub\">{sub}</p>")
    };
    format!(
        "<div class=\"panel\"><h2>{}</h2>{sub_html}{err_html}{inner}</div>",
        esc(title)
    )
}

/// Minimal HTML escaping for user-controlled strings.
pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Full page shell: nav, document metadata and landmarks.
/// `description`/`canonical` feed meta + OpenGraph tags; `csrf` feeds the
/// logout form and `None` hides it.
pub fn layout(
    title: &str,
    description: &str,
    canonical: &str,
    user: Option<&str>,
    csrf: Option<&str>,
    body: &str,
) -> String {
    let auth_links = match user {
        Some(name) => {
            let csrf_field = csrf.map_or(String::new(), |c| {
                format!("<input type=\"hidden\" name=\"csrf\" value=\"{}\">", esc(c))
            });
            format!(
                "<a class=\"l\" href=\"/dashboard\">{}</a>\
                 <form method=\"post\" action=\"/logout\" style=\"display:inline;margin:0\">{csrf_field}\
                 <button class=\"btn ghost\" style=\"padding:8px 14px;margin-left:12px\" type=\"submit\">Abmelden</button></form>",
                esc(name)
            )
        }
        None => "<a class=\"l\" href=\"/login\">Anmelden</a>\
                 <a class=\"btn\" style=\"padding:8px 16px;margin-left:12px\" href=\"/signup\">Registrieren</a>"
            .to_owned(),
    };
    format!(
        "<!doctype html><html lang=\"de\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
         <meta name=\"description\" content=\"{}\">\
         <meta name=\"theme-color\" content=\"#0e0d0c\">\
         <meta property=\"og:type\" content=\"website\">\
         <meta property=\"og:site_name\" content=\"Schrott MCP\">\
         <meta property=\"og:title\" content=\"{title} · Schrott MCP\">\
         <meta property=\"og:description\" content=\"{}\">\
         <meta property=\"og:url\" content=\"{}\">\
         <meta name=\"twitter:card\" content=\"summary\">\
         <meta name=\"twitter:title\" content=\"{title} · Schrott MCP\">\
         <meta name=\"twitter:description\" content=\"{}\">\
         <link rel=\"icon\" href=\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Ccircle cx='50' cy='50' r='42' fill='%23e8832a'/%3E%3Ccircle cx='50' cy='50' r='17' fill='%230e0d0c'/%3E%3C/svg%3E\">\
         <title>{title} · Schrott MCP</title><style>{CSS}</style></head><body>\
         <a class=\"skip\" href=\"{skip_target}\">Zum Inhalt springen</a>\
         <header><nav aria-label=\"Konto\"><div class=\"wrap\"><a class=\"brand\" href=\"/\">Schrott<span>MCP</span></a>\
         <div class=\"sp\"></div>{auth_links}</div></nav></header>\
         <main id=\"main\"><div class=\"wrap\">{body}</div></main>\
         <footer><div class=\"wrap\"><span>Mit Liebe für die Schrottbranche gemacht.</span></div></footer>\
         </body></html>",
        esc(description),
        esc(description),
        esc(canonical),
        esc(description),
        skip_target = "#main",
    )
}

/// Stylish marketing front page.
pub fn marketing(base_url: &str, user: Option<&str>) -> String {
    let mcp = format!("{base_url}/mcp");
    let ctas = match user {
        Some(_) => "<a class=\"btn\" href=\"/dashboard\">Zu deinem Dashboard</a>",
        None => {
            "<a class=\"btn\" href=\"/signup\">Kostenlos registrieren</a> \
                 <a class=\"btn ghost\" href=\"/login\">Anmelden</a>"
        }
    };
    let body = format!(
        "<div class=\"hero\"><h1>Schrottpreise aus Deutschland,<br><em>bereit für deinen Agenten.</em></h1>\
         <p>Schrott MCP sammelt Schrotthändler, Wertstoffhändler &amp; Co. aus ganz Deutschland \
         in einem sauberen, abfragbaren Modell — mit aktuellen und historischen Preisen, \
         transparenter Quellenlage und Anbindung per Model Context Protocol.</p>\
         <div>{ctas}</div>\
         <div class=\"connect\"><code>{mcp}</code><span>← deine MCP-Server-URL</span></div></div>\
         <div class=\"grid\">\
         <div class=\"card\"><h2>Eine URL, keine Konfiguration</h2><p>Trage die MCP-Server-URL in deinem \
         MCP-Host ein. OAuth-Erkennung und dynamische Client-Registrierung übernehmen die Anmeldung.</p></div>\
         <div class=\"card\"><h2>Ein Modell, das man versteht</h2><p>Händler → Materialien → Preise, \
         abfragbar über ein einziges lesendes <code>sql</code>-Werkzeug. Das Schema steht in der \
         Werkzeugbeschreibung, Agenten raten nie.</p></div>\
         <div class=\"card\"><h2>Ehrliche Datenerfassung</h2><p>Jede Preisbeobachtung trägt Quelle, \
         Gültigkeitszeitraum und Unsicherheit mit sich. Ob der Händler den Preis selbst \
         veröffentlicht hat, steht dabei.</p></div>\
         </div>\
         <div class=\"steps\"><h2>In drei Schritten verbunden</h2>\
         <div class=\"step\"><div class=\"n\">1</div><div><b>Registrieren</b> — Benutzername, Passwort, ein Häkchen. \
         Keine E-Mail, kein Telefon, kein Verifizierungstheater.</div></div>\
         <div class=\"step\"><div class=\"n\">2</div><div><b><code>{mcp}</code></b> in deinen MCP-Host eintragen \
         (Claude, OpenCode, alles mit Streamable HTTP).</div></div>\
         <div class=\"step\"><div class=\"n\">3</div><div><b>Per OAuth anmelden</b>, wenn dein Host den Browser \
         öffnet — und dann fragen, z.&nbsp;B.: „Was zahlt Händler X aktuell für Kupfer?“</div></div></div>"
    );
    layout(
        "Schrottdaten per MCP",
        "Schrottpreise und Händlerdaten aus Deutschland per MCP — eine URL, saubere Datensätze, OAuth-Login.",
        &format!("{base_url}/"),
        user,
        None,
        &body,
    )
}

/// Login form. Typed username is echoed back on errors; `next` survives too.
pub fn login(base_url: &str, next: &str, username: &str, err: Option<&str>) -> String {
    let body = panel(
        "Willkommen zurück",
        "Melde dich an, um Token &amp; Daten zu verwalten.",
        err,
        &format!(
            "<form method=\"post\" action=\"/login\" autocomplete=\"on\">\
             <input type=\"hidden\" name=\"next\" value=\"{}\">\
             <label for=\"login-user\">Benutzername</label>\
             <input id=\"login-user\" type=\"text\" name=\"username\" value=\"{}\" autocomplete=\"username\" \
             autocapitalize=\"off\" autocorrect=\"off\" spellcheck=\"false\" maxlength=\"32\" required>\
             <label for=\"login-pass\">Passwort</label>\
             <input id=\"login-pass\" type=\"password\" name=\"password\" autocomplete=\"current-password\" required>\
             <div class=\"rowb\"><button class=\"btn\" type=\"submit\">Anmelden</button>\
             <a class=\"btn ghost\" href=\"/signup?next={}\">Noch kein Konto?</a></div></form>",
            esc(next),
            esc(username),
            esc(&url_encode(next))
        ),
    );
    layout(
        "Anmelden",
        "Melde dich bei Schrott MCP an, um MCP-Token und Daten zu verwalten.",
        &format!("{base_url}/login"),
        None,
        None,
        &body,
    )
}

/// Signup form: username + password + professional checkbox. Nothing else.
/// Username and checkbox survive validation errors; passwords never echo.
/// `next` survives the login↔signup hop so OAuth flows complete.
pub fn signup(
    base_url: &str,
    next: &str,
    username: &str,
    professional: bool,
    err: Option<&str>,
) -> String {
    let checked = if professional { " checked" } else { "" };
    let body = panel(
        "Erstelle dein Konto",
        "Benutzername, Passwort, ein Häkchen. Das ist der ganze Antrag.",
        err,
        &format!(
            "<form method=\"post\" action=\"/signup\" autocomplete=\"on\">\
             <input type=\"hidden\" name=\"next\" value=\"{}\">\
             <label for=\"signup-user\">Benutzername</label>\
             <input id=\"signup-user\" type=\"text\" name=\"username\" value=\"{}\" autocomplete=\"username\" \
             autocapitalize=\"off\" autocorrect=\"off\" spellcheck=\"false\" maxlength=\"32\" required>\
             <p class=\"hint\" id=\"signup-user-hint\">3–32 Zeichen: Buchstaben, Ziffern, '_' und '-'.</p>\
             <label for=\"signup-pass\">Passwort (min. 8 Zeichen)</label>\
             <input id=\"signup-pass\" type=\"password\" name=\"password\" autocomplete=\"new-password\" \
             minlength=\"8\" aria-describedby=\"signup-pass-hint\" required>\
             <p class=\"hint\" id=\"signup-pass-hint\">Mindestens 8 Zeichen — alles andere ist egal.</p>\
             <label for=\"signup-confirm\">Passwort bestätigen</label>\
             <input id=\"signup-confirm\" type=\"password\" name=\"confirm\" autocomplete=\"new-password\" \
             minlength=\"8\" required>\
             <label class=\"check\"><input type=\"checkbox\" name=\"professional\" value=\"yes\"{checked}>\
             <span>Ich bestätige, dass ich <b>professioneller Datennutzer</b> bin und mit den Daten verantwortungsvoll umgehe.</span></label>\
             <div class=\"rowb\"><button class=\"btn\" type=\"submit\">Registrieren</button>\
             <a class=\"btn ghost\" href=\"/login?next={}\">Schon ein Konto?</a></div></form>",
            esc(next),
            esc(username),
            esc(&url_encode(next))
        ),
    );
    layout(
        "Registrieren",
        "Erstelle ein kostenloses Schrott-MCP-Konto: Benutzername, Passwort, ein Häkchen.",
        &format!("{base_url}/signup"),
        None,
        None,
        &body,
    )
}

/// Inputs for the OAuth consent screen.
pub struct ConsentData<'a> {
    pub client_id: &'a str,
    pub redirect_uri: &'a str,
    pub scope: &'a str,
    pub raw_query: &'a str,
    pub username: &'a str,
    pub csrf: &'a str,
    pub code_challenge: &'a str,
    pub oauth_state: Option<&'a str>,
}

/// OAuth consent screen shown to the resource owner. The request fields
/// ride along as hidden inputs so the POST does not depend on the query.
pub fn consent(c: ConsentData<'_>, base_url: &str) -> String {
    let state_field = c.oauth_state.map_or(String::new(), |s| {
        format!(
            "<input type=\"hidden\" name=\"state\" value=\"{}\">",
            esc(s)
        )
    });
    let intro = format!(
        "<p>Angemeldet als <b>{}</b>.</p>\
         <p>Anwendung <code>{}</code> möchte <b>{}</b>-Zugriff auf den geteilten Schrott-Datenbestand \
         und leitet danach zurück zu<br><code>{}</code></p>",
        esc(c.username),
        esc(c.client_id),
        esc(c.scope),
        esc(c.redirect_uri)
    );
    let body = panel(
        "Zugriff erlauben",
        "",
        None,
        &format!(
            "{intro}\
             <form method=\"post\" action=\"/oauth/authorize?{}\">\
             <input type=\"hidden\" name=\"csrf\" value=\"{}\">\
             <input type=\"hidden\" name=\"client_id\" value=\"{}\">\
             <input type=\"hidden\" name=\"redirect_uri\" value=\"{}\">\
             <input type=\"hidden\" name=\"scope\" value=\"{}\">\
             <input type=\"hidden\" name=\"code_challenge\" value=\"{}\">\
             {state_field}\
             <div class=\"rowb\"><button class=\"btn\" name=\"decision\" value=\"approve\" type=\"submit\">Erlauben</button>\
             <button class=\"btn ghost\" name=\"decision\" value=\"deny\" type=\"submit\">Ablehnen</button></div>\
             </form>",
            esc(c.raw_query),
            esc(c.csrf),
            esc(c.client_id),
            esc(c.redirect_uri),
            esc(c.scope),
            esc(c.code_challenge)
        ),
    );
    layout(
        "Erlauben",
        "Erlaube einem MCP-Client den Zugriff auf dein Schrott-MCP-Konto.",
        &format!("{base_url}/oauth/authorize"),
        Some(c.username),
        Some(c.csrf),
        &body,
    )
}

/// Styled error page inside the normal layout.
pub fn error_page(
    base_url: &str,
    path: &str,
    title: &str,
    message: &str,
    user: Option<&str>,
    csrf: Option<&str>,
) -> String {
    let body = panel(
        title,
        "",
        None,
        &format!(
            "<div class=\"err\">{}</div>\
             <div class=\"rowb\"><a class=\"btn ghost\" href=\"/\">Zurück zur Startseite</a></div>",
            esc(message)
        ),
    );
    layout(
        title,
        "Schrott MCP",
        &format!("{base_url}{path}"),
        user,
        csrf,
        &body,
    )
}

/// Data handed to the dashboard renderer.
pub struct DashboardData<'a> {
    pub username: &'a str,
    pub csrf: &'a str,
    pub stats: &'a Stats,
    pub tokens: &'a [ApiTokenView],
    pub runs: &'a [RunRow],
    /// Freshly created secret: rendered once from flash state, never a URL.
    pub flash_secret: Option<&'a str>,
    /// One-time confirmation notice from the last POST.
    pub flash_notice: Option<&'a str>,
}

/// Account dashboard: MCP instructions, tokens, corpus stats, pipeline runs.
pub fn dashboard(base_url: &str, d: DashboardData<'_>) -> String {
    let mcp = format!("{base_url}/mcp");
    let secret = d.flash_secret.map_or(String::new(), |s| {
        format!(
            "<div class=\"ok\"><b>Neues persönliches Token — jetzt kopieren. Wer diese Seite verlässt, sieht es nie wieder:</b>\
             <div class=\"connect\"><code id=\"newtoken\">{}</code>\
             <button class=\"btn\" style=\"padding:8px 14px\" type=\"button\" \
             onclick=\"navigator.clipboard.writeText(document.getElementById('newtoken').textContent);this.textContent='Kopiert ✓'\">Kopieren</button>\
             </div></div>",
            esc(s)
        )
    });
    let notice = d.flash_notice.map_or(String::new(), |n| {
        format!("<div class=\"ok\">{}</div>", esc(n))
    });
    let onboarding = if d.tokens.is_empty() {
        format!(
            "<div class=\"card\"><h3>In einer Minute startklar</h3>\
             <ol class=\"howto\"><li>Lege unten ein Token an und sende es als \
             <code>Authorization: Bearer</code>-Header — oder überspringe Token und melde deinen MCP-Host per OAuth an.</li>\
             <li>Zeige deinen MCP-Host auf <code>{mcp}</code>.</li>\
             <li>Warte die Datenerfassung ab (oder stoße sie an), bis der Bestand gefüllt ist — dann frag los.</li></ol></div>"
        )
    } else {
        String::new()
    };
    let mut token_rows = String::new();
    for t in d.tokens {
        token_rows.push_str(&format!(
            "<tr><td>{}</td><td><code>{}…</code></td><td>{}</td><td>{}</td>\
             <td><form method=\"post\" action=\"/tokens/delete\" style=\"margin:0\">\
             <input type=\"hidden\" name=\"csrf\" value=\"{}\">\
             <input type=\"hidden\" name=\"id\" value=\"{}\">\
             <button class=\"btn ghost\" style=\"padding:6px 12px\" type=\"submit\" \
             onclick=\"return confirm('Token \\\"{}\\\" wirklich widerrufen? Verbundene Clients funktionieren dann nicht mehr.')\">Widerrufen</button>\
             </form></td></tr>",
            esc(&t.name),
            esc(&t.prefix),
            esc(&t.created_at),
            esc(t.last_used_at.as_deref().unwrap_or("nie")),
            esc(d.csrf),
            t.id,
            esc(&t.name)
        ));
    }
    if token_rows.is_empty() {
        token_rows = "<tr><td colspan=\"5\" style=\"color:var(--muted)\">Noch keine Token — lege unten eins an.</td></tr>".to_owned();
    }
    let mut run_rows = String::new();
    for r in d.runs {
        run_rows.push_str(&format!(
            "<tr><td>#{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            r.id,
            esc(&r.started_at),
            esc(r.finished_at.as_deref().unwrap_or("läuft …")),
            esc(&r.status),
            esc(&r.detail)
        ));
    }
    if run_rows.is_empty() {
        run_rows =
            "<tr><td colspan=\"5\" style=\"color:var(--muted)\">Noch keine Läufe.</td></tr>".to_owned();
    }
    let body = format!(
        "<h2 style=\"margin-top:36px\">Hallo, {}.</h2>{secret}{notice}{onboarding}\
         <div class=\"connect\"><code>{mcp}</code><span>deine MCP-Server-URL</span></div>\
         <div class=\"dash\">\
         <div class=\"stat\"><div class=\"v\">{}</div><div class=\"k\">Händler</div></div>\
         <div class=\"stat\"><div class=\"v\">{}</div><div class=\"k\">Materialien</div></div>\
         <div class=\"stat\"><div class=\"v\">{}</div><div class=\"k\">Preisbeobachtungen</div></div></div>\
         <div class=\"card\"><h3>Persönliche Zugriffstoken</h3>\
         <p>Lieber ein festes Token als OAuth? Lege eins an und nutze es als \
         <code>Authorization: Bearer</code>-Header für deinen MCP-Client.</p>\
         <form method=\"post\" action=\"/tokens/create\" class=\"tokrow\">\
         <input type=\"hidden\" name=\"csrf\" value=\"{}\">\
         <input type=\"text\" name=\"name\" placeholder=\"Token-Name (z. B. claude)\" aria-label=\"Token-Name\" autocomplete=\"off\">\
         <button class=\"btn\" type=\"submit\">Anlegen</button></form>\
         <div class=\"tablewrap\"><table><tr><th scope=\"col\">Name</th><th scope=\"col\">Präfix</th><th scope=\"col\">Erstellt</th><th scope=\"col\">Zuletzt genutzt</th><th scope=\"col\"><span class=\"vh\">Aktionen</span></th></tr>{token_rows}</table></div></div>\
         <div class=\"card\"><h3>Datenerfassung</h3>\
         <p>Läuft alle 6 Stunden in diesem Prozess: Materialkatalog plus \
         Händler-Seed (über 2.300 Händler aus der Recherche) werden \
         idempotent aufgefrischt; Händler-Scraper für Preise folgen später. \
         Beobachte die Tabelle unten.</p>\
         <form method=\"post\" action=\"/api/ingest/run\">\
         <input type=\"hidden\" name=\"csrf\" value=\"{}\">\
         <button class=\"btn\" type=\"submit\">Jetzt erfassen</button></form>\
         <div class=\"tablewrap\"><table style=\"margin-top:12px\"><tr><th scope=\"col\">Lauf</th><th scope=\"col\">Gestartet</th><th scope=\"col\">Beendet</th><th scope=\"col\">Status</th><th scope=\"col\">Detail</th></tr>{run_rows}</table></div></div>",
        esc(d.username),
        d.stats.traders,
        d.stats.materials,
        d.stats.prices,
        esc(d.csrf),
        esc(d.csrf)
    );
    layout(
        "Dashboard",
        "Dein Schrott-MCP-Dashboard: MCP-Token, Erfassungsläufe und Bestandszahlen.",
        &format!("{base_url}/dashboard"),
        Some(d.username),
        Some(d.csrf),
        &body,
    )
}
