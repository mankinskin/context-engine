/**
 * E2E smoke tests for all four viewer tools.
 *
 * Validates that each viewer:
 *  1. Loads successfully (HTTP 200).
 *  2. Renders its UI without JavaScript errors or uncaught exceptions.
 *  3. Has no missing static assets (no 404 for JS/CSS/WASM files).
 *  4. Passes a basic content check (a viewer-specific CSS selector is visible).
 *
 * These tests run against the **release binaries** — all servers must be
 * started before the suite executes:
 *
 *   log-viewer    http://localhost:3000  (Vite/Preact, port 3000)
 *   doc-viewer    http://localhost:3001  (Vite/Preact, port 3001)
 *   ticket-viewer http://localhost:3002  (Dioxus WASM, port 3002)
 *   spec-viewer   http://localhost:4002  (Dioxus WASM, port 4002)
 *
 * Ready-selector strategy
 * ───────────────────────
 * • Vite/Preact viewers (log-viewer, doc-viewer): lightweight — wait for the
 *   app root element rendered by the JS framework.
 * • Dioxus WASM viewers (ticket-viewer, spec-viewer): heavyweight — wait for
 *   `header.header` which is part of viewer-api's Layout component and only
 *   appears after the WASM module has loaded, hydrated, and rendered the
 *   initial route.  A 60 s timeout covers slow WASM initialisation.
 */

import { test, expect } from '@playwright/test';
import {
  getHashParam,
  getSelectedTreeLabels,
  loadAndInspectViewer,
} from '../../viewer-api/frontend/dioxus/e2e/test_apis';

// ── Viewer configurations ─────────────────────────────────────────────────────

interface ViewerConfig {
  /** Human-readable name used in test titles. */
  name: string;
  /** Base URL of the release binary server. */
  url: string;
  /**
   * CSS selector whose visibility signals that the app has fully rendered.
   * Chosen to be stable across theme/data changes.
   */
  readySelector: string;
  /**
   * Timeout (ms) for the readySelector wait.
   * WASM apps need more time than pure JS apps.
   */
  readyTimeout: number;
}

const VIEWERS: ViewerConfig[] = [
  {
    name: 'log-viewer',
    url: 'http://localhost:3000',
    readySelector: '.tab-bar',
    readyTimeout: 20_000,
  },
  {
    name: 'doc-viewer',
    url: 'http://localhost:3001',
    readySelector: '.app',
    readyTimeout: 20_000,
  },
  {
    name: 'ticket-viewer',
    url: 'http://localhost:3002',
    // viewer-api Header component renders <header class="header"> — present
    // in all route states (loading, error, data) after WASM hydration.
    readySelector: 'header.header',
    readyTimeout: 60_000,
  },
  {
    name: 'spec-viewer',
    url: 'http://localhost:4002',
    readySelector: 'header.header',
    readyTimeout: 60_000,
  },
];

// ── Helpers ───────────────────────────────────────────────────────────────────

// ── Tests ─────────────────────────────────────────────────────────────────────

for (const viewer of VIEWERS) {
  test.describe(viewer.name, () => {

    test('renders without console errors or uncaught exceptions', async ({ page }) => {
      // Per-test timeout covers WASM load + settle window.
      test.setTimeout(90_000);

      const { errors } = await loadAndInspectViewer(
        page,
        viewer.url,
        viewer.readySelector,
        viewer.readyTimeout,
      );

      expect(
        errors,
        `${viewer.name} produced JS errors after loading`,
      ).toEqual([]);
    });

    test('no missing static assets (no 404 for JS/CSS/WASM)', async ({ page }) => {
      test.setTimeout(90_000);

      const { missingAssets } = await loadAndInspectViewer(
        page,
        viewer.url,
        viewer.readySelector,
        viewer.readyTimeout,
      );

      expect(
        missingAssets,
        `${viewer.name} has missing static assets`,
      ).toEqual([]);
    });

    test('ready-selector is visible after load', async ({ page }) => {
      test.setTimeout(90_000);

      await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });

      const locator = page.locator(viewer.readySelector).first();
      await expect(locator).toBeVisible({ timeout: viewer.readyTimeout });
    });

  });
}

// ── GPU overlay + ThemeSettings — Dioxus viewers only ─────────────────────────

const DIOXUS_VIEWERS = VIEWERS.filter(v =>
  v.name === 'ticket-viewer' || v.name === 'spec-viewer',
);

