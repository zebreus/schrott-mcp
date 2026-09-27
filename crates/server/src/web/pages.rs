//! Server-rendered pages: marketing site, auth forms, consent, dashboard.
//! No frontend build step — one inline stylesheet, system fonts, dark theme.
//!
//! Conventions: typed input is echoed back on errors (passwords never),
//! one-time secrets render from server-side flash state, never from URLs.

use offsite_data_core::{url_encode, Stats};
use offsite_data_store::internal::{ApiTokenView, RunRow};

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
                 <button class=\"btn ghost\" style=\"padding:8px 14px;margin-left:12px\" type=\"submit\">Log out</button></form>",
                esc(name)
            )
        }
        None => "<a class=\"l\" href=\"/login\">Log in</a>\
                 <a class=\"btn\" style=\"padding:8px 16px;margin-left:12px\" href=\"/signup\">Sign up</a>"
            .to_owned(),
    };
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
         <meta name=\"description\" content=\"{}\">\
         <meta name=\"theme-color\" content=\"#0b0e17\">\
         <meta property=\"og:type\" content=\"website\">\
         <meta property=\"og:site_name\" content=\"Offsite Data\">\
         <meta property=\"og:title\" content=\"{title} · Offsite Data\">\
         <meta property=\"og:description\" content=\"{}\">\
         <meta property=\"og:url\" content=\"{}\">\
         <meta name=\"twitter:card\" content=\"summary\">\
         <meta name=\"twitter:title\" content=\"{title} · Offsite Data\">\
         <meta name=\"twitter:description\" content=\"{}\">\
         <link rel=\"icon\" href=\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Ccircle cx='50' cy='50' r='42' fill='%237c6cff'/%3E%3Ccircle cx='50' cy='50' r='16' fill='%233ddc97'/%3E%3C/svg%3E\">\
         <title>{title} · Offsite Data</title><style>{CSS}</style></head><body>\
         <a class=\"skip\" href=\"{skip_target}\">Skip to content</a>\
         <header><nav aria-label=\"Account\"><div class=\"wrap\"><a class=\"brand\" href=\"/\">Offsite<span>Data</span></a>\
         <div class=\"sp\"></div>{auth_links}</div></nav></header>\
         <main id=\"main\"><div class=\"wrap\">{body}</div></main>\
         <footer><div class=\"wrap\"><span>Made with love for domain experts.</span></div></footer>\
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
        Some(_) => "<a class=\"btn\" href=\"/dashboard\">Go to your dashboard</a>",
        None => {
            "<a class=\"btn\" href=\"/signup\">Create a free account</a> \
                 <a class=\"btn ghost\" href=\"/login\">Log in</a>"
        }
    };
    let body = format!(
        "<div class=\"hero\"><h1>Scraped domain data,<br><em>ready for your agent.</em></h1>\
         <p>Offsite Data ingests wildly different websites into one clean, queriable model \
         and serves it over the Model Context Protocol — with love as the secret ingredient.</p>\
         <div>{ctas}</div>\
         <div class=\"connect\"><code>{mcp}</code><span>← your MCP server URL</span></div></div>\
         <div class=\"grid\">\
         <div class=\"card\"><h2>One URL, zero config</h2><p>Add the MCP server URL to your MCP host. \
         Built-in OAuth discovery and dynamic client registration handle login for you.</p></div>\
         <div class=\"card\"><h2>A model humans get</h2><p>Sources → datasets → items, \
         queriable through a single read-only <code>sql</code> tool. The schema rides along \
         in the tool description, so agents query without guessing.</p></div>\
         <div class=\"card\"><h2>Honest ingestion</h2><p>A tiny fetch → parse → normalize → diff → upsert \
         pipeline. Only changed records are rewritten; every fetch is journaled.</p></div>\
         </div>\
         <div class=\"steps\"><h2>Connect in three steps</h2>\
         <div class=\"step\"><div class=\"n\">1</div><div><b>Sign up</b> — username, password, one checkbox. \
         No email, no phone, no verification theatre.</div></div>\
         <div class=\"step\"><div class=\"n\">2</div><div><b>Add <code>{mcp}</code></b> to your MCP host \
         (Claude, OpenCode, anything speaking Streamable HTTP).</div></div>\
         <div class=\"step\"><div class=\"n\">3</div><div><b>Log in via OAuth</b> when your host opens the \
         browser — then ask your domain questions.</div></div></div>"
    );
    layout(
        "Domain data over MCP",
        "Scraped domain data over MCP — one URL, clean datasets, OAuth login.",
        &format!("{base_url}/"),
        user,
        None,
        &body,
    )
}

