Strengthen local pre-flight validation so expensive syntax-debugging loops are rejected before code is saved or finalized.

Requirements:
- run local syntax/format/lint checks at the relevant write boundary
- reject writes or surface immediate local failures when code is obviously broken
- keep the validation targeted and fast enough for routine use
- document repository expectations and fallback behavior when a check is unavailable