# Dialogbeispiel: Verstehen eines Ticket-Zustands

## Ausgangslage

Ein Mensch moechte verstehen, warum ein Ticket noch nicht umgesetzt werden kann.
Der Explainer Agent hat das Ticket und seine Abhaengigkeiten gelesen.

## Frage des Explainer Agents

> Soll ich zuerst den Ticket-Zustand und die offenen Abhaengigkeiten erklaeren,
> oder kennst du den Ticket-Zustand bereits?

Der Mensch antwortet: "Bitte erst erklaeren."

## Erklaerung

**Bestaetigter Fakt:** Das Ticket ist erst dann umsetzbar, wenn seine
Abhaengigkeiten abgeschlossen sind.

**Annahme:** Der Mensch moechte die Abhaengigkeiten im aktuellen Worktree selbst
ueberpruefen.

**Empfehlung:** Lies zuerst den Abhaengigkeitsgraphen, statt eine Aenderung zu
beginnen. Dadurch wird sichtbar, welche Arbeit den Start blockiert.

## Menschlicher Schritt

- **Zweck:** Den aktuellen Abhaengigkeitsgraphen sichtbar machen.
- **Warum jetzt:** Ohne den Graphen kann nicht unterschieden werden, ob der
  Blocker fachlich, technisch oder bereits erledigt ist.
- **Menschliche Handlung:** Fuehre im zugeordneten Worktree selbst aus:

  ```bash
  ticket subgraph <ticket-id> --direction out --toon
  ```

- **Erwartetes Signal:** Die Ausgabe nennt das Ticket und alle offenen
  Abhaengigkeiten.
- **Typische Abweichung:** Der Befehl meldet, dass das Ticket unbekannt ist oder
  keine Abhaengigkeiten besitzt.
- **Naechste Frage:** Welche Ticket-IDs oder Fehlermeldung zeigt deine Ausgabe?

## Rueckmeldung und Fortsetzung

Wenn der Mensch offene Abhaengigkeiten meldet, erklaert der Explainer Agent,
welche Abhaengigkeit zuerst behandelt werden muss und warum. Wenn keine
Abhaengigkeiten erscheinen, fragt der Explainer Agent nach dem Ticket-Zustand
oder einer weiteren Fehlermeldung. Der Explainer Agent fuehrt den Befehl nicht
selbst aus und sendet keine Terminal-Eingabe.
