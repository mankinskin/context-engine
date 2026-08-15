<<<<<<< HEAD
# Explainer Agent: Review and Implementation Plan

## Source and Status

This plan derives from [input.clean.md](input.clean.md). The source describes a
human-in-the-loop agent that explains a task before performing the task, lets a
human evaluate the result, and uses the feedback to improve future explanations
and identify frequent interactions.

**Review verdict: not implementation-ready yet.** The product direction is
clear, but the source does not define the execution boundary, the meaning of
"learn", the feedback store, or measurable success criteria. The decisions in
the [Open Decisions](#open-decisions) section must be resolved before creating
an implementation ticket or specification.

## Confirmed Requirements

The Explainer Agent must:

1. Explain the intended task and approach before performing any mutating work.
2. Keep a human in the control loop, with an explicit opportunity to approve,
   reject, or change the proposed approach.
3. Let the human directly evaluate the quality of both the system behavior and
   the explanation.
4. Capture feedback so later Explainer Agent runs can improve the explanation
   quality and reveal commonly used controls or interaction patterns.
5. Preserve the human's authority to stop work before an action is executed.

## Scope and Non-Goals

### In Scope

- A new agent template at `.agents/agents/explainer.agent.md`.
- A repeatable explain -> approve -> execute -> evaluate workflow.
- Durable feedback capture linked to the task, ticket, or specification when an
  entity exists.
- A reviewable summary of what was proposed, approved, executed, and evaluated.
- Aggregated interaction evidence that can inform future template revisions.

### Out of Scope for the First Version

- Training or fine-tuning a foundation model from user feedback.
- Silent execution after an explanation without an explicit approval.
- Autonomous changes to the Explainer Agent's permissions, instructions, or
  model routing.
- Inferring sensitive user preferences from unrelated conversation data.
- Replacing the specialized Implement, Interview, Review, or Testing Agent
  templates.

## Product Contract

### Workflow

1. **Receive a task.** The Explainer Agent establishes the goal, relevant
   repository entities, and allowed scope.
2. **Explain.** Before any mutation, the Explainer Agent presents a concise
   explanation containing the goal, proposed steps, files or entities expected
   to change, risks, validation, and assumptions.
3. **Obtain a human decision.** The human chooses one of the following:
   `approve`, `revise`, `narrow`, `decline`, or `delegate`.
4. **Execute only after approval.** The Explainer Agent performs the approved
   scope, announces material deviations, and returns to approval when a change
   would exceed the approved plan.
5. **Evaluate.** The human rates the explanation and result, and can submit a
   concise free-text finding.
6. **Record evidence.** The Explainer Agent records the approved plan, execution
   evidence, feedback, and interaction category in the repository's durable
   stores.
7. **Improve future explanations.** A later template-maintenance process uses
   aggregated feedback to update instructions. A run must never rewrite its own
   template or behavior automatically.

### Explanation Requirements

Every pre-execution explanation must state:

- the requested outcome in plain language;
- known constraints and assumptions;
- the proposed sequence of actions;
- the intended files, tickets, specifications, or services;
- the validation method and the expected evidence;
- risks, non-goals, and the human decision required before execution.

The explanation must distinguish facts found in the repository from inferences
and recommendations. The explanation must not claim that a command, test, or
change succeeded before evidence confirms success.

### Human Control Requirements

- No mutating tool call may run before an explicit `approve` decision covering
  the described scope.
- A `revise` or `narrow` decision produces a new explanation and requires a new
  approval.
- A `decline` decision ends the execution path without mutation.
- A material deviation from the approved plan pauses execution and asks for a
  new decision.
- The recorded evaluation must identify the specific run and, where applicable,
  its ticket or specification anchor.

### Learning Requirements

For version one, "learn" means **collecting and reviewing durable feedback**,
not updating model weights or modifying agent instructions during a run.

Each evaluated run should capture:

- explanation rating;
- execution-result rating;
- free-text feedback, when supplied;
- interaction category, such as `approve`, `revise`, `narrow`, `decline`, or
  `delegate`;
- task type and the linked ticket/specification identifiers, when available.

Periodic analysis may propose template changes from the recorded evidence. A
human must review and approve every such template change through the ordinary
ticket, specification, review, and commit workflow.

## Proposed Template Design

Create `.agents/agents/explainer.agent.md` using the repository's existing agent
template format: YAML frontmatter followed by a bounded role contract.

### Frontmatter Requirements

- `name`: `Explainer Agent`
- `description`: states the explain-before-execute and human-approval contract.
- `argument-hint`: requests the task plus an optional existing ticket or spec.
- `user-invocable`: `true`
- `model`: select a model from the canonical routing ladder after the execution
  scope is decided.
- `tools`: include only the tools needed for the approved execution model. The
  final list must not grant mutation tools until the approval gate has been
  designed and tested.

### Required Contract Sections

1. **Purpose**: turn a task into an understandable, human-approved execution
   unit.
2. **Scope**: state which task classes the template may execute and which tasks
   must be delegated to existing specialized agents.
3. **Pre-execution explanation**: require the explanation fields listed above.
4. **Approval gate**: require a `vscode/askQuestions` decision before every
   mutation; list the exact legal decisions and their effects.
5. **Execution boundary**: prohibit scope expansion and require renewed
   approval for material deviations.
6. **Evaluation and evidence**: require a post-execution rating and feedback
   capture through the repository feedback store.
7. **Learning boundary**: prohibit autonomous self-modification; permit only
   evidence-backed recommendations for a later human-reviewed template update.
8. **Return contract**: require a concise result containing the approved plan,
   execution evidence, evaluation, recorded feedback reference, and remaining
   risks.

### Template Skeleton
=======
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
>>>>>>> 77cb9b1e (feat(session-management): add session and transcript JSON files for explainer agent)

```markdown
---
name: "Explainer Agent"
<<<<<<< HEAD
description: "Explain a bounded task, obtain human approval, execute only the approved scope, and record evaluation evidence."
tools: [TBD after approval-gate design]
argument-hint: "Task to explain and execute, with an optional ticket or spec anchor."
=======
description: "Recherchiert eine begrenzte Aufgabe, erklärt sie nachvollziehbar und erfasst menschliche Bewertungen, ohne selbst auszuführen."
tools: [nur nicht mutierende Recherchewerkzeuge]
argument-hint: "Aufgabe zur Recherche und Erklärung, optional mit Ticket- oder Spezifikationsanker."
>>>>>>> 77cb9b1e (feat(session-management): add session and transcript JSON files for explainer agent)
user-invocable: true
model: "Claude Sonnet 5"
---

<<<<<<< HEAD
You are the Explainer Agent. Turn a task into a clear, human-approved execution
unit. Never perform a mutating action until the human has explicitly approved
the presented scope.

## Pre-execution Explanation

Present the outcome, constraints, assumptions, proposed actions, affected
entities, validation, risks, non-goals, and the specific approval requested.

## Approval Gate

Ask the human to approve, revise, narrow, decline, or delegate. Record the
decision. Only `approve` permits execution within the stated boundary.

## Execution and Re-approval

Execute only the approved scope. Stop and request new approval before a
material deviation or an additional mutation.

## Evaluation and Evidence

Ask the human to rate the explanation and execution result. Record feedback
with the run's task and ticket/spec anchor. Do not change this template or its
permissions from feedback during a run.
```

The model value above is a provisional default. The final model and tools must
follow the canonical model-routing and tool-grant rules once the execution
boundary is decided.

## Implementation Plan

### Phase 0: Resolve Product Decisions

1. Define the first-version task class: read-only research, a narrow code edit,
   or delegated execution through existing agents.
2. Define whether one approval covers one command, one plan, or one ticket
   slice.
3. Define the allowed feedback audience and retention policy.
4. Define success thresholds for explanation quality and human control.

**Exit criterion:** every entry in [Open Decisions](#open-decisions) has an
owner and a recorded answer, or is explicitly deferred from version one.

### Phase 1: Establish Durable Product Scope

1. Search existing ticket and specification stores for overlapping human-loop,
   feedback, agent-template, and execution-gating work.
2. Create or update one tracking ticket and a linked specification.
3. Record the approved contract, acceptance criteria, privacy boundary, and
   validation plan.

**Exit criterion:** the tracking ticket is implementation-ready and the
specification defines the complete version-one behavior.

### Phase 2: Author the Agent Template

1. Add `.agents/agents/explainer.agent.md` from the approved template design.
2. Select the narrowest tool grant that can implement the approved task class.
3. Include an explicit approval gate before every mutation.
4. Define the expected result and evidence record formats.
5. Regenerate repository-managed agent artifacts if the template is mirrored.

**Exit criterion:** the template follows the repository's frontmatter and role
contract conventions and has no capability beyond the approved scope.

### Phase 3: Add Feedback and Evidence Wiring

1. Select the canonical feedback target format for agent-run evaluations.
2. Record explanation and execution ratings separately.
3. Capture the interaction category and linked entity identifiers.
4. Provide a query or report that groups feedback by task type and interaction
   category without exposing sensitive content unnecessarily.

**Exit criterion:** a completed trial run can be read back from durable storage
and connected to its task or entity anchor.

### Phase 4: Validate Human Control

1. Test that a declined task produces no mutation.
2. Test that `revise` and `narrow` require a new explanation and approval.
3. Test that an unplanned mutation pauses for re-approval.
4. Test that feedback is recorded and can be read back.
5. Run the narrowest existing agent-template, store, and integration checks.

**Exit criterion:** every acceptance criterion below has executable evidence or
a documented manual validation result.

### Phase 5: Pilot and Review

1. Run a small set of representative tasks with a human reviewer.
2. Collect explanation and result evaluations for every pilot run.
3. Review recurring revisions, declines, and feedback themes.
4. Propose only evidence-backed changes to the template through the normal
   review workflow.

**Exit criterion:** the human reviewer approves the pilot outcome and accepts
the template's boundaries.

## Acceptance Criteria

1. The Explainer Agent presents a complete pre-execution explanation before any
   mutating action.
2. A human can approve, revise, narrow, decline, or delegate the proposed task.
3. No mutation occurs after a decline or before approval.
4. A material execution deviation blocks further mutation until renewed human
   approval is recorded.
5. Every completed run records explanation feedback, execution feedback, and
   the interaction category in durable storage.
6. Recorded feedback can be retrieved for the run and linked to a ticket or
   specification when an anchor exists.
7. The Explainer Agent does not modify its own template, permissions, or routing
   based on runtime feedback.
8. A template-maintenance review can identify frequent interaction patterns
   from aggregated evidence without treating those patterns as automatic policy
   changes.

## Validation Matrix

| Criterion | Validation |
| --- | --- |
| Explain before mutation | Capture the transcript and assert the approval question precedes the first mutating tool call. |
| Approval boundary | Run approve, revise, narrow, decline, and delegate scenarios. |
| Re-approval | Introduce a planned scope change and assert that execution pauses. |
| Feedback persistence | Write ratings and notes, then read the stored feedback back by target. |
| Anchor linkage | Verify a ticket/specification-linked run resolves the same identifiers on read-back. |
| No self-modification | Review the template contract and test that feedback cannot edit the template in the same run. |
| Pattern analysis | Query multiple trial records and confirm the report groups interaction categories accurately. |

## Open Decisions

| Decision | Options | Recommended default | Why the decision matters |
| --- | --- | --- | --- |
| Version-one task scope | Read-only; narrow direct edits; delegated execution | Delegated execution | Reuses specialized agents and limits new permissions. |
| Approval granularity | Per command; per bounded plan; per ticket | Per bounded plan | Keeps interaction usable while retaining a clear scope boundary. |
| Definition of material deviation | New file; new entity; new command class; changed outcome | Any new mutation target or changed outcome | Produces an objective re-approval rule. |
| Meaning of learning | Runtime self-modification; feedback analysis; model training | Feedback analysis only | Allows improvement without uncontrolled behavior changes. |
| Feedback visibility | Private to run; team-visible; anonymized aggregate | Private run feedback plus approved aggregates | Balances traceability with privacy. |
| Pilot success threshold | Qualitative review; rating threshold; task-completion rate | Set before pilot | Prevents interpreting feedback after the fact. |

## Review Summary

The transcript establishes a valuable interaction model: explanation is part of
the product, not merely narration before execution. The critical design rule is
that feedback produces evidence for a later, human-reviewed improvement cycle;
feedback must not grant the Explainer Agent authority to change its own behavior
or execute beyond the approved plan.
=======
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
>>>>>>> 77cb9b1e (feat(session-management): add session and transcript JSON files for explainer agent)
