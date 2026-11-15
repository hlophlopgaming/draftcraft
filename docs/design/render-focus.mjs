// Renders docs/design/focus.html to PNGs: node docs/design/render-focus.mjs
// Uses the Playwright + Chromium that the dev container provides.
import { createRequire } from 'node:module';
const { chromium } = createRequire(import.meta.url)('playwright');
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';

const dir = path.dirname(fileURLToPath(import.meta.url));
const url = pathToFileURL(path.join(dir, 'focus.html')).href;
const screens = ['write', 'typewriter', 'navigator', 'cards', 'stats', 'panels', 'av', 'rtl', 'welcome'];
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SCALE || 1) });
for (const s of screens) {
  for (const theme of ['light', 'dark']) {
    await page.goto(`${url}?s=${s}&theme=${theme}`);
    await page.evaluate(() => document.fonts.ready);
    await page.screenshot({ path: path.join(dir, 'focus', `${s}-${theme}.png`) });
  }
}
await browser.close();
