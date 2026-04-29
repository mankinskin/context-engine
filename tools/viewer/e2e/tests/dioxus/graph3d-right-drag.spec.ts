import { test, expect } from '@playwright/test';
import { TICKET_VIEWER } from '../shared/viewers';

// Headless Chromium ships WebGPU behind a flag; without it the ticket viewer
// falls back to a DOM-only SVG dep-graph, and the Graph3D listeners under
// test never install. Use headed mode + software adapter to expose
// `navigator.gpu` reliably on Windows CI.
test.use({
  headless: false,
  launchOptions: {
    args: [
      '--enable-unsafe-webgpu',
      '--enable-features=Vulkan',
      '--use-vulkan=swiftshader',
      '--use-webgpu-adapter=swiftshader',
    ],
  },
});

/**
 * Verifies the fix in `tools/viewer/viewer-api/frontend/dioxus/src/graph3d/interaction.rs`:
 *
 * Right-button drags on the 3-D graph view must NOT trigger the browser
 * context menu (the menu is suppressed via a document-scoped `contextmenu`
 * listener that only `prevent_default()`s when the cursor moved while the
 * right button was held). Plain right-clicks (no drag) must still allow
 * the default context menu so the rest of the page is unaffected.
 */
test.describe('ticket-viewer — graph3d right-drag does not open context menu', () => {
  test('right-drag on graph suppresses contextmenu; plain right-click does not', async ({ page }) => {
    test.setTimeout(120_000);

    // Capture all console output (not just errors) for diagnostics.
    const consoleErrors: string[] = [];
    const consoleAll: string[] = [];
    page.on('pageerror', (err) => {
      consoleErrors.push(`pageerror: ${err.message}`);
    });
    page.on('console', (msg) => {
      consoleAll.push(`[${msg.type()}] ${msg.text()}`);
      if (msg.type() === 'error') {
        const text = msg.text();
        // Ignore unrelated 404s for missing static assets (favicons, etc.) —
        // they don't pertain to the right-drag interaction under test.
        if (/Failed to load resource.*404/.test(text)) return;
        consoleErrors.push(`console.error: ${text}`);
      }
    });

    // Pick a ticket that actually has dependencies (the Graph3D component
    // only mounts when the dep-graph has nodes/edges to render). Walk the
    // workspace until we find one whose subgraph has >= 1 edge.
    const listResp = await page.request.get(
      `${TICKET_VIEWER!.url}/api/tickets?workspace=default&limit=500`,
    );
    expect(listResp.ok(), 'ticket list API must respond').toBe(true);
    const listBody = await listResp.json();
    const ids: string[] = (listBody?.items ?? []).map((it: { id: string }) => it.id);
    expect(ids.length, 'workspace `default` must contain tickets').toBeGreaterThan(0);

    let firstId: string | undefined;
    for (const id of ids) {
      const sgResp = await page.request.get(
        `${TICKET_VIEWER!.url}/api/graph/subgraph?workspace=default&root=${id}&depth=2`,
      );
      if (!sgResp.ok()) continue;
      const sg = await sgResp.json();
      if ((sg?.edges?.length ?? 0) >= 1) {
        firstId = id;
        break;
      }
    }
    expect(firstId, 'no ticket with dependencies found in workspace').toBeTruthy();

    // The TicketListPage selects the open ticket from `localStorage` /
    // URL hash (`#id=…`). Visiting `/ticket/<id>` only redirects, which is
    // racy in tests — set localStorage upfront then load the list page.
    await page.goto(`${TICKET_VIEWER!.url}/workspace/default`, {
      waitUntil: 'domcontentloaded',
    });
    await page.evaluate(
      ({ key, id }) => {
        const raw = localStorage.getItem(key);
        const obj = raw ? JSON.parse(raw) : {};
        obj.open_ticket_id = id;
        localStorage.setItem(key, JSON.stringify(obj));
      },
      { key: 'ticket-viewer:default:ui', id: firstId! },
    );
    await page.goto(`${TICKET_VIEWER!.url}/workspace/default#id=${firstId}`, {
      waitUntil: 'domcontentloaded',
    });
    await page.locator(TICKET_VIEWER!.readySelector).first().waitFor({
      state: 'visible',
      timeout: TICKET_VIEWER!.readyTimeout,
    });

    // Wait for the Graph3D container to mount.
    const graphContainer = page.locator('#graph3d-nodes');
    await expect(graphContainer).toBeAttached({ timeout: 30_000 });

    // Wait for the WgpuOverlay-driven Graph3D to fully bootstrap. The
    // interaction listeners are installed *after* GPU init succeeds, which
    // can take several seconds on a software adapter.
    const graphRoot = page.locator('#graph3d-container');
    await expect(graphRoot).toBeAttached({ timeout: 30_000 });
    await page.waitForFunction(() => {
      const el = document.getElementById('graph3d-container');
      if (!el) return false;
      const cards = el.querySelectorAll('[data-node-idx]');
      // At least one card has been positioned (style display !== "none").
      for (const c of Array.from(cards)) {
        if ((c as HTMLElement).style.display !== 'none') return true;
      }
      return false;
    }, null, { timeout: 30_000 });
    await page.waitForTimeout(1_000);

    // Install a document-level capture listener that records whether the
    // most recent `contextmenu` event was prevented by the page. We read
    // this back after each gesture.
    await page.evaluate(() => {
      // @ts-expect-error attach test-only state to window
      window.__lastContextMenu = { fired: false, prevented: false };
      // @ts-expect-error attach test-only debug
      window.__rmbDebug = { mousedowns: 0, mousemoves: 0, mouseups: 0, cmCapture: 0, cmBubble: 0, cmTarget: '' };
      const dbg = (window as any).__rmbDebug;
      document.addEventListener('mousedown', (e) => {
        if ((e as MouseEvent).button === 2) dbg.mousedowns++;
      }, true);
      document.addEventListener('mousemove', (e) => {
        if ((e as MouseEvent).buttons & 2) dbg.mousemoves++;
      }, true);
      document.addEventListener('mouseup', (e) => {
        if ((e as MouseEvent).button === 2) dbg.mouseups++;
      }, true);
      // Capture-phase: runs BEFORE app's bubble-phase listener.
      document.addEventListener(
        'contextmenu',
        (evt) => {
          dbg.cmCapture++;
          dbg.cmTarget = (evt.target as HTMLElement)?.id || (evt.target as HTMLElement)?.tagName || '?';
        },
        true,
      );
      document.addEventListener(
        'contextmenu',
        (evt) => {
          dbg.cmBubble++;
          // @ts-expect-error
          window.__lastContextMenu = { fired: true, prevented: evt.defaultPrevented };
        },
        false,
      );
    });

    // ── Gesture 1: right-button DRAG over the graph ─────────────────
    // Aim for the centre of the Graph3D container itself — the WebGPU
    // canvas spans the whole viewport (and has `pointer-events: none`),
    // but the container only covers the middle panel between the sidebar
    // and the ticket detail. The mousedown listener is on `#graph3d-container`.
    const container = page.locator('#graph3d-container');
    await expect(container).toBeVisible();
    const box = await container.boundingBox();
    expect(box).not.toBeNull();
    const cx = box!.x + box!.width / 2;
    const cy = box!.y + box!.height / 2;

    await page.evaluate(() => {
      // @ts-expect-error reset before the gesture
      window.__lastContextMenu = { fired: false, prevented: false };
    });

    await page.mouse.move(cx, cy);
    await page.mouse.down({ button: 'right' });
    // Several intermediate moves to ensure mousemove fires and arms the flag.
    await page.mouse.move(cx + 30, cy + 10, { steps: 5 });
    await page.mouse.move(cx + 60, cy + 25, { steps: 5 });
    await page.mouse.up({ button: 'right' });

    // contextmenu fires synchronously on mouseup for the right button.
    await page.waitForTimeout(150);

    const afterDrag = await page.evaluate(() => {
      // @ts-expect-error
      const lc = window.__lastContextMenu as { fired: boolean; prevented: boolean };
      // @ts-expect-error
      const dbg = window.__rmbDebug as { mousedowns: number; mousemoves: number; mouseups: number };
      return { ...lc, ...dbg };
    });
    console.log('AFTER_DRAG', JSON.stringify(afterDrag));

    expect(
      afterDrag.fired,
      'contextmenu event should still fire after right-drag (so we can check prevention)',
    ).toBe(true);
    expect(
      afterDrag.prevented,
      'right-drag on graph must call preventDefault() on the contextmenu event',
    ).toBe(true);

    // ── Gesture 2: plain right-CLICK (no movement) on the graph ─────
    // The fix should leave plain right-clicks alone (no preventDefault).
    await page.evaluate(() => {
      // @ts-expect-error
      window.__lastContextMenu = { fired: false, prevented: false };
    });

    await page.mouse.move(cx, cy);
    await page.mouse.down({ button: 'right' });
    await page.mouse.up({ button: 'right' });
    await page.waitForTimeout(150);

    const afterClick = await page.evaluate(() => {
      // @ts-expect-error
      return window.__lastContextMenu as { fired: boolean; prevented: boolean };
    });

    expect(afterClick.fired, 'plain right-click should still trigger contextmenu').toBe(true);
    expect(
      afterClick.prevented,
      'plain right-click (no drag) must NOT be suppressed by the graph view',
    ).toBe(false);

    // ── Final check: no console errors during the whole interaction ──
    expect(consoleErrors, 'graph3d right-drag interaction produced JS errors').toEqual([]);
  });
});
