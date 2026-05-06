import { defineConfig } from '@playwright/test';
import path from 'node:path';

/**
 * Playwright configuration for the centralized viewer E2E suite.
 *
 * Tests run against the **release binaries**. Playwright starts the viewer
 * servers automatically through `viewer-ctl` before executing the suite:
 *
 *   log-viewer    http://127.0.0.1:3000
 *   doc-viewer    http://127.0.0.1:3001
 *   ticket-viewer http://127.0.0.1:3002
 *   spec-viewer   http://127.0.0.1:4002
 *
 * Run with:
 *   cd tools/viewer/e2e
 *   npm install            # first time only
 *   npx playwright install chromium  # first time only
 *   npx playwright test
 *   npx playwright test --headed
 *
 * Or via cargo-make from the workspace root:
 *   cargo make test-e2e
 */
const repoRoot = path.resolve(__dirname, '../../..');

const managedViewers = [
  { name: 'log-viewer', url: 'http://127.0.0.1:3000' },
  { name: 'doc-viewer', url: 'http://127.0.0.1:3001' },
  { name: 'ticket-viewer', url: 'http://127.0.0.1:3002' },
  { name: 'spec-viewer', url: 'http://127.0.0.1:4002' },
];

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

  webServer: managedViewers.map(({ name, url }) => ({
    command: `viewer-ctl prepare ${name} && viewer-ctl start ${name} --foreground`,
    url,
    cwd: repoRoot,
    reuseExistingServer: !process.env['CI'],
    timeout: 300_000,
  })),
});
