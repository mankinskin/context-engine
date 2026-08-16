# Traceability Checklist

| Rohanforderung | Dossierziel | Geplanter Nachweis |
| --- | --- | --- |
| Deutsch und Sprachadaption | Strang A | Deutscher Dialogfall und expliziter Sprachwechsel |
| Problem verstehen und erklaeren | Strang A | Fakten, Ursache, Zielbild, Alternativen und Risiken im Beispiel |
| Mensch fuehrt Loesung aus | Alle Straenge | Keine Agent-Eingabe oder Mutation in Tool-Grenztest |
| Frage-Antwort-Format | Strang A und B | Dialogischer Beispielablauf mit Rueckfragen |
| Beobachtbares Terminal | Strang C | Sitzungs- und Ausgabereadback ohne Agent-Eingabe |
| Teacher delegiert Explore | Strang B | Delegationsvertrag und Tool-Grenztest |
| Lektionen mit mehreren Aufgaben | Strang B | Lektionsbeispiel mit drei Aufgaben |
| Verifizieren statt benoten | Strang B und C | Statuswerte abgeschlossen, wiederholen, offen; keine Pass/Fail-Skala |
| Menschliche Rueckmeldung | Strang A und B | Rueckfrage und dokumentierte naechste Lehrentscheidung |

## Deterministische Artefaktpruefung

- `input.md`, `input.clean.md`, `REVIEW.md`, diese Checkliste und
  `01-implementation-slices.md` existieren und sind nicht leer.
- Eine neue Teacher-Agent-Vorlage wird erst mit sechs Frontmatter-Feldern und
  sechs Vertragsabschnitten erstellt.
- Ein Mensch-Terminal wird erst nach einer eigenen Tool-Grenz- und
  Integrationsspezifikation implementiert.

## Verbleibende offene Frage

Die konkrete UI fuer das Mensch-Terminal ist noch nicht entschieden. Diese
Frage blockiert nur Strang C, nicht die deutsche Explainer- oder
Teacher-Vertragsarbeit.
