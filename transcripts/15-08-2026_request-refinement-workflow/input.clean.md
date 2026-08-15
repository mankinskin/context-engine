# Request: Formalize a Request-Refinement Workflow

Look at [transcripts/15-08-2026_verification-first-workflow/](../15-08-2026_verification-first-workflow/) and use it as the model for our end-to-end request-to-solution workflow.

In that example, a purely unstructured raw input transcript was structured step by step:

1. Starting from the raw input, a cheap agent cleaned it up.
2. The cleaned input was then reviewed for structure.
3. Influenced by research, the reviewed input was further improved, and new files were created.

We want to always run this workflow in the future, before we take a user's raw prompt and use it to kick off a more complex downstream workflow — for example, creating tickets for it — without going ahead and implementing anything yet.

This workflow could also be applied directly right now, to carry out this very request.
