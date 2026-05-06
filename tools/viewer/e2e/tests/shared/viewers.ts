import type { Page } from '@playwright/test';

const VIEWER_HOST = '127.0.0.1';

export interface ViewerConfig {
  /** Human-readable name used in test titles. */
  name: string;
  /** Base URL of the release binary server. */
  url: string;
  /** CSS selector whose visibility signals that the app has fully rendered. */
  readySelector: string;
  /** Timeout (ms) for waiting on readySelector. */
  readyTimeout: number;
}

export const VIEWERS: ViewerConfig[] = [
  {
    name: 'log-viewer',
    url: `http://${VIEWER_HOST}:3000`,
    readySelector: '.tab-bar',
    readyTimeout: 20_000,
  },
  {
    name: 'doc-viewer',
    url: `http://${VIEWER_HOST}:3001`,
    readySelector: '.app',
    readyTimeout: 20_000,
  },
  {
    name: 'ticket-viewer',
    url: `http://${VIEWER_HOST}:3002`,
    // viewer-api Header component renders <header class="header">.
    readySelector: 'header.header',
    readyTimeout: 60_000,
  },
  {
    name: 'spec-viewer',
    url: `http://${VIEWER_HOST}:4002`,
    readySelector: 'header.header',
    readyTimeout: 60_000,
  },
];

export const LOG_VIEWER = VIEWERS.find((viewer) => viewer.name === 'log-viewer');
export const DOC_VIEWER = VIEWERS.find((viewer) => viewer.name === 'doc-viewer');
export const TICKET_VIEWER = VIEWERS.find((viewer) => viewer.name === 'ticket-viewer');

export const DIOXUS_VIEWERS = VIEWERS.filter(
  (viewer) => viewer.name === 'ticket-viewer' || viewer.name === 'spec-viewer',
);

export const SPEC_VIEWER = VIEWERS.find((viewer) => viewer.name === 'spec-viewer');

if (!SPEC_VIEWER) {
  throw new Error('spec-viewer config is missing from VIEWERS');
}

if (!LOG_VIEWER) {
  throw new Error('log-viewer config is missing from VIEWERS');
}

if (!DOC_VIEWER) {
  throw new Error('doc-viewer config is missing from VIEWERS');
}

if (!TICKET_VIEWER) {
  throw new Error('ticket-viewer config is missing from VIEWERS');
}

/** Navigate to a viewer and wait until its ready selector is visible. */
export async function gotoAndWaitForViewer(page: Page, viewer: ViewerConfig): Promise<void> {
  await page.goto(viewer.url, { waitUntil: 'domcontentloaded' });
  await page.locator(viewer.readySelector).first().waitFor({
    state: 'visible',
    timeout: viewer.readyTimeout,
  });
}
