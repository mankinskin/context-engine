import { registerCommonViewerSuite } from '../shared/suites/common-viewer-suite';
import { registerDioxusThemeSuite } from '../shared/suites/dioxus-theme-suite';
import { SPEC_VIEWER } from '../shared/viewers';

registerCommonViewerSuite(SPEC_VIEWER);
registerDioxusThemeSuite(SPEC_VIEWER);
