# Deutscher Explainer und Teacher Agent

## Lesereihenfolge

1. [input.md](input.md) bewahrt die Rohanforderung.
2. [input.clean.md](input.clean.md) enthaelt die bereinigten und bestaetigten
   Entscheidungen.
3. [REVIEW.md](REVIEW.md) begrenzt die Risiken und den Umfang.
4. [01-implementation-slices.md](01-implementation-slices.md) beschreibt die
   drei unabhaengigen Umsetzungsstraenge.
5. [02-completion-checklist.md](02-completion-checklist.md) ordnet jede
   Anforderung einem pruefbaren Ergebnis zu.

## Entscheidungsgrenze

Dieses Dossier ist eine Forschungs- und Planungsgrundlage. Es aendert keine
Agentenvorlage, kein Tool und keinen Storevertrag. Die Umsetzung beginnt erst,
nachdem eigene Tickets und Spezifikationen die drei Straenge, ihre Tool-Grenzen
und ihre Validierungen verbindlich festgelegt haben.

Die daraus entstandene Spezifikation ist
[03d93adb Interactive human learning guidance](.spec/specs/03d93adb-59a8-44be-af95-3b4b208e7e9a/spec.toml).
Die drei Umsetzungs-Tickets sind
[9f617940 Make Explainer Agent German-first and interactive](.ticket/tickets/9f617940-a3fd-4990-b3fd-a3fa95c10ae7/ticket.toml),
[f3cc69a4 Add Teacher Agent lesson guidance](.ticket/tickets/f3cc69a4-03de-4b45-8b87-a548d5669afe/ticket.toml)
und
[ea52bd6f Add human-owned observer terminal sessions](.ticket/tickets/ea52bd6f-aa48-43f5-9228-0bff7190abf8/ticket.toml).
