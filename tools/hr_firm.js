// Handelsregister-Firmeneintrag: Suchtreffer anklicken und die Detailseite lesen.
// Nur lesende oeffentliche Abfrage, keine Anmeldung.
//
// Aufruf: node tools/hr_firm.js <Suchbegriff> [<Suchbegriff> ...]
const {chromium} = require('/tmp/opencode/pw/node_modules/playwright-core');
const CHROME = process.env.CHROMIUM_PATH
  || '/root/.cache/ms-playwright/chromium-1254/chrome-linux64/chrome';

async function main() {
  const queries = process.argv.slice(2);
  const browser = await chromium.launch({
    headless: true, executablePath: CHROME, args: ['--no-sandbox', '--disable-dev-shm-usage'],
  });
  const page = await browser.newPage({viewport: {width: 1500, height: 1200}});
  for (const q of queries) {
    const url = `https://www.online-handelsregister.de/suche/${encodeURIComponent(q)}`;
    console.log(`=== QUERY: ${q}`);
    try {
      await page.goto(url, {waitUntil: 'domcontentloaded', timeout: 120000});
      await page.waitForTimeout(7000);
      // Firmeneintrag-Link suchen und oeffnen
      const links = await page.evaluate(() => {
        const out = [];
        for (const a of document.querySelectorAll('a')) {
          const h = a.getAttribute('href') || '';
          if (/\/(register|unternehmen|anzeige)/.test(h) && a.innerText.trim()) {
            out.push({href: a.href, text: a.innerText.trim().replace(/\s+/g, ' ')});
          }
        }
        return out;
      });
      console.log('  LINKS:', JSON.stringify(links.slice(0, 6)));
      if (links.length) {
        await page.goto(links[0].href, {waitUntil: 'domcontentloaded', timeout: 120000});
        await page.waitForTimeout(7000);
        const txt = await page.evaluate(() => document.body.innerText);
        const start = txt.indexOf('Firmeneintrag');
        const block = start >= 0 ? txt.slice(start, start + 3500) : txt.slice(0, 2500);
        console.log(block.trim());
      } else {
        const txt = await page.evaluate(() => document.body.innerText);
        const s = txt.indexOf('Ihr Suchergebnis');
        console.log((s >= 0 ? txt.slice(s, s + 800) : txt.slice(0, 800)).trim());
      }
      console.log('');
    } catch (e) {
      console.log(`  FEHLER ${e.message}\n`);
    }
  }
  await browser.close();
}
main().catch((e) => { console.error('ERR', e.message); process.exit(1); });
