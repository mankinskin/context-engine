import { defineConfig } from '@playwright/test';

/**
 * Playwright configuration for the centralized viewer E2E suite.
 *
 * Tests run against the **release binaries** — all four viewer servers must
 * already be running on their default ports before executing the suite:
 *
 *   log-viewer    http://localhost:3000
 *   doc-viewer    http://localhost:3001
 *   ticket-viewer http://localhost:3002
 *   spec-viewer   http://localhost:4002
 *
 * Run with (servers must be started first):
 *   cd tools/viewer/e2e
 *   npm install            # first time only
 *   npx playwright install chromium  # first time only
 *   npx playwright test
 *   npx playwright test --headed
 *
 * Or via cargo-make from the workspace root:
 *   cargo make test-e2e
 */
export default defineConfig({
  testDir: './tests',

  // WASM hydration can take up to 60 s; give plenty of headroom.
  timeout: 90_000,
  expect: { timeout: 15_000 },

  // Viewers share static state — run sequentially to avoid race conditions.
  fullyParallel: false,
  workers: 1,

  forbidOnly: !!process.env['CI'],
  retries: process.env['CI'] ? 1 : 0,

  reporter: [
    ['list'],
    ['html', { outputFolder: 'playwright-report', open: 'never' }],
  ],

  use: {
    browserName: 'chromium',
    headless: true,
    viewport: { width: 1280, height: 800 },
    // Capture trace on first retry to help debug CI failures.
    trace: 'on-first-retry',
  },

  // No webServer block — assumes release binaries are already running.
});
