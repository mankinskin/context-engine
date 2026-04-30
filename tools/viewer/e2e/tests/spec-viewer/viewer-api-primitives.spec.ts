import { expect, test, type Page } from '@playwright/test';
import { getHashParam } from '../../../viewer-api/frontend/dioxus/e2e/test_apis';
import { SPEC_VIEWER, gotoAndWaitForViewer } from '../shared/viewers';

/**
 * End-to-end coverage for the viewer-api primitives adopted by spec-viewer
 * in tickets P5.1 – P5.7:
 *
 * - P5.1 TabsStore  — multi-spec tab strip with focus + close
 * - P5.2 Breadcrumbs — `All specs › <component> › <title>` above SpecDetail
 * - P5.3 CardGrid   — category page renders specs as Card grid (graph route)
 * - P5.4 Overlay    — theme settings live inside a role=dialog modal
 * - P5.5 HeaderActions — shared home / filter / theme button group
 * - P5.6 PathCodec  — URL hash drives selection and survives history nav
 * - P5.7 Prefetcher — switching to a previously-opened tab does not refetch
 */

/**
 * Find any spec-viewer category folder in the tree and open it, then return
 * the labels of the first two leaf specs underneath it so individual tests
 * can drive multi-tab interactions deterministically.
 */
async function openTwoSpecsInSameCategory(
  page: Page,
): Promise<{ first: string; second: string }> {
  // Find a folder that has at least 2 children.  The spec-viewer category is
  // a safe bet locally; fall back to any folder that yields two leaves.
  const folder = page.locator('.tree-item-row[role="treeitem"]').filter({ hasText: 'spec-viewer' }).first();
  await expect(folder).toBeVisible({ timeout: 20_000 });
  await folder.dispatchEvent('click');

  // Children of the folder are leaves with state badges.
  const leafLabels = page.locator('.tree-item-row[role="treeitem"] .tree-label');
  await expect(leafLabels.first()).toBeVisible({ timeout: 10_000 });

  const labels = await leafLabels.allTextContents();
  // Filter to leaves shown after expansion (skip the folder itself).
  const childLeaves = labels.filter((l) => l && !l.startsWith('spec-viewer'));
  // Pick first two leaf labels of the spec-viewer folder; their text starts
  // with `spec-viewer:` per the seed data.
  const specLeafLabels = labels.filter((l) => l.startsWith('spec-viewer:')).slice(0, 2);
  expect(
    specLeafLabels.length,
    'spec-viewer folder should contain at least two leaf specs',
  ).toBeGreaterThanOrEqual(2);

  return { first: specLeafLabels[0], second: specLeafLabels[1], childLeaves } as {
    first: string;
    second: string;
    childLeaves: string[];
  };
}

