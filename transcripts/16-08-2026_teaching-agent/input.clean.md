# Bereinigte Anforderung: Deutscher Explainer und Teacher Agent

## Ziel

Menschen sollen technische Aufgaben, Probleme und Systeme nachvollziehen und
anschliessend selbst loesen koennen. Agenten recherchieren, strukturieren und
erklaeren; Menschen behalten die Ausfuehrungsautoritaet.

## Explainer Agent

- Arbeitet standardmaessig auf Deutsch.
- Recherchiert einen Sachverhalt, erklaert Problem, Ursache, Zielbild,
  Alternativen, Risiken und Validierungsschritte.
- Versteht und beschreibt einen moeglichen Loesungsweg, fuehrt die eigentliche
  Loesung aber nicht selbst aus.
- Nutzt ein gefuehrtes Frage-Antwort-Format, um Voraussetzungen, Verstaendnis,
  Entscheidungen und Rueckmeldungen sichtbar zu machen.
- Darf Terminalausgabe beobachten, darf aber Menschen nur zu eigenen Eingaben
  auffordern und darf keine Befehle oder Mutationen selbst ausfuehren.
- Erklaert zu jedem menschlichen Schritt: was geschieht, warum der Schritt
  notwendig ist, welches Ergebnis erwartet wird und wie Erfolg oder Fehler
  erkennbar sind.

## Teacher Agent

- Ist ein eigener, benutzeraufrufbarer Lern-Agent mit einer klaren Grenze zum
  Explainer Agent.
- Akzeptiert eine Fragestellung, ein Problem, ein System oder eine zu loesende
  Aufgabe als Eingang.
- Nutzt begrenzte Recherche, gegebenenfalls ueber den Explorer Agent, um einen
  Lernplan zu erstellen.
- Synthesisiert eine Lektion aus kleinen, menschlich ausfuehrbaren Aufgaben fuer
  Terminal oder UI.
- Fuehrt Menschen dialogisch durch die Lektion, erklaert jeden Schritt und
  verarbeitet menschliche Rueckmeldungen fuer den naechsten Lehrschritt.
- Darf Aufgaben wiederholen oder vereinfachen, bewertet Menschen aber nicht als
  bestanden oder nicht bestanden.
- Kann vor dem Start einer Lektion eine menschliche Review-Freigabe verlangen.

## Gemeinsame Grenzen

- Menschen fuehren Befehle, UI-Aktionen und andere Mutationen selbst aus.
- Agenten duerfen keine menschlichen Ergebnisse, Bewertungen oder
  Ausfuehrungen erfinden.
- Beobachtete Terminalausgabe ist Evidenz, ersetzt aber keine menschliche
  Entscheidung oder Ausfuehrung.
- Rueckmeldungen dienen der Verbesserung von Erklaerungen und Lektionen, nicht
  der automatischen Aenderung von Agentenvorlagen oder Berechtigungen.

## Bestaetigte Entscheidungen

1. Deutsch ist Standard; der Explainer und Teacher passen sich auf Wunsch an
  die Sprache des Menschen an.
2. Ein beobachtbares Mensch-Terminal ist das bevorzugte Komfort-Interface.
  Menschen geben dort Befehle ein; Agenten duerfen Ausgabe lesen, aber keine
  Eingabe in die Sitzung senden.
3. Der Teacher Agent delegiert Recherche intern an den Explore Agent und
  verwaltet selbst Lektion, Reihenfolge, Erklaerungen und Aufgaben.
4. Menschen muessen keine Lektion oder Aufgabe genehmigen. Sie beantworten
  Fragen und loesen Aufgaben im zugeordneten Worktree.
5. Der Teacher Agent ist ein beratender Verifizierer: Er prueft Ergebnisse mit
  seinen erlaubten Mitteln, erklaert Abweichungen und kann um menschliche
  Entscheidung bitten. Menschen werden nicht automatisch benotet.

## Abgeleitete Produktgrenzen

- Ein beobachtbares Mensch-Terminal ist eine neue Infrastruktur-Capability. Die
  bestehende `execute`- oder `compact-terminal-mcp`-Freigabe ist kein Ersatz,
  weil beide Agenten selbst Befehle ausfuehren lassen koennen.
- Terminal-Aufgaben muessen den Arbeitsordner, den vom Menschen einzugebenden
  Befehl, erwartete Ausgabe, sichere Fehlerbeobachtung und naechste Frage
  enthalten.
- Der Teacher Agent darf keine Terminal-Eingaben senden und keine Aufgabe,
  Datei, UI oder Store-Mutation selbst ausfuehren.