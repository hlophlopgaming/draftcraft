// Renders docs/design/redesign-mockups.html to PNGs: node docs/design/render.mjs
// Uses the Playwright + Chromium that the dev container provides.
import { createRequire } from 'node:module';
const { chromium } = createRequire(import.meta.url)('playwright');
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';

const dir = path.dirname(fileURLToPath(import.meta.url));
const url = pathToFileURL(path.join(dir, 'redesign-mockups.html')).href;
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SCALE || 1) });
for (const v of ['a', 'b', 'c']) {
  for (const theme of ['light', 'dark']) {
    await page.goto(`${url}?v=${v}&theme=${theme}`);
    await page.evaluate(() => document.fonts.ready);
    await page.screenshot({ path: path.join(dir, `mockup-${v}-${theme}.png`) });
  }
}
await browser.close();
