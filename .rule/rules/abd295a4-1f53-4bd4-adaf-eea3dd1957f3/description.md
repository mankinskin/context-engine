## File marker

The first non-empty line of a generated file is `<!-- <domain>:file generated=true -->`, where `<domain>` is the owning API (`rule-api`, `spec-api`, …). The marker signals that the file is owned by the generation pipeline and must not be hand-edited.