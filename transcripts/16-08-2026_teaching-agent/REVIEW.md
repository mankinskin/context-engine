# Review: Deutscher Explainer und Teacher Agent

## Urteil

**Aenderungen erforderlich.** Die Anforderung beschreibt ein klares
menschenzentriertes Lernziel, vermischt aber eine reine Lese- und
Erklaerungsrolle mit Terminal-Beobachtung und interner Delegation. Beide
Capabilities brauchen eine ausdrueckliche Entscheidung, bevor Ticket oder
Spezifikation geaendert werden.

## Bestaetigte Ausgangslage

- `.agents/agents/explainer.agent.md` ist derzeit englisch und auf
  `read`, `search`, `vscodeGeneral/toolSearch` und `peek-mcp/*` beschraenkt.
- Der bestehende Explainer-Vertrag verbietet Ausfuehrung, Mutationen,
  Feedback-Schreiben und Delegation.
- Ein Teacher Agent existiert nicht.
- Der bestehende Explainer-Ticket- und Spezifikationsvertrag verlangt weiterhin
  menschliche Ausfuehrung und Rueckmeldung, enthaelt aber keinen
  Lektionen- oder Uebungsablauf.

## Befunde

| Schwere | Befund | Erforderliche Entscheidung |
| --- | --- | --- |
| hoch | Terminal-Beobachtung setzt mindestens eine Ausfuehrungs- oder Terminal-Session-Capability voraus. | Beobachtung als neue Capability zulassen oder auf vom Menschen gepostete Ausgabe beschraenken. |
| hoch | Interne Explorer-Nutzung ist Delegation und widerspricht der bestehenden Nichtdelegationsgrenze. | Teacher darf Explorer delegieren oder Teacher recherchiert selbst mit begrenzten Lesetools. |
| hoch | Eine Lektion braucht eine Freigabegrenze, bevor Menschen Aufgaben ausfuehren. | Gesamte Lektion oder jede Aufgabe einzeln freigeben. |
| mittel | Terminal- und UI-Aufgaben benoetigen eine portable Darstellungsform. | Befehle als kopierbare Schritte mit erwarteter Ausgabe und Fehlerpfad definieren. |
| mittel | Der Wunsch nach "bestehen" steht im Konflikt mit dem Ziel, nicht zu benoten. | Nur Fortschritt, Wiederholung und menschliche Bestaetigung erfassen; keine Pass/Fail-Bewertung. |

## Begrenzter Umfang

Das anschliessende Dossier darf enthalten:

- einen deutschen, interaktiven Explainer-Vertrag;
- einen separaten Teacher-Agent-Vertrag mit Lektionen und menschlichen
  Uebungen;
- eine klare Ausfuehrungs-, Terminal- und Delegationsgrenze;
- eine minimale, nicht bewertende Fortschritts- und Evidenzform;
- Ticket-, Spezifikations- und Validierungsanforderungen.

Das anschliessende Dossier darf nicht enthalten:

- autonome Befehlsausfuehrung, Mutation oder UI-Aktion durch Explainer oder
  Teacher;
- erfundene menschliche Resultate;
- automatische Bewertung oder Benotung von Menschen;
- automatische Aenderung von Vorlagen, Berechtigungen oder Routing anhand von
  Lektionen.