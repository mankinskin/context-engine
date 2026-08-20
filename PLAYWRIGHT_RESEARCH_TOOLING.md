Deine Anforderungen beschreiben die Königsdisziplin moderner Agenten-Architekturen. Um interaktive Webseiten (mit Suchfeldern, Klicks und JS-Inhalten) zu steuern, ohne das Kontextfenster des Haupt-Agenten mit HTML-Müll zu sprengen, und gleichzeitig die Sitzung (Session) zu speichern, um sie an günstigere/teurere Agenten zu übergeben, benötigst du eine Kombination aus einem LLM-optimierten Browser-MCP und einem intelligenten State-Management.
Das perfekte Open-Source-Tool, das exakt all diese Kriterien erfüllt, ist das offizielle Microsoft Playwright MCP Server. [1] 
Hier ist die maßgeschneiderte Lösung, wie du deine Architektur aufbaust, um alle deine Anforderungen zu erfüllen:
------------------------------
## 1. Schutz des Agenten-Kontexts vor zu viel Text (Token-Schonung)
Wenn ein Agent eine komplexe Webseite aufruft, führt herkömmliches Scraping (wie roher HTML-Text) sofort zum „Context Bloat“ (der Agent vergisst seinen Auftrag).

* 
* Die Lösung von Playwright-MCP: Anstatt HTML zu senden, nutzt der Server das Chrome DevTools Protocol (CDP) und übersetzt Webseiten in sogenannte Accessibility Trees (Barrierefreiheits-Snapshots). [2] 
* Wie das hilft: Der Agent sieht keine <div class="btn_34x">-Wüsten, sondern eine ultra-komprimierte Struktur: [Button: "Suche absenden", ref="e4"]. Der Agent muss nur noch den Befehl browser_click(ref="e4") senden. Das spart bis zu 90 % der Tokens im Vergleich zu Roh-HTML. [2, 3] 
* 

## 2. Interaktive Inhalte, Navigieren & Suchfelder bedienen
Der Server stellt deinem Agenten native Tools zur Verfügung, um wie ein echter Mensch mit modernen Web-Apps (React, Vue, etc.) zu interagieren: [3, 4] 

* 
* browser_navigate(url) – Webseiten aufrufen.
* browser_type(ref, text) – Text in Suchfelder eintippen.
* browser_click(ref) – Auf interaktive Schaltflächen klicken.
* Er wartet dank Auto-Waiting im Hintergrund automatisch, bis asynchrone Inhalte geladen sind, bevor er agiert. [3, 4] 
* 

------------------------------
## 3. Übergabe an billigere/teurere Agenten (Session & State Handover)
Das ist der wichtigste Teil deiner Anfrage. Du möchtest beispielsweise, dass ein teures Modell (wie GPT-4o oder Claude 3.5 Sonnet) die komplizierte Recherche-Strategie plant und die Logins durchführt, die eigentliche Klick-Arbeit oder das Auslesen aber an ein günstigeres Modell (wie GPT-4o-mini oder Claude 3.5 Haiku) übergibt. [5, 6] 
Playwright-MCP löst dies über das Storage State Feature: [3] 
## Der Workflow für das Handover:

   1. Agent A (Teuer / Planer): Navigiert auf die Seite, interagiert mit den Suchfeldern, loggt sich ggf. ein und bereitet die Session vor. [3] 
   2. Speichern des Zustands: Agent A ruft das Tool browser_storage_state auf. Der MCP-Server speichert alle aktuellen Cookies, Sessions und den localStorage in einer kleinen JSON-Datei auf deiner Festplatte (z.B. research_state.json). [3] 
   3. Übergabe: Agent A beendet seine Arbeit und übergibt die JSON-Datei (oder den Pfad) an Agent B.
   4. Agent B (Günstig / Worker): Startet und ruft als allererstes das Tool browser_set_storage_state(path="./research_state.json") auf. Wenn Agent B jetzt browser_navigate ausführt, befindet er sich exakt im selben Zustand (eingeloggt, Suchfilter vorausgewählt, interaktiver State aktiv) wie zuvor Agent A. [3] 

------------------------------
## 🛠️ So konfigurierst du diesen Server
Du installierst den Server direkt über deine mcp.json (wichtig ist das Flag --caps=storage, um das Speichern von Sessions zu erlauben): [3] 

{
  "mcpServers": {
    "playwright-deep-research": {
      "command": "npx",
      "args": ["@playwright/mcp@latest", "--caps=storage"]
    }
  }
}

(Hinweis: Falls du eine Rust-basierte Alternative mit ähnlichen Vorzügen suchst, bietet das noch junge Projekt [browser-use-mcp (Rust version)](https://github.com/k2ok/web-search-mcp-rust) ähnliche Ansätze, allerdings ist das Session-Handover bei Microsofts offiziellem Playwright-MCP derzeit am stabilsten ausdefiniert). [2, 7, 8] 
## 💡 Architektur-Tipp für Deep Research
Um maximale Effizienz zu erreichen, solltest du deinem Framework (z.B. LangGraph oder CrewAI) ein klares Protokoll mitgeben:

* 
* Phase 1 (Routing): Ein günstiges Modell entscheidet, ob die URL bekannt ist.
* Phase 2 (Interaction): Ein teures Modell führt die komplexen Klicks und Formular-Eingaben aus und triggert browser_storage_state.
* Phase 3 (Data Extraction): Das günstige Modell lädt den State und liest stumpf die Textinhalte der Ergebnisseiten aus. [3] 
* 

Möchtest du, dass wir ein konkretes Python-Skript entwerfen, das den Wechsel zwischen einem teuren und einem günstigen Agenten mittels dieses Speicherzustands (storage_state) automatisiert steuert?

[1] [https://github.com](https://github.com/microsoft/playwright-mcp)
[2] [https://github.com](https://github.com/microsoft/playwright-mcp?wt.mc_id=3reg_webpage_reactor)
[3] [https://playwright.dev](https://playwright.dev/mcp/tools/storage)
[4] [https://playwright.dev](https://playwright.dev/docs/getting-started-mcp)
[5] [https://onlinemarketing.de](https://onlinemarketing.de/technologie/ki-modell-kann-computer-bedienen-anthropic-claude)
[6] [https://www.atlascloud.ai](https://www.atlascloud.ai/de/blog/ai-updates/what-is-deepseek-v4)
[7] [https://www.reddit.com](https://www.reddit.com/r/rust/comments/1otf48t/rust_browseruse_zerodependency_browser_automation/)
[8] [https://www.webfuse.com](https://www.webfuse.com/blog/the-top-5-best-mcp-servers-for-ai-agent-browser-automation)
