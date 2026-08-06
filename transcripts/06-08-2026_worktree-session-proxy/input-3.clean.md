# Work Tree Reuse and Renaming Within a Session

A new problem has appeared: when a session changes topic or needs a different work tree, the change currently has to happen through an agent. Creating a brand-new work tree for every request would be too expensive and would produce too many work trees. The system would then need to decide very often whether to close or keep each work tree.

A simpler alternative is to reuse the same work tree for the session. The main downside is that the work-tree names may no longer be as human-readable, but that may be acceptable if the same work tree can be renamed when needed.

Renaming should be possible through the `worktree move` command, and the associated branch should also be renamed. That would let an agent adjust the work tree name and topic while keeping the same session and the same underlying work tree.

In that model, the session always uses the same work tree. The work tree is initialized at the start of the session, between user turns, and before a user request is processed. Nothing special happens there by default, but agents can still change the work tree's name when they are working on a new topic.

This model matches the desired behavior well: the session keeps moving forward, previous states can be overwritten, and each session still has its own work tree.