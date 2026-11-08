// Detaildialog + Zertifikats-PDF des amtlichen Fachbetrieberegisters (Vaadin).
// Nur lesende öffentliche Abfragen: keine Anmeldung, keine Schreiboperation.
//
// Aufruf: node tools/fbr_detail_pdf.js <query.json> <outdir>
const {chromium} = require('/tmp/opencode/pw/node_modules/playwright-core');
const fs = require('fs');
const path = require('path');

const BASE = 'https://fachbetrieberegister.gadsys.de/fachbetrieberegister';
const CHROME = process.env.CHROMIUM_PATH
  || '/root/.cache/ms-playwright/chromium-1254/chrome-linux64/chrome';

function buildUrl(q) {
  const p = [];
  if (q.adr) {
    const r = q.dst || 10;
    const dlat = r / 111.32;
    const dlon = r / (111.32 * Math.cos((q.lat * Math.PI) / 180));
    const adr = `${q.adr}, Deutschland;${q.lon};${q.lat};`
      + `${(q.lon + dlon).toFixed(7)};${(q.lat + dlat).toFixed(7)};`
      + `${(q.lon - dlon).toFixed(7)};${(q.lat - dlat).toFixed(7)}`;
    p.push(`adr=${encodeURIComponent(adr)}`);
  }
  p.push(`dst=${q.dst || 10}`);
  p.push('is=1');
  p.push('e=1');
  if (q.name) p.push(`n=${encodeURIComponent(q.name)}`);
  if (q.p) p.push(`p=${encodeURIComponent(q.p)}`);
  if (q.fd) p.push(`fd=${q.fd}`);
  return `${BASE}/${q.route}/${p.join('&')}`;
}

async function readRows(page) {
  return page.evaluate(async () => {
    const g = document.getElementById('Ergebnisliste');
    if (!g) return [];
    g.style.height = '40000px';
    g.style.minHeight = '40000px';
    await new Promise((r) => setTimeout(r, 2500));
    const sr = g.shadowRoot;
    const items = sr && sr.getElementById('items');
    if (!items) return [];
    const out = [];
    for (const r of items.querySelectorAll('tr')) {
      const tds = Array.from(r.querySelectorAll('td'));
      out.push(tds.map((td) => {
        const slot = td.querySelector('slot');
        const el = slot && slot.assignedElements ? slot.assignedElements() : [];
        return el.map((e) => (e.innerText || e.textContent || '').trim().replace(/\s+/g, ' ')).join(' ');
      }));
    }
    return out;
  });
}

async function readDialog(page) {
  return page.evaluate(() => {
    const ov = document.querySelector('vaadin-dialog-overlay');
    let text = '';
    if (ov) {
      const host = ov.shadowRoot ? ov.shadowRoot.querySelector('vaadin-dialog') : ov;
      text = host ? (host.innerText || host.textContent || '') : (ov.innerText || ov.textContent || '');
    }
    const links = [];
    const scan = (root) => {
      for (const e of root.querySelectorAll('a')) {
        const h = typeof e.href === 'string' ? e.href : e.getAttribute('href');
        if (h) links.push(h);
      }
      for (const e of root.querySelectorAll('*')) if (e.shadowRoot) scan(e.shadowRoot);
    };
    scan(document);
    return {text: text.replace(/\n{3,}/g, '\n\n').trim(), links};
  });
}

async function clickRowThenDetails(page, idx) {
  const box = await page.evaluate(async (i) => {
    const g = document.getElementById('Ergebnisliste');
    g.style.height = '40000px';
    g.style.minHeight = '40000px';
    await new Promise((r) => setTimeout(r, 2500));
    const sr = g.shadowRoot;
    const items = sr && sr.getElementById('items');
    const tr = items && items.querySelectorAll('tr')[i];
    if (!tr) return null;
    const td = tr.querySelectorAll('td')[0];
    const slot = td.querySelector('slot');
    const el = slot && slot.assignedElements ? slot.assignedElements()[0] : null;
    if (!el) return null;
    const r = el.getBoundingClientRect();
    return {x: r.x + 30, y: r.y + r.height / 2};
  }, idx);
  if (!box || !(box.x > 0)) return false;
  await page.mouse.click(box.x, box.y);
  await page.waitForTimeout(3000);
  const d = await page.evaluate(() => {
    const btns = Array.from(document.querySelectorAll('vaadin-button'))
      .filter((b) => b.textContent.trim() === 'Details');
    if (!btns.length) return null;
    const r = btns[0].getBoundingClientRect();
    return {x: r.x + r.width / 2, y: r.y + r.height / 2};
  });
  if (!d) return false;
  await page.mouse.click(d.x, d.y);
  await page.waitForTimeout(7000);
  return true;
}

async function main() {
  const q = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
  const outdir = process.argv[3] || '/tmp/opencode/fbrpdf';
  fs.mkdirSync(outdir, {recursive: true});
  const browser = await chromium.launch({
    headless: true, executablePath: CHROME,
    args: ['--no-sandbox', '--disable-dev-shm-usage'],
  });
  const ctx = await browser.newContext({viewport: {width: 1800, height: 1400}, acceptDownloads: true});
  const page = await ctx.newPage();
  const url = buildUrl(q);
  await page.goto(url, {waitUntil: 'domcontentloaded', timeout: 120000});
  let hits = null;
  for (let i = 0; i < 45; i++) {
    await page.waitForTimeout(1500);
    const txt = await page.evaluate(() => document.body.innerText);
    const m = txt.match(/([\d.]+)\s+Treffer gefunden/);
    if (m) { hits = parseInt(m[1].replace(/\./g, ''), 10); break; }
  }
  const rows = await readRows(page);
  let idx = -1;
  if (q.match) idx = rows.findIndex((r) => r.join(' | ').toLowerCase().includes(q.match.toLowerCase()));
  else if (q.rowIndex !== undefined) idx = q.rowIndex;

  let detail = null;
  let links = [];
  if (q.p && q.fd) {
    await page.waitForTimeout(9000);
    const d = await readDialog(page);
    detail = d.text; links = d.links;
  }
  if ((!detail || !detail.trim()) && idx >= 0) {
    await clickRowThenDetails(page, idx);
    const d = await readDialog(page);
    detail = d.text; links = d.links;
  }
  // PDF-Permalinks im Session-Kontext herunterladen
  const saved = [];
  for (const l of links) {
    if (!/\.pdf($|\?)/i.test(l)) continue;
    const resp = await ctx.request.get(l, {timeout: 120000});
    const buf = await resp.body();
    const name = path.basename(new URL(l).pathname) || 'cert.pdf';
    fs.writeFileSync(path.join(outdir, name), buf);
    saved.push({name, status: resp.status(), size: buf.length, url: l});
  }
  const out = {url, hits, rowIndex: idx, rows, detail, links, saved};
  fs.writeFileSync(path.join(outdir, 'detail.json'), JSON.stringify(out, null, 2));
  console.log(JSON.stringify({hits, rowIndex: idx, detailLen: detail ? detail.length : 0, saved}));
  await browser.close();
}
main().catch((e) => { console.error('ERR', e.message); process.exit(1); });
