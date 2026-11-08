// USt-IdNr-Abfrage im amtlichen BZOL-Portal (Bundeszentralamt für Steuern).
// Nur lesende öffentliche Abfrage.
//
// Aufruf: node tools/ustidnr.js <USt-IdNr> [<USt-IdNr> ...]
const {chromium} = require('/tmp/opencode/pw/node_modules/playwright-core');
const fs = require('fs');

const CHROME = process.env.CHROMIUM_PATH
  || '/root/.cache/ms-playwright/chromium-1254/chrome-linux64/chrome';

async function main() {
  const ids = process.argv.slice(2);
  const browser = await chromium.launch({
    headless: true, executablePath: CHROME,
    args: ['--no-sandbox', '--disable-dev-shm-usage'],
  });
  const page = await browser.newPage({viewport: {width: 1400, height: 1000}});
  for (const id of ids) {
    const url = `https://www.bzol.de/search/ustidnr/?query=${encodeURIComponent(id)}`;
    try {
      await page.goto(url, {waitUntil: 'domcontentloaded', timeout: 90000});
      await page.waitForTimeout(7000);
      const txt = await page.evaluate(() => document.body.innerText);
      console.log(`=== USt-IdNr ${id}`);
      console.log(txt.replace(/\n{3,}/g, '\n\n').trim().slice(0, 1800));
      console.log('');
    } catch (e) {
      console.log(`=== USt-IdNr ${id}\nFEHLER ${e.message}\n`);
    }
  }
  await browser.close();
}
main().catch((e) => { console.error('ERR', e.message); process.exit(1); });
