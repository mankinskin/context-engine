/**
 * E2E tests for the WASM structured tracing feature.
 *
 * Covers:
 *   - Console-layer behaviour (subscriber installed, default filter, opt-in
 *     filter overrides via URL param and localStorage).
 *   - Network-sink behaviour (?log_sink=on URL flag, localStorage opt-in,
 *     filter blocks events from reaching the sink).
 *   - POST /api/client-log server endpoint contract (status codes, body-size
 *     limit, malformed payloads).
 *
 * Specifications:
 *   - viewer-api/tracing        (b06c9df8-2866-433a-af73-ae9b1f4a0f0a)
 *   - viewer-api/tracing/file-sink (479e226a-b4ef-4e30-ade0-ebdabbf956ed)
 *
 * Run:
 *   cd tools/viewer/e2e
 *   npx playwright test tests/viewers/tracing.spec.ts
 */

import { registerTracingConsoleSuite } from '../shared/suites/tracing-console-suite';
import { registerClientLogApiSuite } from '../shared/suites/client-log-api-suite';
import { SPEC_VIEWER, TICKET_VIEWER } from '../shared/viewers';

// ── Console-layer + network-sink behaviour ────────────────────────────────────

registerTracingConsoleSuite(SPEC_VIEWER);
registerTracingConsoleSuite(TICKET_VIEWER);

// ── Server endpoint contract ──────────────────────────────────────────────────

registerClientLogApiSuite(SPEC_VIEWER);
registerClientLogApiSuite(TICKET_VIEWER);
