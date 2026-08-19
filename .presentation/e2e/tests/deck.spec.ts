import { test, expect } from '@playwright/test';

test('context-engine deck loads its first slide', async ({ page }) => {
  await page.goto('http://localhost:3030');
  await expect(page.getByRole('heading', { name: 'context-engine' })).toBeVisible();
  await page.screenshot({ path: 'screenshots/context-engine-slide-1.png' });
});

test('workflow-tools deck loads its first slide standalone', async ({ page }) => {
  await page.goto('http://localhost:3031');
  await expect(page.getByRole('heading', { name: 'Workflow Tools' })).toBeVisible();
  await page.screenshot({ path: 'screenshots/workflow-tools-slide-1.png' });
});
