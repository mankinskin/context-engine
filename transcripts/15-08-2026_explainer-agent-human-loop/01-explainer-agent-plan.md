# Explainer Agent: Review- und Umsetzungsplan

## Quelle und Status

Dieser Plan leitet sich von [input.clean.md](input.clean.md) ab. Die Quelle
beschreibt einen menschenzentrierten Agenten, der eine Aufgabe und ihren Kontext
erklärt, bevor ein Mensch die Aufgabe ausführt. Der Mensch bewertet anschließend
Erklärung und Ergebnis. Die Rückmeldungen verbessern spätere Erklärungen und
machen häufige Interaktionsmuster sichtbar.

**Review-Status: Der Version-eins-Vertrag ist entschieden.** Die geprüften
Entscheidungen stehen in [00-review-verbesserungen.md](00-review-verbesserungen.md).
Vor der Vorlagenerstellung müssen Ticket `79449c3f-2f49-4925-b8fd-3751face53b5`
und eine verknüpfte Spezifikation die konkrete Feedback-Speicherung, Abfragen
und Validierung präzisieren.

## Bestätigte Anforderungen

Der Explainer Agent muss:

1. eine Aufgabe, ihren Kontext und einen begrenzten Ansatz erklären, bevor ein
   Mensch die Aufgabe ausführt;
2. den Menschen in der Kontrolle halten, indem er eine explizite Gelegenheit
   zum Bestätigen, Überarbeiten, Eingrenzen, Ablehnen oder Weitergeben bietet;
3. dem Menschen ermöglichen, die Qualität der Erklärung und des ausgeführten
   Ergebnisses direkt zu bewerten;
4. Feedback erfassen, damit spätere Explainer-Agent-Läufe die Qualität von
   Erklärungen verbessern und häufig verwendete Interaktionsmuster erkennen
   können;
5. die Autorität des Menschen wahren, die vorgeschlagene Arbeit zu stoppen oder
   an einen geeigneten ausführenden Prozess weiterzugeben.

## Umfang und Nichtziele

### Umfang von Version eins

- Eine neue Agentenvorlage unter `.agents/agents/explainer.agent.md`.
- Vollständige, nicht mutierende Repository-Recherche vor der Erklärung.
- Ein wiederholbarer Ablauf: lesen -> erklären -> menschlich ausführen ->
  bewerten.
- Dauerhafte, teamweit sichtbare Feedback-Erfassung mit Bezug zur Aufgabe,
  einem Ticket oder einer Spezifikation, wenn ein Anker existiert.
- Eine prüfbare Zusammenfassung von Recherche, Erklärung, menschlicher
  Entscheidung und Bewertung.
- Aggregierte Interaktionsnachweise, die spätere menschlich geprüfte
  Vorlagenänderungen begründen können.

### Nichtziele von Version eins

- Der Explainer Agent führt keine Aufgabe selbst aus und delegiert keine
  Ausführung an einen anderen Agenten.
- Der Explainer Agent mutiert keine Dateien, Stores oder Dienste.
- Training oder Feinabstimmung eines Basismodells aus Benutzerfeedback.
- Automatische Änderungen an Vorlage, Berechtigungen, Modellrouting oder
  Anweisungen auf Grundlage von Laufzeit-Feedback.
- Ableiten sensibler Präferenzen aus nicht zusammenhängenden Gesprächen.
- Ersetzen der spezialisierten Vorlagen für Implementierung, Interview, Review
  oder Tests.

## Produktvertrag

### Ablauf

1. **Aufgabe entgegennehmen.** Der Explainer Agent bestimmt Ziel, relevante
   Repository-Entitäten und den angefragten Rahmen.
2. **Repository lesen.** Der Explainer Agent darf Repository-Inhalte vollständig
   lesen, um Fakten, Einschränkungen, Risiken und relevante Nachweise zu
   ermitteln. Diese Recherche verändert keine Repository-Daten.
3. **Erklären.** Der Explainer Agent präsentiert eine präzise,
   nachweisgestützte Erklärung mit Ziel, Erkenntnissen, vorgeschlagenen
   menschlichen Schritten, betroffenen Dateien oder Entitäten, Risiken,
   Validierung und Annahmen.
