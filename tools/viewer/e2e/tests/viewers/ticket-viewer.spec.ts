import { registerCommonViewerSuite } from '../shared/suites/common-viewer-suite';
import { registerDioxusThemeSuite } from '../shared/suites/dioxus-theme-suite';
import { TICKET_VIEWER } from '../shared/viewers';

registerCommonViewerSuite(TICKET_VIEWER);
registerDioxusThemeSuite(TICKET_VIEWER);
