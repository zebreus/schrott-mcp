// Firmeneintrag im amtlichen Handelsregisterportal (online-handelsregister.de).
// Nur lesende oeffentliche Suche, keine Anmeldung, kein Download.
//
// Aufruf: node tools/hr_detail.js <Suchbegriff> [<Suchbegriff> ...]
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
    try {
      await page.goto(url, {waitUntil: 'domcontentloaded', timeout: 120000});
      await page.waitForTimeout(6000);
      const txt = await page.evaluate(() => document.body.innerText);
      const start = txt.indexOf('Ihr Suchergebnis');
      const end = txt.indexOf('Amtliche Handelsregister-Dokumente');
      const block = start >= 0 ? txt.slice(start, end > start ? end : start + 1500) : txt.slice(0, 800);
      console.log(`=== QUERY: ${q}`);
      console.log(block.trim());
      console.log('');
    } catch (e) {
      console.log(`=== QUERY: ${q}\nFEHLER ${e.message}\n`);
    }
  }
  await browser.close();
}
main().catch((e) => { console.error('ERR', e.message); process.exit(1); });