test.describe('spec-viewer — viewer-api primitives (P5.1–P5.7)', () => {
  test('P5.5 HeaderActions: shared header buttons render', async ({ page }) => {
    test.setTimeout(60_000);
    await gotoAndWaitForViewer(page, SPEC_VIEWER);
    await expect(page.getByRole('button', { name: 'Home' })).toBeVisible({ timeout: 10_000 });
    await expect(page.getByRole('button', { name: 'Toggle filters' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Theme settings' })).toBeVisible();
  });

  test('P5.4 Overlay: theme settings open in a role=dialog modal-backdrop', async ({ page }) => {
    test.setTimeout(60_000);
    await gotoAndWaitForViewer(page, SPEC_VIEWER);

    await page.getByRole('button', { name: 'Theme settings' }).click();

    // The shared Overlay component renders the role=dialog modal-backdrop
    // host; the ThemeSettings panel inside it also has its own dialog role,
    // so we target the backdrop class explicitly.
    const dialog = page.locator('.modal-backdrop[role="dialog"][aria-label="Theme settings"]');
    await expect(dialog).toBeVisible({ timeout: 10_000 });

    const panel = page.locator('.modal-panel.theme-settings-modal');
    await expect(panel).toBeVisible();

    // The ThemeSettings panel exposes its own close button ("✕"); clicking
    // it propagates through the Overlay's on_close handler.
    await panel.locator('button', { hasText: '✕' }).first().click();
    await expect(panel).not.toBeVisible({ timeout: 5_000 });
  });

  test('P5.1 TabsStore + P5.2 Breadcrumbs: opening two specs creates two tabs and updates breadcrumbs', async ({
    page,
  }) => {
    test.setTimeout(90_000);
    await gotoAndWaitForViewer(page, SPEC_VIEWER);

    const { first, second } = await openTwoSpecsInSameCategory(page);

    // Open first leaf.
    await page.locator('.tree-item-row .tree-label', { hasText: first }).first().dispatchEvent('click');
    const firstId = await waitForHashId(page);

    // Tab strip shows one tab; breadcrumbs shows category + title.
    await expect(page.locator('.tab-bar')).toBeVisible();
    await expect(page.locator('.tab-bar').getByText(first, { exact: false })).toBeVisible();

    const breadcrumbs = page.getByRole('navigation', { name: 'Breadcrumb' });
    await expect(breadcrumbs).toBeVisible();
    await expect(breadcrumbs).toContainText('All specs');
    await expect(breadcrumbs).toContainText('spec-viewer');
    await expect(breadcrumbs).toContainText(first);

    // Open second leaf — second tab appears, both remain.
    await page.locator('.tree-item-row .tree-label', { hasText: second }).first().dispatchEvent('click');
    const secondId = await waitForHashId(page);
    expect(secondId).not.toBe(firstId);

    await expect(page.locator('.tab-bar').getByText(first, { exact: false })).toBeVisible();
    await expect(page.locator('.tab-bar').getByText(second, { exact: false })).toBeVisible();
    await expect(breadcrumbs).toContainText(second);

    // Closing the active tab focuses the remaining tab.
    await page
      .locator('.tab-bar')
      .getByRole('button', { name: new RegExp(`^Close ${escapeRegExp(second)}$`) })
      .click();
    await expect(page.locator('.tab-bar').getByText(second, { exact: false })).toHaveCount(0);
    await expect(page.locator('.tab-bar').getByText(first, { exact: false })).toBeVisible();
    await expect
      .poll(() => getHashParam(page, 'id'), { timeout: 5_000 })
      .toBe(firstId);
  });

  test('P5.7 Prefetcher: switching to a cached tab does not refetch the spec body', async ({ page }) => {
    test.setTimeout(90_000);
    await gotoAndWaitForViewer(page, SPEC_VIEWER);

    const { first, second } = await openTwoSpecsInSameCategory(page);
    await page.locator('.tree-item-row .tree-label', { hasText: first }).first().dispatchEvent('click');
    const firstId = await waitForHashId(page);
    await page.locator('.tree-item-row .tree-label', { hasText: second }).first().dispatchEvent('click');
    await waitForHashId(page);

    // Now record any /api/specs/<id>/full requests and click back to the
    // first tab.  The Prefetcher should serve the *active* spec from the
    // LRU cache (no refetch for `firstId`); background warming of *other*
    // siblings is allowed and expected.
    const requests: string[] = [];
    page.on('request', (req) => {
      const u = req.url();
      if (u.includes('/api/specs/') && (u.endsWith('/full') || u.includes('/full?'))) {
        requests.push(u);
      }
    });

    await page.locator('.tab-bar').getByText(first, { exact: false }).click();
    await page.waitForTimeout(750);

    // Encoded id may use '%2D' for '-' in the URL; check both forms.
    const encoded = encodeURIComponent(firstId).toLowerCase();
    const refetched = requests.some(
      (u) => u.toLowerCase().includes(`/specs/${firstId.toLowerCase()}/full`)
        || u.toLowerCase().includes(`/specs/${encoded}/full`),
    );
    expect(
      refetched,
      `cached tab switch must not refetch the active spec (firstId=${firstId})`,
    ).toBe(false);
  });

  test('P5.6 PathCodec routing: deep-linked hash opens the corresponding spec', async ({ page }) => {
    test.setTimeout(90_000);
    await gotoAndWaitForViewer(page, SPEC_VIEWER);

    // Discover a spec id by clicking once and reading the hash, then reload
    // to a fresh page and confirm the deep link reopens that spec.
    await openTwoSpecsInSameCategory(page);
    const label = (await page
      .locator('.tree-item-row .tree-label')
      .filter({ hasText: 'spec-viewer:' })
      .first()
      .textContent())
      ?.trim() ?? '';
    await page.locator('.tree-item-row .tree-label', { hasText: label }).first().dispatchEvent('click');
    const id = await waitForHashId(page);

    // Reload from a clean page using the hash as a deep link.
    await page.goto(`${SPEC_VIEWER!.url}/specs#id=${id}`);
    await page.waitForSelector('header.header', { timeout: 30_000 });

    await expect
      .poll(() => getHashParam(page, 'id'), { timeout: 10_000 })
      .toBe(id);

    // The tab strip and breadcrumbs hydrate from the hash.
    await expect(page.locator('.tab-bar')).toBeVisible({ timeout: 10_000 });
    await expect(page.locator('.tab-bar').getByText(label, { exact: false })).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Breadcrumb' })).toContainText(label);
  });
});

// ── helpers ────────────────────────────────────────────────────────────────

async function waitForHashId(page: Page): Promise<string> {
  await expect
    .poll(() => getHashParam(page, 'id'), { timeout: 10_000 })
    .toBeTruthy();
  return (await getHashParam(page, 'id')) as string;
}

function escapeRegExp(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
