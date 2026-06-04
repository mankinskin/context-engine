Create a token-bounded file inspection utility that defaults to narrow line windows instead of whole-file reads.

Requirements:
- require explicit target line coordinates or bounded windows by default
- make line-range inspection the default interaction pattern
- preserve a clear escape hatch for larger reads when truly necessary
- document the intended agent usage pattern so full-module pulls become the exception