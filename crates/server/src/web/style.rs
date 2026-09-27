//! Single inline stylesheet for all server-rendered pages.

pub(super) const CSS: &str = "
:root{color-scheme:dark;--bg:#0b0e17;--bg2:#111627;--card:#161d33;--line:#263054;
--text:#eef1ff;--muted:#9aa5c7;--acc:#7c6cff;--acc2:#3ddc97;--danger:#ff6b81}
*{box-sizing:border-box}body{margin:0;font-family:ui-sans-serif,system-ui,-apple-system,
'Segoe UI',Roboto,Helvetica,Arial,sans-serif;background:radial-gradient(1200px 600px at 80% -10%,
#2a2560 0%,transparent 60%),radial-gradient(900px 500px at 10% 0%,#123f38 0%,transparent 55%),
var(--bg);color:var(--text);min-height:100vh}
.wrap{max-width:1020px;margin:0 auto;padding:0 24px}
nav{border-bottom:1px solid var(--line);backdrop-filter:blur(8px)}
nav .wrap{display:flex;align-items:center;gap:20px;height:64px}
.brand{font-weight:800;font-size:19px;letter-spacing:.3px;text-decoration:none;color:var(--text)}
.brand span{color:var(--acc2)}
nav .sp{flex:1}nav a.l{color:var(--muted);text-decoration:none;font-size:14px;margin-left:16px}
nav a.l:hover{color:var(--text)}
.btn{display:inline-block;background:linear-gradient(135deg,var(--acc),#4f8cff);color:#fff;
border:0;border-radius:12px;padding:12px 22px;font-weight:700;font-size:15px;cursor:pointer;
text-decoration:none;box-shadow:0 8px 30px rgba(124,108,255,.35)}
.btn:hover{filter:brightness(1.1)}.btn.ghost{background:transparent;border:1px solid var(--line);
box-shadow:none;color:var(--text)}
.hero{padding:84px 0 40px;text-align:center}
.hero h1{font-size:52px;line-height:1.05;margin:0 0 16px;letter-spacing:-1px}
.hero h1 em{font-style:normal;background:linear-gradient(90deg,var(--acc2),#6ec6ff);
-webkit-background-clip:text;background-clip:text;color:transparent}
.hero p{color:var(--muted);font-size:18px;max-width:640px;margin:0 auto 28px}
.connect{margin:26px auto 0;max-width:640px;background:#0a0f1f;border:1px solid var(--line);
border-radius:14px;padding:14px 16px;font-family:ui-monospace,Menlo,Consolas,monospace;
font-size:14px;display:flex;gap:10px;align-items:center;justify-content:space-between;flex-wrap:wrap}
.connect code{color:var(--acc2);overflow:hidden;text-overflow:ellipsis;overflow-wrap:anywhere}
.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(240px,1fr));gap:16px;margin:44px 0}
.card{background:linear-gradient(180deg,var(--card),#121830);border:1px solid var(--line);
border-radius:16px;padding:22px;margin-bottom:16px}
.card h2{margin:0 0 8px;font-size:17px}.card h3{margin:0 0 8px;font-size:17px}.card p{margin:0 0 8px;color:var(--muted);font-size:14px;line-height:1.55}
.card p:last-child{margin-bottom:0}
.steps{margin:10px 0 60px}.step{display:flex;gap:14px;margin:14px 0;align-items:flex-start}
.n{flex:0 0 30px;height:30px;border-radius:50%;background:var(--acc);display:flex;align-items:center;
justify-content:center;font-weight:800}
.panel{max-width:460px;margin:60px auto;background:var(--card);border:1px solid var(--line);
border-radius:18px;padding:30px}
.panel h2{margin:0 0 6px}.panel p.sub{color:var(--muted);font-size:14px;margin:0 0 20px}
.hint{color:var(--muted);font-size:12px;margin:6px 0 0}
label{display:block;font-size:13px;color:var(--muted);margin:14px 0 6px}
input[type=text],input[type=password]{width:100%;padding:12px 14px;border-radius:10px;border:1px solid
var(--line);background:#0a0f1f;color:var(--text);font-size:15px}
input:focus{outline:2px solid var(--acc);border-color:transparent}
a:focus-visible,button:focus-visible{outline:2px solid var(--acc2);outline-offset:3px;border-radius:6px}
.vh{position:absolute;width:1px;height:1px;margin:-1px;padding:0;overflow:hidden;clip:rect(0 0 0 0);white-space:nowrap;border:0}
.skip{position:absolute;left:16px;top:-48px;z-index:10;background:var(--acc2);color:#06130d;
font-weight:700;padding:10px 16px;border-radius:10px;text-decoration:none;transition:top .15s}
.skip:focus{top:12px}
.check{display:flex;gap:10px;align-items:flex-start;margin:16px 0;font-size:14px;color:var(--muted)}
.check input{margin-top:3px}.check b{color:var(--text)}
.err{background:rgba(255,107,129,.12);border:1px solid var(--danger);color:#ffc9d2;border-radius:10px;
padding:10px 14px;font-size:14px;margin-bottom:10px}
.ok{background:rgba(61,220,151,.12);border:1px solid var(--acc2);color:#c9f5e2;border-radius:10px;
padding:10px 14px;font-size:14px;margin-bottom:10px;word-break:break-all}
.tablewrap{overflow-x:auto}
table{width:100%;border-collapse:collapse;font-size:14px;min-width:560px}
th,td{text-align:left;padding:10px 8px;border-bottom:1px solid var(--line)}
th{color:var(--muted);font-weight:600;font-size:12px;text-transform:uppercase;letter-spacing:.5px}
.dash{display:grid;grid-template-columns:repeat(auto-fit,minmax(150px,1fr));gap:14px;margin:24px 0}
.stat{background:var(--card);border:1px solid var(--line);border-radius:14px;padding:16px}
.stat .v{font-size:30px;font-weight:800}.stat .k{color:var(--muted);font-size:13px}
.rowb{display:flex;gap:10px;margin-top:18px;flex-wrap:wrap}
.tokrow{display:flex;gap:8px;margin:12px 0;flex-wrap:wrap}
.tokrow input{flex:1;min-width:200px}
ol.howto{color:var(--muted);font-size:14px;line-height:1.7;margin:8px 0 0;padding-left:22px}
ol.howto b{color:var(--text)}
footer{border-top:1px solid var(--line);margin-top:60px;padding:26px 0;color:var(--muted);font-size:13px}
footer .wrap{display:flex;gap:12px;align-items:center}
@media(max-width:640px){.hero h1{font-size:36px}.connect{flex-direction:column;align-items:stretch}}
";
