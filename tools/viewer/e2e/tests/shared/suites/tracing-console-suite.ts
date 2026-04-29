import { test, expect } from '@playwright/test';
import type { ViewerConfig } from '../viewers';

/**
 * Suite: WASM structured tracing — console layer and network sink behaviour.
 *
 * Spec refs:
 *   - viewer-api/tracing       (b06c9df8)  AC #2, #3, #4, #5
 *   - viewer-api/tracing/file-sink (479e226a)  AC #3
 *
 * Prerequisites: the viewer server for the given config must be running and
 * the WASM frontend must be pre-built (trunk / dist present).
 */
export function registerTracingConsoleSuite(viewer: ViewerConfig): void {
  test.describe(`${viewer.name} — WASM tracing`, () => {
    // ── AC #4: subscriber installed; startup record reaches browser console ──

    test('startup: tracing subscriber emits first record to browser console', async ({ page }) => {
      test.setTimeout(90_000);

      const consoleLines: string[] = [];
      page.on('console', (msg) => consoleLines.push(msg.text()));

      await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });
      await page.waitForTimeout(2_000);

      // tracing-wasm emits the message text as part of the formatted console
      // line. install() emits info!(target:"viewer_api::tracing",
      // "tracing subscriber installed") as the very first record.
      expect(
        consoleLines.some((t) => t.includes('tracing subscriber installed')),
        `${viewer.name}: expected "tracing subscriber installed" record in console.\n` +
          `Captured ${consoleLines.length} lines:\n${consoleLines.slice(0, 20).join('\n')}`,
      ).toBe(true);
    });

    // ── AC #3 (tracing spec): console-only by default; no network traffic ──

    test('default: no POST to /api/client-log without log_sink flag', async ({ page }) => {
      test.setTimeout(30_000);

      const clientLogUrls: string[] = [];
      page.on('request', (req) => {
        if (req.url().includes('/api/client-log')) clientLogUrls.push(req.url());
      });

      await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });
      // Wait well beyond the 2-second flush interval to confirm no traffic.
      await page.waitForTimeout(5_000);

      expect(
        clientLogUrls,
        `${viewer.name}: unexpected POST to /api/client-log without log_sink flag`,
      ).toEqual([]);
    });

    // ── AC #3 (file-sink spec): network layer activates via URL flag ─────────

    test('?log_sink=on: network layer posts batched records to /api/client-log', async ({
      page,
    }) => {
      test.setTimeout(30_000);

      const postedBodies: unknown[] = [];
      page.on('request', (req) => {
        if (!req.url().includes('/api/client-log')) return;
        const body = req.postDataJSON();
        if (body) postedBodies.push(body);
      });

      await page.goto(`${viewer.url}?log_sink=on`, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });
      // Wait for at least one flush cycle (2 s) with comfortable margin.
      await page.waitForTimeout(5_000);

      expect(
        postedBodies.length,
        `${viewer.name}: no POSTs to /api/client-log with ?log_sink=on`,
      ).toBeGreaterThan(0);

      const first = postedBodies[0] as { records: unknown[] };
      expect(first, 'payload missing "records" field').toHaveProperty('records');
      expect(Array.isArray(first.records), '"records" is not an array').toBe(true);
      expect(first.records.length, '"records" array is empty').toBeGreaterThan(0);
    });

    // ── AC #3 (file-sink spec): network layer activates via localStorage ─────

    test('localStorage opt-in: network layer activates when viewer-api-log-sink=on', async ({
      page,
    }) => {
      test.setTimeout(30_000);

      // Inject the opt-in flag before any page scripts run so that install()
      // reads it fresh on the first call.
      await page.addInitScript(() => {
        localStorage.setItem('viewer-api-log-sink', 'on');
      });

      const clientLogUrls: string[] = [];
      page.on('request', (req) => {
        if (req.url().includes('/api/client-log')) clientLogUrls.push(req.url());
      });

      await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });
      await page.waitForTimeout(5_000);

      expect(
        clientLogUrls.length,
        `${viewer.name}: no POSTs to /api/client-log with localStorage opt-in`,
      ).toBeGreaterThan(0);
    });

    // ── AC #5 (tracing spec): filter blocks all events when level=off ────────
    //
    // When ?log=off, the EnvFilter drops every event before it reaches any
    // layer — neither the console layer nor the network layer see records.
    // Even with ?log_sink=on, no POSTs should be made.

    test('?log=off: filter blocks all events from reaching the network layer', async ({
      page,
    }) => {
      test.setTimeout(30_000);

      const clientLogUrls: string[] = [];
      page.on('request', (req) => {
        if (req.url().includes('/api/client-log')) clientLogUrls.push(req.url());
      });

      // Both opt-ins active; level "off" should still block everything.
      await page.goto(`${viewer.url}?log=off&log_sink=on`, {
        waitUntil: 'domcontentloaded',
      });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });
      await page.waitForTimeout(5_000);

      expect(
        clientLogUrls,
        `${viewer.name}: records reached /api/client-log despite ?log=off`,
      ).toEqual([]);
    });

    // ── AC #5 (tracing spec): filter resolved from localStorage ──────────────

    test('localStorage filter: viewer-api-log-filter overrides default level', async ({
      page,
    }) => {
      test.setTimeout(30_000);

      // Set the log filter to "off" via localStorage to suppress all events.
      await page.addInitScript(() => {
        localStorage.setItem('viewer-api-log-filter', 'off');
      });

      const clientLogUrls: string[] = [];
      page.on('request', (req) => {
        if (req.url().includes('/api/client-log')) clientLogUrls.push(req.url());
      });

      await page.goto(`${viewer.url}?log_sink=on`, { waitUntil: 'domcontentloaded' });
      await page.locator(viewer.readySelector).first().waitFor({
        state: 'visible',
        timeout: viewer.readyTimeout,
      });
      await page.waitForTimeout(5_000);

      expect(
        clientLogUrls,
        `${viewer.name}: records reached /api/client-log despite localStorage filter=off`,
      ).toEqual([]);
    });
  });
}