/// Login form. Typed username is echoed back on errors; `next` survives too.
pub fn login(base_url: &str, next: &str, username: &str, err: Option<&str>) -> String {
    let body = panel(
        "Welcome back",
        "Log in to manage tokens &amp; data.",
        err,
        &format!(
            "<form method=\"post\" action=\"/login\" autocomplete=\"on\">\
             <input type=\"hidden\" name=\"next\" value=\"{}\">\
             <label for=\"login-user\">Username</label>\
             <input id=\"login-user\" type=\"text\" name=\"username\" value=\"{}\" autocomplete=\"username\" \
             autocapitalize=\"off\" autocorrect=\"off\" spellcheck=\"false\" maxlength=\"32\" required>\
             <label for=\"login-pass\">Password</label>\
             <input id=\"login-pass\" type=\"password\" name=\"password\" autocomplete=\"current-password\" required>\
             <div class=\"rowb\"><button class=\"btn\" type=\"submit\">Log in</button>\
             <a class=\"btn ghost\" href=\"/signup?next={}\">Need an account?</a></div></form>",
            esc(next),
            esc(username),
            esc(&url_encode(next))
        ),
    );
    layout(
        "Log in",
        "Log in to Offsite Data to manage MCP tokens and data.",
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
        "Create your account",
        "Username, password, one checkbox. That's the whole application.",
        err,
        &format!(
            "<form method=\"post\" action=\"/signup\" autocomplete=\"on\">\
             <input type=\"hidden\" name=\"next\" value=\"{}\">\
             <label for=\"signup-user\">Username</label>\
             <input id=\"signup-user\" type=\"text\" name=\"username\" value=\"{}\" autocomplete=\"username\" \
             autocapitalize=\"off\" autocorrect=\"off\" spellcheck=\"false\" maxlength=\"32\" required>\
             <p class=\"hint\" id=\"signup-user-hint\">3–32 characters: letters, digits, '_' and '-'.</p>\
             <label for=\"signup-pass\">Password (min. 8 characters)</label>\
             <input id=\"signup-pass\" type=\"password\" name=\"password\" autocomplete=\"new-password\" \
             minlength=\"8\" aria-describedby=\"signup-pass-hint\" required>\
             <p class=\"hint\" id=\"signup-pass-hint\">At least 8 characters — anything else goes.</p>\
             <label for=\"signup-confirm\">Confirm password</label>\
             <input id=\"signup-confirm\" type=\"password\" name=\"confirm\" autocomplete=\"new-password\" \
             minlength=\"8\" required>\
             <label class=\"check\"><input type=\"checkbox\" name=\"professional\" value=\"yes\"{checked}>\
             <span>I confirm that I am a <b>professional data-user</b> and will treat scraped data responsibly.</span></label>\
             <div class=\"rowb\"><button class=\"btn\" type=\"submit\">Sign up</button>\
             <a class=\"btn ghost\" href=\"/login?next={}\">Have an account?</a></div></form>",
            esc(next),
            esc(username),
            esc(&url_encode(next))
        ),
    );
    layout(
        "Sign up",
        "Create a free Offsite Data account: username, password, one checkbox.",
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
        "<p>Signed in as <b>{}</b>.</p>\
         <p>Application <code>{}</code> wants <b>{}</b> access to the shared queriable dataset \
         and will redirect back to<br><code>{}</code></p>",
        esc(c.username),
        esc(c.client_id),
        esc(c.scope),
        esc(c.redirect_uri)
    );
    let body = panel(
        "Authorize access",
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
             <div class=\"rowb\"><button class=\"btn\" name=\"decision\" value=\"approve\" type=\"submit\">Authorize</button>\
             <button class=\"btn ghost\" name=\"decision\" value=\"deny\" type=\"submit\">Deny</button></div>\
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
        "Authorize",
        "Authorize an MCP client to access your Offsite Data account.",
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
             <div class=\"rowb\"><a class=\"btn ghost\" href=\"/\">Back home</a></div>",
            esc(message)
        ),
    );
    layout(
        title,
        "Offsite Data",
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
            "<div class=\"ok\"><b>New personal token — copy it now. Leaving this page loses it forever:</b>\
             <div class=\"connect\"><code id=\"newtoken\">{}</code>\
             <button class=\"btn\" style=\"padding:8px 14px\" type=\"button\" \
             onclick=\"navigator.clipboard.writeText(document.getElementById('newtoken').textContent);this.textContent='Copied ✓'\">Copy</button>\
             </div></div>",
            esc(s)
        )
    });
    let notice = d.flash_notice.map_or(String::new(), |n| {
        format!("<div class=\"ok\">{}</div>", esc(n))
    });
    let onboarding = if d.tokens.is_empty() {
        format!(
            "<div class=\"card\"><h3>Get started in a minute</h3>\
             <ol class=\"howto\"><li>Create a token below, then send it as your \
             <code>Authorization: Bearer</code> header — or skip tokens and let your MCP host log in via OAuth.</li>\
             <li>Point your MCP host at <code>{mcp}</code>.</li>\
             <li>Run ingestion (or wait for the 6-hour schedule) so the corpus fills up, then ask away.</li></ol></div>"
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
             onclick=\"return confirm('Revoke token \\\"{}\\\"? Connected clients using it will stop working.')\">Revoke</button>\
             </form></td></tr>",
            esc(&t.name),
            esc(&t.prefix),
            esc(&t.created_at),
            esc(t.last_used_at.as_deref().unwrap_or("never")),
            esc(d.csrf),
            t.id,
            esc(&t.name)
        ));
    }
    if token_rows.is_empty() {
        token_rows = "<tr><td colspan=\"5\" style=\"color:var(--muted)\">No tokens yet — create one below.</td></tr>".to_owned();
    }
    let mut run_rows = String::new();
    for r in d.runs {
        run_rows.push_str(&format!(
            "<tr><td>#{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            r.id,
            esc(&r.started_at),
            esc(r.finished_at.as_deref().unwrap_or("running…")),
            esc(&r.status),
            esc(&r.detail)
        ));
    }
    if run_rows.is_empty() {
        run_rows =
            "<tr><td colspan=\"5\" style=\"color:var(--muted)\">No runs yet.</td></tr>".to_owned();
    }
    let body = format!(
        "<h2 style=\"margin-top:36px\">Hi, {}.</h2>{secret}{notice}{onboarding}\
         <div class=\"connect\"><code>{mcp}</code><span>your MCP server URL</span></div>\
         <div class=\"dash\">\
         <div class=\"stat\"><div class=\"v\">{}</div><div class=\"k\">sources</div></div>\
         <div class=\"stat\"><div class=\"v\">{}</div><div class=\"k\">datasets</div></div>\
         <div class=\"stat\"><div class=\"v\">{}</div><div class=\"k\">items</div></div></div>\
         <div class=\"card\"><h3>Personal access tokens</h3>\
         <p>Prefer a static token over OAuth? Create one and use it as your MCP \
         <code>Authorization: Bearer</code> header.</p>\
         <form method=\"post\" action=\"/tokens/create\" class=\"tokrow\">\
         <input type=\"hidden\" name=\"csrf\" value=\"{}\">\
         <input type=\"text\" name=\"name\" placeholder=\"Token name (e.g. claude)\" aria-label=\"Token name\" autocomplete=\"off\">\
         <button class=\"btn\" type=\"submit\">Create</button></form>\
         <div class=\"tablewrap\"><table><tr><th scope=\"col\">Name</th><th scope=\"col\">Prefix</th><th scope=\"col\">Created</th><th scope=\"col\">Last used</th><th scope=\"col\"><span class=\"vh\">Actions</span></th></tr>{token_rows}</table></div></div>\
         <div class=\"card\"><h3>Ingestion pipeline</h3>\
         <p>fetch → parse → normalize → diff → upsert, every 6 hours in this same process. \
         Only changed records are rewritten. Watch the table below for completion.</p>\
         <form method=\"post\" action=\"/api/ingest/run\">\
         <input type=\"hidden\" name=\"csrf\" value=\"{}\">\
         <button class=\"btn\" type=\"submit\">Run ingestion now</button></form>\
         <div class=\"tablewrap\"><table style=\"margin-top:12px\"><tr><th scope=\"col\">Run</th><th scope=\"col\">Started</th><th scope=\"col\">Finished</th><th scope=\"col\">Status</th><th scope=\"col\">Detail</th></tr>{run_rows}</table></div></div>",
        esc(d.username),
        d.stats.sources,
        d.stats.datasets,
        d.stats.items,
        esc(d.csrf),
        esc(d.csrf)
    );
    layout(
        "Dashboard",
        "Your Offsite Data dashboard: MCP tokens, ingestion runs and corpus stats.",
        &format!("{base_url}/dashboard"),
        Some(d.username),
        Some(d.csrf),
        &body,
    )
}
