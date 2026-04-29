import { test, expect } from '@playwright/test';
import type { ViewerConfig } from '../viewers';

/**
 * Suite: POST /api/client-log server endpoint contract.
 *
 * Spec ref: viewer-api/tracing/file-sink (479e226a)
 *   AC #1 — valid payload returns 204
 *   AC #6 — body exceeding 1 MiB returns 413 Payload Too Large
 *
 * These tests use Playwright's `request` fixture (no browser required) to
 * call the endpoint directly, independent of WASM initialisation.
 */
export function registerClientLogApiSuite(viewer: ViewerConfig): void {
  test.describe(`${viewer.name} — POST /api/client-log`, () => {
    const endpoint = (): string => `${viewer.url}/api/client-log`;

    const minimalRecord = () => ({
      ts: new Date().toISOString(),
      level: 'info',
      target: 'e2e.test',
      message: 'playwright end-to-end test record',
      fields: { source: 'client-log-api-suite' },
    });

    // ── AC #1: endpoint accepts a valid single-record payload ─────────────────

    test('valid payload returns 204 No Content', async ({ request }) => {
      const response = await request.post(endpoint(), {
        data: { records: [minimalRecord()] },
      });
      expect(response.status()).toBe(204);
    });

    // ── AC #1: degenerate case — empty records is a valid no-op ──────────────

    test('empty records array returns 204 No Content', async ({ request }) => {
      const response = await request.post(endpoint(), {
        data: { records: [] },
      });
      expect(response.status()).toBe(204);
    });

    // ── AC #1: endpoint rejects a structurally invalid body ───────────────────

    test('malformed JSON body returns 422 Unprocessable Entity', async ({ request }) => {
      const response = await request.post(endpoint(), {
        headers: { 'Content-Type': 'application/json' },
        data: '{ not: valid json }',
      });
      expect(response.status()).toBe(422);
    });

    // ── AC #1: endpoint rejects a valid JSON object that doesn't match schema ─

    test('JSON missing "records" field returns 422 Unprocessable Entity', async ({ request }) => {
      const response = await request.post(endpoint(), {
        data: { events: [minimalRecord()] }, // wrong field name
      });
      expect(response.status()).toBe(422);
    });

    // ── AC #6: body size limit enforced at 1 MiB ─────────────────────────────

    test('body exceeding 1 MiB returns 413 Payload Too Large', async ({ request }) => {
      // Build a payload whose JSON serialisation exceeds MAX_BODY_BYTES (1_048_576).
      // The overflow field alone is 1_050_000 bytes; surrounding JSON adds more.
      const response = await request.post(endpoint(), {
        data: {
          records: [
            {
              ...minimalRecord(),
              overflow: 'x'.repeat(1_050_000),
            },
          ],
        },
      });
      expect(response.status()).toBe(413);
    });

    // ── AC #1 (file-sink spec): multiple records in one batch are accepted ────

    test('batch of multiple records returns 204 No Content', async ({ request }) => {
      const batch = Array.from({ length: 10 }, (_, i) => ({
        ...minimalRecord(),
        message: `batch record ${i}`,
      }));
      const response = await request.post(endpoint(), {
        data: { records: batch },
      });
      expect(response.status()).toBe(204);
    });
  });
}