for (const viewer of DIOXUS_VIEWERS) {
  test.describe(`${viewer.name} — GPU overlay & theme settings`, () => {

    test('WebGPU canvas element is present in the DOM', async ({ page }) => {
      test.setTimeout(90_000);

      await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });

      // ViewerShell always renders <canvas id="webgpu-canvas">.
      const canvas = page.locator('#webgpu-canvas');
      await expect(canvas).toBeAttached({ timeout: 5_000 });
    });

    if (viewer.name === 'spec-viewer') {
      test('tree selection follows URL hash when navigating browser history', async ({ page }) => {
        // Regression guard:
        // spec-viewer updates #id=... from selection and restores it on back/forward.
        // The highlighted tree row must track that hash-driven selection.
        test.setTimeout(90_000);

        await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
        await page.locator(viewer.readySelector).first().waitFor({
          state: 'visible',
          timeout: viewer.readyTimeout,
        });

        const rows = page.locator('.tree-item-row');
        await expect(rows.first()).toBeVisible({ timeout: 20_000 });

        const firstLabel = (await rows.nth(0).locator('.tree-label').textContent())?.trim() ?? '';
        const secondLabel = (await rows.nth(1).locator('.tree-label').textContent())?.trim() ?? '';

        await rows.nth(0).click();
        const firstId = await getHashParam(page, 'id');
        expect(firstId, 'first selection should set #id').toBeTruthy();

        await rows.nth(1).click();
        const secondId = await getHashParam(page, 'id');
        expect(secondId, 'second selection should set #id').toBeTruthy();
        expect(secondId).not.toBe(firstId);

        await page.goBack();

        await expect
          .poll(() => getHashParam(page, 'id'), {
            timeout: 10_000,
            message: 'URL hash id should return to the first selected spec after browser back',
          })
          .toBe(firstId);

        await expect
          .poll(() => getSelectedTreeLabels(page), {
            timeout: 10_000,
            message: 'selected file-tree row should track hash-driven selection',
          })
          .toEqual([firstLabel]);

        expect(secondLabel).not.toBe(firstLabel);
      });
    }

    test('theme settings panel opens and closes via the palette button', async ({ page }) => {
      test.setTimeout(90_000);

      await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });

      // The 🎨 button in the header right slot.
      // ticket-viewer does a redirect (/→/workspace/default) + SSE connect,
      // so give it up to 30 s to settle before asserting the button.
      const themeBtn = page.locator('button[aria-label="Theme settings"]');
      await expect(themeBtn).toBeVisible({ timeout: 30_000 });

      // Panel is not yet visible.
      const panel = page.locator('.theme-settings');
      await expect(panel).not.toBeVisible();

      // Open the panel.
      await themeBtn.click();
      await expect(panel).toBeVisible({ timeout: 5_000 });

      // The panel should contain the "Theme Settings" heading.
      await expect(panel.locator('.glass-panel__title')).toContainText('Theme Settings');

      // Close via the ✕ button inside the panel.
      await panel.locator('button[aria-label="Close theme settings"]').click();
      await expect(panel).not.toBeVisible({ timeout: 5_000 });
    });

    test('GPU overlay master toggle defaults ON and toggles off/on without errors', async ({ page }) => {
      test.setTimeout(90_000);

      // Track JS errors so we can assert nothing throws while interacting.
      // We deliberately ignore network/resource-load errors (covered by
      // loadAndInspect) — only true JS exceptions and console.error from app
      // code should fail this test.
      const jsErrors: string[] = [];
      page.on('pageerror', (err) => jsErrors.push(`pageerror: ${err.message}`));
      page.on('console', (msg) => {
        if (msg.type() !== 'error') return;
        const text = msg.text();
        if (text.includes('Failed to load resource')) return;
        jsErrors.push(`console.error: ${text}`);
      });

      await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });

      // Wipe persisted toggle so default-ON assertion is meaningful.
      await page.evaluate(() => localStorage.removeItem('viewer-api-gpu-enabled'));
      await page.reload({ waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });

      const themeBtn = page.locator('button[aria-label="Theme settings"]');
      await expect(themeBtn).toBeVisible({ timeout: 30_000 });
      await themeBtn.click();

      const panel = page.locator('.theme-settings');
      await expect(panel).toBeVisible({ timeout: 5_000 });

      // The "Enable GPU overlay" row in the Effects section.
      const label = panel.locator('.theme-settings__effect-label', {
        hasText: 'Enable GPU overlay',
      });
      await expect(label).toBeVisible();

      const checkbox = panel.locator('.theme-settings__toggle-switch input[type="checkbox"]').first();
      // Hidden by CSS (opacity:0) — verify state via DOM property, not visibility.
      // Default for all viewers: ON (the viewer is fully GPU-accelerated by default).
      await expect.poll(() => checkbox.evaluate((el: HTMLInputElement) => el.checked)).toBe(true);

      // Toggle OFF via the visible slider span (the input itself is invisible).
      const slider = panel.locator('.theme-settings__toggle-slider').first();
      await slider.click({ force: true });
      await expect.poll(() => checkbox.evaluate((el: HTMLInputElement) => el.checked)).toBe(false);

      // localStorage reflects the new state.
      await expect
        .poll(() => page.evaluate(() => localStorage.getItem('viewer-api-gpu-enabled')))
        .toBe('false');

      // Toggle back ON.
      await slider.click({ force: true });
      await expect.poll(() => checkbox.evaluate((el: HTMLInputElement) => el.checked)).toBe(true);
      await expect
        .poll(() => page.evaluate(() => localStorage.getItem('viewer-api-gpu-enabled')))
        .toBe('true');

      // No JS errors during the entire interaction.
      expect(jsErrors, `${viewer.name} produced JS errors during toggle interaction`).toEqual([]);
    });

    test('every theme-settings toggle and button can be activated without JS errors', async ({ page }) => {
      // Smoke test for *all* interactive controls in the theme panel.
      // Visual correctness of effects is verified manually / by an agent;
      // this test only catches JS exceptions, console.error, or unhandled
      // promise rejections triggered by the UI interactions themselves.
      test.setTimeout(120_000);

      // Ignore network/resource-load errors (covered by loadAndInspect) — only
      // app-code JS exceptions and console.error should fail this test.
      const jsErrors: string[] = [];
      page.on('pageerror', (err) => jsErrors.push(`pageerror: ${err.message}`));
      page.on('console', (msg) => {
        if (msg.type() !== 'error') return;
        const text = msg.text();
        if (text.includes('Failed to load resource')) return;
        jsErrors.push(`console.error: ${text}`);
      });

      await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });

      const themeBtn = page.locator('button[aria-label="Theme settings"]');
      await expect(themeBtn).toBeVisible({ timeout: 30_000 });
      await themeBtn.click();

      const panel = page.locator('.theme-settings');
      await expect(panel).toBeVisible({ timeout: 5_000 });

      // ── Exercise every iOS-style toggle (master GPU + any per-effect toggles).
      // Toggle each one OFF then back ON via its visible slider span.
      const sliders = panel.locator('.theme-settings__toggle-slider');
      const sliderCount = await sliders.count();
      for (let i = 0; i < sliderCount; i++) {
        const s = sliders.nth(i);
        await s.click({ force: true });
        await page.waitForTimeout(50);
        await s.click({ force: true });
        await page.waitForTimeout(50);
      }

      // ── Exercise every preset card (if present).
      const presetCards = panel.locator('.theme-preset-card, .theme-settings__preset-button');
      const presetCount = await presetCards.count();
      for (let i = 0; i < presetCount; i++) {
        await presetCards.nth(i).click();
        await page.waitForTimeout(50);
      }

      // ── Exercise every range slider (if present): set to min, then to max.
      const ranges = panel.locator('input[type="range"]');
      const rangeCount = await ranges.count();
      for (let i = 0; i < rangeCount; i++) {
        const r = ranges.nth(i);
        const max = await r.evaluate((el: HTMLInputElement) => el.max);
        const min = await r.evaluate((el: HTMLInputElement) => el.min);
        await r.evaluate((el: HTMLInputElement, v: string) => {
          el.value = v;
          el.dispatchEvent(new Event('input', { bubbles: true }));
          el.dispatchEvent(new Event('change', { bubbles: true }));
        }, min);
        await page.waitForTimeout(20);
        await r.evaluate((el: HTMLInputElement, v: string) => {
          el.value = v;
          el.dispatchEvent(new Event('input', { bubbles: true }));
          el.dispatchEvent(new Event('change', { bubbles: true }));
        }, max);
        await page.waitForTimeout(20);
      }

      // ── Exercise color pickers: set to a known value via DOM API.
      const colorPickers = panel.locator('input[type="color"]');
      const colorCount = await colorPickers.count();
      for (let i = 0; i < colorCount; i++) {
        const c = colorPickers.nth(i);
        await c.evaluate((el: HTMLInputElement) => {
          el.value = '#abcdef';
          el.dispatchEvent(new Event('input', { bubbles: true }));
          el.dispatchEvent(new Event('change', { bubbles: true }));
        });
        await page.waitForTimeout(20);
      }

      // ── Close the panel.
      const closeBtn = panel.locator('button[aria-label="Close theme settings"]');
      if (await closeBtn.count() > 0) {
        await closeBtn.click();
        await expect(panel).not.toBeVisible({ timeout: 5_000 });
      }

      // No JS errors during the entire interaction.
      expect(
        jsErrors,
        `${viewer.name} produced JS errors while exercising theme controls`,
      ).toEqual([]);
    });

  });
}
