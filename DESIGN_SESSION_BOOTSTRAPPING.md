## Architektur-Spezifikation: Dynamic Session Bootstrapping & Context Routing## 1. Die Motivation: Das "Metadata Paradox" der Agenten-Entwicklung
In hochgradig strukturierten, agentischen Workflows (wie mit der memory-api) führt ein hohes Maß an Dokumentation paradoxerweise zu einem Systemkollaps bei der KI-Modell-Inferenz. Wenn Regeln (rule-tool), Spezifikationen (spec-tool), Tickets (ticket-tool) und Indizes wachsen, kollabiert die Core-Reasoning-Kapazität des Agenten unter der Last seiner eigenen Meta-Vorgaben.
## Die Kernprobleme des aktuellen Zustands:

* Kontext-Verwässerung (Loss-in-the-Middle): Wenn ein Agent bei jedem Turn 10.000 Token an statischen Instruktionen (.agents/instructions/*.md) mitschleppt, verliert die Self-Attention des LLMs den Fokus auf den eigentlichen Code und die akute Aufgabe.
* Zustands-Amnesie (Relearning Cost): Der Agent muss sich zu Beginn jeder neuen Sitzung mühsam zusammensuchen, woran er zuletzt gearbeitet hat. Das verbrennt massenhaft Token für "Mental Checking".
* Mangelnde Agency (Reaktives Verhalten): Der Agent agiert wie ein Fließbandarbeiter, dem alle Handbücher gleichzeitig auf den Tisch geworfen werden, anstatt wie ein autonomer Architekt, der gezielt nach der passenden Information greift.
* Das "Australien-Wetter"-Problem: Ein System-Prompt darf kein implizites Wissen erzwingen. Wenn man den Agenten nach dem Wetter in Australien fragt, darf er keine Token für das Laden von Ticketing-Regeln verschwenden.

------------------------------
## 2. Der Designansatz: "Just-In-Time Metakognition"
Die Lösung ist die strikte Trennung von Metakognition (Wie verwalte ich mein Wissen?) und Fachexpertise (Welches Ticket/Code-Segment bearbeite ich?). Anstatt den Agenten mit Kontext zu füttern, wird er zum Kurator seines eigenen Kontextes.
Das System wird um eine aktive Laufzeitkomponente erweitert: Das session-tool (als Ausbau der bestehenden, archivierenden session-api). Es fungiert als flüchtiger, dateibasierter "Cognitive Workspace" (RAM des Agenten).

[Benutzer-Prompt]
        │
        ▼
[Bootstrapper Prompt] (Minimal, <500 Token. Kennt nur Such- & Session-Tools)
        │
        ├── Modus-Erkennung (Code-Task vs. Allgemeines Plaudern)
        ▼
   session_init() ──> Initialisiert session_context.json
        │
        ├── Autonome Suche via Tantivy (`rule_search`, `spec_search`)
        ▼
   session_pin()  ──> Lädt Volltext NUR der relevanten Entitäten in den Prompt
        │
        ▼
[Eigentlicher Task-Turn] ──> Minimaler, hochfokussierter Kontext

------------------------------
## 3. Technische Implementierung: Die Kernkomponenten## A. Das Session-Schema (session_context.json)
Die Session speichert nicht mehr nur die Historie (wie im bestehenden Archiv-Ansatz), sondern den aktiven Aufmerksamkeits-Zustand über Datei- und Ticketgrenzen hinweg:

{
  "session_id": "823b22cf-c0dc-46c6-a03d-00cdd3c4c83a",
  "current_mode": "engineering",
  "pinned_entities": {
    "tickets": [
      { "id": "ticket-101", "relation": "primary_focus" },
      { "id": "ticket-94", "relation": "blocked_by" }
    ],
    "specs": [
      { "id": "spec-session-api", "section": "read_path" }
    ],
    "rules": [
      { "id": "quality-gate-playwright", "reason": "Frontend-Änderung erkannt" }
    ]
  }
}

## B. Die neuen MCP-Schnittstellen (Interface Layer)

   1. session_init(ticket_id: Option<String>) -> SessionState: Prüft, ob bereits eine aktive Sitzung existiert. Wenn eine ticket_id übergeben wird, führt die Core-Bibliothek im Hintergrund eine Kaskaden-Suche via Tantivy aus, um direkt verknüpfte Specs und Rules zu identifizieren und als "Vorschlag" zu pinnen.
   2. session_pin(entity_type: String, entity_id: String): Der Agent fügt dem Kontext eine Entität (z.B. eine spezifische Test-Regel) hinzu. Erst jetzt wird deren Volltext in das aktive Prompt-Fenster injiziert.
   3. session_unpin(entity_type: String, entity_id: String): Der Agent entfernt die Entität. Ermöglicht sauberes Multi-Ticket-Arbeiten (Wechsel von Ticket A zu Ticket B und zurück, ohne Context-Bloat).

------------------------------
## 4. Offene Entscheidungspunkte (Architectural Decisions)
Bevor du den Code in Rust umsetzt, müssen drei strategische Weichen gestellt werden:
## Entscheidungspunkt 1: Wer führt das Context-Rendering aus?

* Option A (Client-Side Rendering): Das MCP-Tool session_view gibt die strukturierten Volltexte zurück, und der Client (z.B. das Agenten-Framework/Cursor/Claude Desktop) klebt sie in den Prompt.
* Option B (Server-Side Injection): Die memory-api modifiziert die Instruktions-Dateien im .agents/instructions/-Ordner dynamisch vor jedem Turn im Hintergrund.
* Empfehlung: Option A. Es hält die Dateiablage sauberer und verhindert Race-Conditions, wenn mehrere Agenten parallel auf dem Dateisystem operieren.

## Entscheidungspunkt 2: Flüchtiger RAM vs. Aggressives File-I/O

* Option A (Reines File-Backed): Jeder session_pin-Aufruf schreibt sofort die session_context.json auf die Platte. Sicher bei Abstürzen, aber langsamer.
* Option B (In-Memory mit Lazy-Persistence): Die Session lebt im RAM des MCP-Servers und wird nur über SessionStorePlan::persist() bei explizitem Aufruf oder am Ende eines Tasks weggeschrieben.
* Empfehlung: Option A, da MCP-Server per Definition zustandslos/flüchtig sein können (z.B. bei Timeouts oder Neustarts des Editors). Da es sich um kleine JSON-Dateien handelt, ist der I/O-Overhead vernachlässigbar.

## Entscheidungspunkt 3: Die Such-Kaskade (Automatisches Vorschlagen)

* Wie aggressiv soll session_init im Hintergrund via Tantivy nach Regeln suchen? Wenn der Nutzer schreibt: "Fix den Button", findet die Suche vielleicht 50 Regeln zu "Button".
* Lösung: Die Core-Bibliothek sollte bei session_init nur Entitäten pinnen, die eine explizite, harte ID-Verknüpfung im Ticket-Metadatensatz haben. Alles andere (semantische Suche nach vagen Regeln) muss der Agent im ersten Turn via rule_search explizit selbst tun und autonom pinnen.

------------------------------
## Fazit für den Entwickler
Um dieses System zu bauen, musst du die bereits in Rust validierte SessionStore-Logik von einem reinen Schreib/Archiv-Pfad zu einem Lese/Laufzeit-Pfad erweitern. Sobald die MCP-Endpoints stehen, wird das System radikal billiger im Tokenverbrauch, extrem schnell in der Auffassungsgabe und verhält sich endlich wie ein menschlicher Senior-Entwickler: Erst das Problem analysieren, dann das passende Handbuch aufschlagen, dann den Code anfassen.