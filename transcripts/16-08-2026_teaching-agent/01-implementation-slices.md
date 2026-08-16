# Umsetzungsstränge: Deutscher Explainer, Teacher und Mensch-Terminal

## Voraussetzung

Die Entscheidungen in [input.clean.md](input.clean.md) sind verbindlich. Die
folgenden Stränge sind eine Umsetzungsplanung, keine bereits implementierte
Capability.

## Strang A: Deutscher dialogischer Explainer

**Ergebnis:** Die vorhandene Explainer-Vorlage antwortet standardmaessig auf
Deutsch, passt sich einer ausdruecklich gewuenschten anderen Sprache an und
fuehrt Menschen in einem Frage-Antwort-Ablauf durch das Verstehen eines
Problems.

**Vertrag:**

- Recherchiert Fakten und benennt Ursache, Zielbild, Alternativen und Risiken.
- Fragt vor jedem vorgeschlagenen menschlichen Schritt nach fehlendem Kontext
  oder Verstaendnis.
- Beschreibt jeden Schritt als: Zweck, warum jetzt, menschliche Handlung,
  erwartetes Signal, typische Abweichung und naechste Frage.
- Loest keine Aufgabe selbst und sendet keine Terminal- oder UI-Eingabe.

**Nichtziel:** Ein Beobachtungsterminal oder eine neue Ausfuehrungsfreigabe.

**Validierung:** Statische Vorlagenpruefung und ein dialogischer Beispielablauf
mit deutscher Standardsprache, Sprachwechsel und klarer menschlicher Handlung.

## Strang B: Teacher Agent und Lektionen

**Ergebnis:** Eine neue benutzeraufrufbare Teacher-Agent-Vorlage nimmt ein
Problem oder Lernziel entgegen und erzeugt eine Lektion aus kleineren,
menschlich ausfuehrbaren Aufgaben.

**Vertrag:**

- Delegiert begrenzte Repository-Recherche an den Explore Agent.
- Verwaltet selbst Lernziel, Voraussetzungen, Reihenfolge, Erklaerungen,
  Rueckfragen, Wiederholung und Zusammenfassung.
- Gibt Menschen Aufgaben fuer Terminal oder UI, ohne selbst eine Mutation
  auszufuehren.
- Prueft menschliche Rueckmeldungen und beobachtete Ausgabe gegen erwartete
  Signale; bei Unsicherheit erklaert er die Abweichung und fragt nach.
- Dokumentiert Fortschritt als abgeschlossene, wiederholte oder offene Aufgabe,
  nicht als Bestehen oder Nichtbestehen.

**Nichtziel:** Automatische Bewertung von Menschen, automatische Aenderung von
Vorlagen oder verdeckte Befehlsausfuehrung.

**Validierung:** Statische Tool-Grenzen, ein Lektionsbeispiel mit mindestens
drei Aufgaben und ein Test, dass der Teacher nur Explore delegiert und keine
Ausfuehrungsdelegation startet.

## Strang C: Beobachtbares Mensch-Terminal

**Ergebnis:** Eine neue, explizit eingeschraenkte Terminal-Sitzung, in der ein
Mensch Befehle eingibt und ein Agent nur Ausgabe, Exit-Zustand sowie Arbeitsort
lesen kann.

**Vertrag:**

- Der Agent kann eine Sitzung mit Arbeitsverzeichnis und Anzeigenamen erstellen.
- Der Agent kann Ausgabe in begrenzten Fenstern lesen und den Sitzungsstatus
  abfragen.
- Nur die Mensch-UI kann Eingaben senden; die Agent-API besitzt keine
  `send_input`-, `execute`- oder Shell-Argument-Operation.
- Die Ausgabe referenziert eine Sitzung und ist als Evidenz fuer einen
  Lernschritt lesbar.
- Sitzungen enden explizit oder bei einem sicheren Timeout; alte Ausgabe bleibt
  nicht als aktuelle Evidenz missverstaendlich.

**Abhaengigkeit:** Der offene Research-Track
[0dd23fe6 Audit execute MCP tools for terminal reuse](.ticket/tickets/0dd23fe6-6892-4d21-9927-4a81584dc77a/ticket.toml)
liefert die Bestandsaufnahme. Eine Implementierung darf erst nach dessen
Entscheidung fuer eine sichere, wiederverwendbare Terminal-Sitzung beginnen.

**Validierung:** Ein Integrationsfall startet die Beobachtungssitzung, simuliert
menschliche Ausgabe ueber die UI-Grenze, liest sie begrenzt aus und beweist,
dass keine Agent-Operation Eingabe senden kann.

## Abhaengigkeiten und Reihenfolge

1. Strang A kann sofort nach einer Vertragsamendierung umgesetzt werden.
2. Strang B braucht Strang A als Erklaerungsformat, nicht aber Strang C.
3. Strang C braucht die Research-Entscheidung aus `0dd23fe6` und liefert danach
die komfortable Terminal-Evidenz fuer Strang B.

## Offene technische Entscheidung

Die UI-Form des Mensch-Terminals ist noch nicht festgelegt: VS-Code-Terminal,
eigene Viewer-Ansicht oder eine andere sichere UI-Integration. Der Vertrag muss
vor Implementierung festlegen, wo die Mensch-Eingabe stattfindet und wie die
Leseberechtigung technisch ohne Agent-Eingaberecht durchgesetzt wird.
