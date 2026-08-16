# Lektionsbeispiel: Einen Ticket-Blocker verstehen

## Lernziel

Der Mensch kann erkennen, warum ein Ticket blockiert ist, seine
Abhaengigkeiten lesen und einen naechsten menschlichen Arbeitsschritt ableiten.

## Aufgabe 1: Ticket lesen

- **Status:** `open`
- **Arbeitsort:** Zugeordneter Worktree
- **Zweck:** Den aktuellen Ticketzustand und sein Ziel verstehen.
- **Begruendung:** Ein Blocker kann erst erklaert werden, wenn Ziel und Zustand
  bekannt sind.
- **Menschliche Handlung:** Lies das Ticket mit dem vorhandenen Ticket-Werkzeug
  oder der Ticket-Ansicht.
- **Erwartetes Signal:** Titel, Zustand und Ziel des Tickets sind sichtbar.
- **Typische Abweichung:** Das Ticket ist nicht erreichbar oder die Ausgabe
  enthaelt keinen Zustand.
- **Verifikation:** Der Mensch nennt Titel und Zustand in eigenen Worten.
- **Naechste Frage:** Welchen Zustand und welches Ziel zeigt das Ticket?

## Aufgabe 2: Abhaengigkeiten untersuchen

- **Status:** `open`
- **Arbeitsort:** Zugeordneter Worktree
- **Zweck:** Die Arbeit identifizieren, die das Ticket blockiert.
- **Begruendung:** Eine Abhaengigkeit erklaert, warum eine Loesung noch nicht
  begonnen oder abgeschlossen werden kann.
- **Menschliche Handlung:** Fuehre selbst eine Ticket-Graph-Abfrage fuer das
  Ticket aus oder oeffne die Abhaengigkeiten in der UI.
- **Erwartetes Signal:** Die Ausgabe nennt offene Voraussetzungen oder zeigt,
  dass keine Voraussetzung existiert.
- **Typische Abweichung:** Die Graph-Abfrage findet kein Ticket oder die
  Richtung ist unklar.
- **Verifikation:** Der Mensch nennt die erste offene Voraussetzung oder
  bestaetigt, dass keine sichtbar ist.
- **Naechste Frage:** Welche Voraussetzung blockiert das Ticket, oder welche
  andere Evidenz fehlt?

## Aufgabe 3: Naechsten Schritt formulieren

- **Status:** `open`
- **Arbeitsort:** Zugeordneter Worktree oder Ticket-UI
- **Zweck:** Einen kleinen, menschlich ausfuehrbaren Folgeschritt waehlen.
- **Begruendung:** Ein erklaerter Blocker ist erst nuetzlich, wenn daraus eine
  konkrete Handlung folgt.
- **Menschliche Handlung:** Beschreibe den kleinsten Schritt, der die offene
  Voraussetzung reduziert, und fuehre ihn nur selbst aus, wenn du bereit bist.
- **Erwartetes Signal:** Der Schritt nennt ein Ziel, einen Arbeitsort und eine
  pruefbare Beobachtung.
- **Typische Abweichung:** Der Schritt ist zu gross, mutiert mehrere Bereiche
  oder hat kein pruefbares Ergebnis.
- **Verifikation:** Der Teacher Agent fragt nach Ziel, Arbeitsort und erwartetem
  Signal; bei einer Luecke markiert er die Aufgabe `repeat`.
- **Naechste Frage:** Ist der Schritt klein genug, um sein Ergebnis eindeutig zu
  beobachten?

## Wiederholung ohne Benotung

Wenn Aufgabe 2 keine eindeutige Voraussetzung zeigt, bleibt sie `repeat`. Der
Teacher Agent erklaert die unklare Ausgabe, fragt nach einem Ausschnitt oder
einer anderen Ansicht und gibt keine Bestehen- oder Nichtbestehen-Bewertung ab.
