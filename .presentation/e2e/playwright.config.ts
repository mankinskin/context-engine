import { defineConfig, devices } from '@playwright/test';

// Smoke-checks both decks proving the composition mechanism: the
// context-engine deck (port 3030) imports workflow-tools's slides.md
// (port 3031) via Slidev's `src:` include.
export default defineConfig({
  testDir: './tests',
  webServer: [
    {
      command: 'npm run dev -- --port 3030',
      cwd: '..',
      url: 'http://localhost:3030',
      reuseExistingServer: !process.env.CI,
      timeout: 60_000,
    },
    {
      command: 'npm run dev -- --port 3031',
      cwd: '../../workflow-tools/.presentation',
      url: 'http://localhost:3031',
      reuseExistingServer: !process.env.CI,
      timeout: 60_000,
    },
  ],
  use: {
    ...devices['Desktop Chrome'],
  },
});