4. **Menschliche Entscheidung einholen.** Der Mensch wählt `approve`, `revise`,
   `narrow`, `decline` oder `delegate`. In Version eins bestätigt `approve` die
   Erklärung und den vorgeschlagenen menschlichen Arbeitsplan; der Explainer
   Agent führt danach keine Arbeit aus.
5. **Menschliche Ausführung.** Der Mensch führt die gewählte Aufgabe selbst aus
   oder startet einen geeigneten bestehenden Prozess. Die Ausführung liegt
   außerhalb des Explainer Agents.
6. **Bewerten.** Der Mensch bewertet Erklärung und Ergebnis und kann eine kurze
   Freitext-Rückmeldung geben.
7. **Nachweise erfassen.** Der Explainer Agent speichert Erklärung, Entscheidung,
   Bewertungen, Rückmeldung und Interaktionskategorie in den dauerhaften Stores.
8. **Spätere Erklärungen verbessern.** Eine getrennte
   Vorlagenwartungsüberprüfung darf aggregierte Rückmeldungen für Vorschläge
   nutzen. Kein einzelner Lauf darf seine eigene Vorlage oder sein Verhalten
   automatisch umschreiben.

### Anforderungen an die Erklärung

Jede Erklärung muss enthalten:

- das gewünschte Ergebnis in verständlicher Sprache;
- aus dem Repository ermittelte Fakten, getrennt von Annahmen und Empfehlungen;
- bekannte Einschränkungen und offene Annahmen;
- die vorgeschlagene Abfolge menschlicher Schritte;
- die vorgesehenen Dateien, Tickets, Spezifikationen oder Dienste;
- die Validierungsmethode und den erwarteten Nachweis;
- Risiken, Nichtziele und die vom Menschen benötigte Entscheidung.

Eine Erklärung darf nicht behaupten, dass ein Befehl, ein Test oder eine Änderung
erfolgreich war, bevor ein Nachweis vorliegt.

### Anforderungen an menschliche Kontrolle

- Der Explainer Agent besitzt in Version eins keine Mutationswerkzeuge.
- `revise` und `narrow` erzeugen eine neue Erklärung, bevor der Mensch einen
  überarbeiteten Plan ausführt.
- `decline` beendet den vorgeschlagenen Arbeitsweg ohne Ausführung durch den
  Explainer Agent.
- `delegate` dokumentiert die Entscheidung, dass der Mensch einen geeigneten
  spezialisierten Prozess auswählt; der Explainer Agent startet keine Delegation.
- Eine spätere ausführende Version muss bei einem neuen Ziel oder einem
  geänderten Ergebnis eine erneute Zustimmung einholen.
- Die gespeicherte Bewertung benennt den konkreten Lauf sowie, wenn vorhanden,
  Ticket- oder Spezifikationsanker.

### Anforderungen an Lernen und Feedback

Für Version eins bedeutet "Lernen" ausschließlich **dauerhaftes Sammeln und
Überprüfen von Feedback**, nicht das Ändern von Modellgewichten oder
Agentenanweisungen während eines Laufs.

Jeder bewertete Lauf soll erfassen:

- Bewertung der Erklärung;
- Bewertung des menschlich ausgeführten Ergebnisses;
- Freitext-Rückmeldung, sofern vorhanden;
- Interaktionskategorie wie `approve`, `revise`, `narrow`, `decline` oder
  `delegate`;
- Aufgabentyp und verknüpfte Ticket- oder Spezifikationskennungen, sofern
  vorhanden.

Die Rückmeldungen sind für das Team sichtbar und werden nach dem gewöhnlichen
Repository-Lebenszyklus aufbewahrt. Version eins enthält keine automatische
Ablauf- oder Löschlogik. Eine regelmäßige Analyse kann Vorlagenänderungen
vorschlagen, die ein Mensch über den üblichen Ticket-, Spezifikations-, Review-
und Commit-Ablauf prüfen und genehmigen muss.

## Vorgeschlagenes Vorlagendesign

Erstelle `.agents/agents/explainer.agent.md` im bestehenden Format für
Agentenvorlagen: YAML-Frontmatter gefolgt von einem begrenzten Rollenvertrag.

### Anforderungen an Frontmatter

- `name`: `Explainer Agent`
- `description`: beschreibt den lesenden, erklärenden und
  menschenkontrollierten Vertrag.
- `argument-hint`: fordert die zu erklärende Aufgabe sowie optional ein
  vorhandenes Ticket oder eine Spezifikation an.
