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

import { test, expect, type Page } from '@playwright/test';

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

/** Static asset extensions — 404s for these indicate a broken build. */
const STATIC_EXTENSIONS = /\.(js|ts|css|wasm|png|svg|ico|woff2?)(\?.*)?$/i;

interface LoadResult {
  /** console.error() messages and uncaught page exceptions. */
  errors: string[];
  /** URLs of responses with HTTP 404 that match static asset patterns. */
  missingAssets: string[];
}

/**
 * Navigate to `config.url`, wait for the app to render, then return any
 * collected errors and missing-asset URLs.
 *
 * Error collection starts *before* navigation so that errors fired during
 * the initial page load are not missed.
 */
async function loadAndInspect(page: Page, config: ViewerConfig): Promise<LoadResult> {
  const errors: string[] = [];
  const missingAssets: string[] = [];

  // Collect uncaught JS exceptions (e.g. WASM panic).
  page.on('pageerror', (err) => {
    errors.push(`pageerror: ${err.message}`);
  });

  // Collect console.error() calls (includes fetch 404 messages from WASM SSE).
  page.on('console', (msg) => {
    if (msg.type() === 'error') {
      errors.push(`console.error: ${msg.text()}`);
    }
  });

  // Collect HTTP 404s for static asset URLs (JS, CSS, WASM, fonts, images).
  // API 404s (e.g. empty list endpoints) are intentionally excluded.
  page.on('response', (response) => {
    if (response.status() === 404) {
      const url = response.url();
      if (STATIC_EXTENSIONS.test(url)) {
        missingAssets.push(url);
      }
    }
  });

  await page.goto(config.url, { waitUntil: 'domcontentloaded' });

  // Wait for the viewer-specific ready signal.
  await page.locator(config.readySelector).first().waitFor({
    state: 'visible',
    timeout: config.readyTimeout,
  });

  // Brief settle window: lets SSE reconnect logic and background fetches
  // complete so their errors (if any) are captured before assertions run.
  await page.waitForTimeout(2_000);

  return { errors, missingAssets };
}

// ── Tests ─────────────────────────────────────────────────────────────────────

for (const viewer of VIEWERS) {
  test.describe(viewer.name, () => {

    test('renders without console errors or uncaught exceptions', async ({ page }) => {
      // Per-test timeout covers WASM load + settle window.
      test.setTimeout(90_000);

      const { errors } = await loadAndInspect(page, viewer);

      expect(
        errors,
        `${viewer.name} produced JS errors after loading`,
      ).toEqual([]);
    });

    test('no missing static assets (no 404 for JS/CSS/WASM)', async ({ page }) => {
      test.setTimeout(90_000);

      const { missingAssets } = await loadAndInspect(page, viewer);

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
