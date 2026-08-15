# Explainer Agent: Geprüfte Verbesserungen

## Grundlage

Dieses Dokument hält die Entscheidungen aus der Review-Sitzung fest. Die
Entscheidungen ersetzen die im ursprünglichen Plan noch offenen Produktfragen.

## Verbindliche Entscheidungen

| Bereich | Entscheidung | Auswirkung auf den Plan |
| --- | --- | --- |
| Aufgabenbereich der ersten Version | Reine Recherche im Lesemodus | Der Explainer Agent verändert keine Dateien, Stores oder Dienste. |
| Vorab-Recherche | Vollständiges Lesen des Repositorys ist erlaubt | Der Explainer Agent darf Fakten sammeln, bevor er eine Erklärung erstellt. |
| Ausführung | Der Mensch führt die Aufgabe aus | Der Explainer Agent führt keine Aufgabe aus und delegiert keine Ausführung. |
| Erklärungsumfang | Eine begrenzte Recherche- bzw. Ausführungsplanung | Die Erklärung benennt Ziel, Annahmen, Quellen, geplante Schritte, Risiken und erwartete Nachweise. |
| Abweichungen | Neues Ziel oder geändertes Ergebnis | Eine spätere ausführende Version benötigt vor der Erweiterung eine erneute Zustimmung. |
| Feedback-Sichtbarkeit | Teamweit sichtbar | Der Plan muss deutlich machen, dass Bewertungen und Freitext für das Team sichtbar sind. |
| Feedback-Aufbewahrung | Gewöhnlicher Repository-Lebenszyklus | Für Version eins gibt es keine automatische Ablauf- oder Löschlogik. |
| Lernen | Nur Feedback-Analyse | Laufzeit-Feedback darf keine Vorlage, Berechtigung, Modellwahl oder Richtlinie ändern. |
| Pilot-Erfolg | Fünf repräsentative Durchläufe | Mittelwerte für Erklärung und Ergebnis betragen jeweils mindestens 4 von 5; es gibt keine Grenzverletzung. |

## Erforderliche Textänderungen

1. Den Status von "nicht umsetzungsbereit" auf einen geprüften, aber noch zu
   spezifizierenden Version-eins-Vertrag ändern.
2. Den Ablauf von "erklären, genehmigen, ausführen, bewerten" auf "lesen,
   erklären, menschlich ausführen, bewerten" umstellen.
3. Alle Aussagen entfernen, nach denen der Explainer Agent nach Zustimmung
   selbst mutiert oder an spezialisierte Agenten delegiert.
4. Die Liste offener Entscheidungen durch eine Tabelle der entschiedenen
   Vertragsgrenzen ersetzen.
5. Die Frontmatter- und Vertragsvorgaben für eine reine Lesevorlage ohne
   Mutationswerkzeuge formulieren.
6. Die Akzeptanzkriterien und die Validierung auf nachvollziehbare Erklärungen,
   menschliche Ausführung, Feedback-Lesbarkeit und den Pilot-Maßstab ausrichten.
7. Die gesamte überarbeitete Fassung ins Deutsche übersetzen.

## Verbleibende Folgeschritte

Ticket `79449c3f-2f49-4925-b8fd-3751face53b5` benötigt weiterhin eine verknüpfte
Spezifikation, die das konkrete Feedback-Ziel, die Abfrage für aggregierte
Muster und die ausführbaren Validierungen festlegt. Diese Folgeschritte ändern
nicht die hier festgehaltenen Produktentscheidungen.