- `user-invocable`: `true`
- `model`: verwendet den Standard aus der kanonischen Routing-Leiter.
- `tools`: enthält ausschließlich nicht mutierende Recherche- und Lesewerkzeuge.

### Erforderliche Vertragsabschnitte

1. **Zweck:** Eine Aufgabe durch Repository-Recherche verständlich machen und
   eine belastbare menschliche Ausführung vorbereiten.
2. **Umfang:** Den Lesemodus der ersten Version sowie die Grenzen zu
   spezialisierten ausführenden Vorlagen benennen.
3. **Recherche vor der Erklärung:** Vollständiges, aber nicht mutierendes Lesen
   erlauben und Fakten von Folgerungen trennen.
4. **Erklärung und Entscheidung:** Die Pflichtfelder der Erklärung und die
   Wirkungen von `approve`, `revise`, `narrow`, `decline` und `delegate`
   definieren.
5. **Ausführungsgrenze:** Festlegen, dass der Mensch ausführt und der Explainer
   Agent weder mutiert noch delegiert.
6. **Bewertung und Nachweise:** Nach der menschlichen Ausführung Bewertungen und
   Feedback im Repository-Feedback-Store erfassen.
7. **Lerngrenze:** Selbständerung ausschließen und nur nachweisgestützte,
   später menschlich geprüfte Empfehlungen erlauben.
8. **Rückgabevertrag:** Eine knappe Zusammenfassung von Erkenntnissen,
   Erklärung, Entscheidung, Bewertungen, Feedback-Referenz und verbleibenden
   Risiken verlangen.

### Vorlagengerüst

```markdown
---
name: "Explainer Agent"
description: "Recherchiert eine begrenzte Aufgabe, erklärt sie nachvollziehbar und erfasst menschliche Bewertungen, ohne selbst auszuführen."
tools: [nur nicht mutierende Recherchewerkzeuge]
argument-hint: "Aufgabe zur Recherche und Erklärung, optional mit Ticket- oder Spezifikationsanker."
user-invocable: true
model: "Claude Sonnet 5"
---

Du bist der Explainer Agent. Untersuche eine Aufgabe im Repository und bereite
eine klare, nachweisgestützte Erklärung für die menschliche Ausführung vor.
Verändere keine Dateien, Stores oder Dienste und delegiere keine Ausführung.

## Recherche und Erklärung

Lies die benötigten Repository-Inhalte. Präsentiere anschließend Ziel,
Erkenntnisse, Einschränkungen, Annahmen, vorgeschlagene menschliche Schritte,
betroffene Entitäten, Validierung, Risiken, Nichtziele und die benötigte
Entscheidung.

## Menschliche Entscheidung

Bitte um `approve`, `revise`, `narrow`, `decline` oder `delegate`. Dokumentiere
die Entscheidung. Der Mensch führt den bestätigten Plan selbst oder über einen
eigenständig gewählten Prozess aus.

## Bewertung und Nachweise

Bitte nach der menschlichen Ausführung um Bewertungen von Erklärung und Ergebnis
sowie optionales Feedback. Speichere den Nachweis mit dem Aufgaben- und
Ticket-/Spezifikationsanker. Ändere die Vorlage, Berechtigungen oder das Routing
nicht auf Grundlage dieses Feedbacks.
```

Das Modell ist der vorläufige Standard der kanonischen Routing-Leiter. Die
endgültige Werkzeugliste darf ausschließlich nicht mutierende Fähigkeiten
enthalten.

## Umsetzungsplan

### Phase 0: Prüfergebnisse übernehmen

1. Die in [00-review-verbesserungen.md](00-review-verbesserungen.md)
   dokumentierten Entscheidungen in Ticket und Spezifikation übernehmen.
2. Den ersten Umfang als lesenden, erklärenden Agenten mit menschlicher
   Ausführung festschreiben.
3. Die teamweite Sichtbarkeit und die Aufbewahrung nach Repository-Lebenszyklus
   festschreiben.
4. Den Pilot-Maßstab vor Beginn des Piloten festschreiben.

**Austrittskriterium:** Ticket und Spezifikation enthalten alle entschiedenen
Vertragsgrenzen ohne verbleibende Produktfrage für Version eins.

### Phase 1: Dauerhaften Produktumfang festlegen

