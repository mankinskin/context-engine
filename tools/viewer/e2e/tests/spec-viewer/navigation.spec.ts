import { test, expect } from '@playwright/test';
import {
  getHashParam,
  getSelectedTreeLabels,
} from '../../../viewer-api/frontend/dioxus/e2e/test_apis';
import { SPEC_VIEWER, gotoAndWaitForViewer } from '../shared/viewers';

/**
 * Spec-viewer-specific regression checks.
 */
test.describe('spec-viewer — navigation and selection', () => {
  test('clicking a spec in the tree opens detail content in the main view', async ({ page }) => {
    test.setTimeout(90_000);

    await gotoAndWaitForViewer(page, SPEC_VIEWER);

    const placeholder = page.getByText('Select a specification to view details.');
    await expect(placeholder).toBeVisible({ timeout: 20_000 });

    const firstTreeLabel = page.locator('.tree-item-row .tree-label').first();
    await expect(firstTreeLabel).toBeVisible({ timeout: 20_000 });
    await firstTreeLabel.click();

    await expect
      .poll(() => getHashParam(page, 'id'), {
        timeout: 10_000,
        message: 'clicking a tree item should set #id in the URL hash',
      })
      .toBeTruthy();

    await expect(placeholder).not.toBeVisible({ timeout: 10_000 });
    await expect(page.getByRole('button', { name: 'Body' })).toBeVisible({ timeout: 10_000 });
    await expect(page.locator('.markdown-body')).toBeVisible({ timeout: 10_000 });
  });

  test('tree selection follows URL hash when navigating browser history', async ({ page }) => {
    test.setTimeout(90_000);

    await gotoAndWaitForViewer(page, SPEC_VIEWER);

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
});