1. Ticket- und Spezifikations-Stores nach überlappender Arbeit zu
Menschenkontrolle, Feedback und Agentenvorlagen durchsuchen.
2. Ticket `79449c3f-2f49-4925-b8fd-3751face53b5` aktualisieren und eine
verknüpfte Spezifikation erstellen.
3. Den Feedback-Ziel-URN, die Akzeptanzkriterien, die Sichtbarkeitsgrenze und
den Validierungsplan dokumentieren.

**Austrittskriterium:** Das Tracking-Ticket ist umsetzungsbereit und die
Spezifikation definiert das vollständige Verhalten von Version eins.

### Phase 2: Agentenvorlage verfassen

1. `.agents/agents/explainer.agent.md` aus dem genehmigten Vorlagendesign
   erstellen.
2. Die engste mögliche, rein lesende Werkzeugfreigabe wählen.
3. Vollständige Recherche vor der Erklärung erlauben und Ausführung durch den
   Menschen verbindlich festlegen.
4. Formate für Erklärung, Entscheidung, Bewertung und Nachweis definieren.
5. Repository-verwaltete Spiegel der Vorlage regenerieren, sofern nötig.

**Austrittskriterium:** Die Vorlage folgt den Frontmatter- und
Rollenvertragskonventionen und besitzt keine Fähigkeit zur Mutation oder
Delegation.

### Phase 3: Feedback und Nachweise anbinden

1. Das kanonische Feedback-Zielformat für Bewertungen eines Agentenlaufs wählen.
2. Erklärungs- und Ergebnisbewertungen getrennt erfassen.
3. Interaktionskategorie, Aufgabentyp und verknüpfte Entitätskennungen erfassen.
4. Eine Abfrage oder einen Bericht bereitstellen, der teamweit sichtbares
Feedback nach Aufgabentyp und Interaktionskategorie gruppiert.

**Austrittskriterium:** Ein abgeschlossener Probelauf lässt sich dauerhaft
auslesen und mit seinem Aufgaben- oder Entitätsanker verbinden.

### Phase 4: Menschliche Kontrolle validieren

1. Testen, dass die Vorlage ausschließlich nicht mutierende Werkzeuge erhält.
2. Testen, dass die Erklärung Fakten, Annahmen, Risiken und menschliche Schritte
voneinander trennt.
3. Szenarien für `approve`, `revise`, `narrow`, `decline` und `delegate`
dokumentieren und prüfen.
4. Testen, dass Feedback gespeichert und über den gewählten Anker ausgelesen
werden kann.
5. Die engsten vorhandenen Vorlagen-, Store- und Integrationsprüfungen ausführen.

**Austrittskriterium:** Jedes Akzeptanzkriterium besitzt einen ausführbaren
Nachweis oder ein dokumentiertes manuelles Validierungsergebnis.

### Phase 5: Pilot und Review

1. Fünf repräsentative Aufgaben mit einem menschlichen Reviewer durchführen.
2. Für jeden Durchlauf Erklärungs- und Ergebnisbewertung erfassen.
3. Wiederkehrende Überarbeitungen, Ablehnungen und Feedback-Themen auswerten.
4. Nur nachweisgestützte Vorlagenänderungen über den üblichen Review-Ablauf
   vorschlagen.

**Austrittskriterium:** Die Mittelwerte für Erklärung und Ergebnis liegen jeweils
bei mindestens 4 von 5. Kein Durchlauf überschreitet die Lese-, Erklärungs- oder
Ausführungsgrenze der Version eins.

## Akzeptanzkriterien

1. Der Explainer Agent kann Repository-Inhalte lesen und erklärt die Aufgabe
   anhand nachweisgestützter Fakten, getrennt von Annahmen und Empfehlungen.
2. Jede Erklärung enthält Ziel, Einschränkungen, menschliche Schritte, betroffene
   Entitäten, Validierung, Risiken, Nichtziele und die benötigte Entscheidung.
3. Der Explainer Agent besitzt und verwendet in Version eins keine
   Mutationswerkzeuge und delegiert keine Ausführung.
4. Ein Mensch kann `approve`, `revise`, `narrow`, `decline` oder `delegate`
   wählen; die jeweilige Entscheidung ist im Lauf nachvollziehbar.
5. Jeder abgeschlossene Lauf speichert Erklärungsfeedback, Ergebnisfeedback und
   die Interaktionskategorie dauerhaft und teamweit sichtbar.
6. Gespeichertes Feedback kann für den Lauf gelesen und bei vorhandenem Anker mit
   einem Ticket oder einer Spezifikation verbunden werden.
7. Der Explainer Agent ändert seine Vorlage, Berechtigungen oder sein Routing
   nicht auf Grundlage von Laufzeit-Feedback.
8. Eine Vorlagenwartungsüberprüfung kann häufige Interaktionsmuster aus
aggregierten Nachweisen erkennen, ohne sie als automatische Richtlinienänderung
zu behandeln.
9. Ein Pilot mit fünf repräsentativen Läufen erreicht für Erklärung und Ergebnis
jeweils einen Mittelwert von mindestens 4 von 5 und enthält keine
Grenzverletzung.

## Validierungsmatrix

| Kriterium | Validierung |
| --- | --- |
| Recherche vor Erklärung | Transcript erfassen und prüfen, dass die Erklärung auf gelesene Repository-Fakten verweist. |
| Vollständige Erklärung | Für einen Probelauf die Pflichtfelder gegen das Erklärungsformat prüfen. |
| Lesemodus | Werkzeugsatz der Vorlage und Transcript prüfen; keine mutierende Tool-Anfrage oder Delegation darf vorkommen. |
| Menschliche Entscheidung | Szenarien für `approve`, `revise`, `narrow`, `decline` und `delegate` ausführen und die gespeicherte Entscheidung prüfen. |
| Menschliche Ausführung | Prüfen, dass die Erklärung die Ausführung dem Menschen zuweist und der Explainer Agent danach keine Aufgabe ausführt. |
| Feedback-Persistenz | Bewertungen und Notizen schreiben und über das Feedback-Ziel wieder auslesen. |
| Anker-Verknüpfung | Prüfen, dass ein Ticket- oder Spezifikationsanker beim Auslesen dieselbe Kennung besitzt. |
| Keine Selbständerung | Vorlagenvertrag und Lauf prüfen; Feedback darf während desselben Laufs keine Vorlagendatei oder Routing-Konfiguration ändern. |
| Musteranalyse | Mehrere Probeläufe abfragen und die Gruppierung nach Interaktionskategorie prüfen. |
| Pilot-Erfolg | Fünf Durchläufe auswerten; beide Bewertungsmittelwerte sowie die Zahl der Grenzverletzungen berechnen. |

## Entschiedene Vertragsgrenzen

| Entscheidung | Festlegung | Bedeutung |
| --- | --- | --- |
| Aufgabenbereich von Version eins | Reine Recherche im Lesemodus | Der Agent erklärt, aber führt nicht aus. |
| Lesen vor der Erklärung | Vollständiges Repository-Lesen | Die Erklärung darf auf geprüften Repository-Fakten beruhen. |
| Ausführung | Menschlich | Der Mensch führt aus oder wählt eigenständig einen Prozess. |
| Erklärungsumfang | Ein begrenzter Plan | Die Erklärung definiert Ziel, Nachweise, Risiken und menschliche Schritte. |
| Spätere Abweichungen | Neues Ziel oder geändertes Ergebnis | Eine künftige ausführende Version benötigt erneute Zustimmung. |
| Sichtbarkeit von Feedback | Teamweit sichtbar | Bewertungen und Freitext sind für das Team zugänglich. |
| Aufbewahrung | Repository-Lebenszyklus | Version eins implementiert keine automatische Löschung. |
| Lernen | Feedback-Analyse | Nur spätere menschlich geprüfte Änderungen sind zulässig. |
| Pilot-Erfolg | Fünf Läufe, mindestens 4 von 5 | Beide Bewertungsmittelwerte und die Kontrollgrenzen müssen erfüllt sein. |

## Review-Zusammenfassung

Die Quelle etabliert ein wertvolles Interaktionsmodell: Erklärung ist ein
eigenständiges Produktmerkmal, nicht bloß eine Einleitung zur Ausführung. Der
entscheidende Grundsatz für Version eins lautet: Der Explainer Agent darf das
Repository gründlich lesen und seine Erkenntnisse verständlich erklären, doch
die Ausführung verbleibt beim Menschen. Feedback erzeugt Nachweise für einen
späteren, menschlich geprüften Verbesserungszyklus und verleiht dem Agenten
keine Autorität zur Selbständerung oder Ausführung.
