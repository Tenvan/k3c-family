# RU65 · DEV · Rust-Backend für Workbench und Studio: Spike (Tauri) und Entscheidung 014

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-25

## Ausgangslage

Entwurf (2026-11-25, Ralf: Plan für einen schrittweisen Umstieg von Workbench und Studio auf Rust, bei Bedarf abzuarbeiten). Beide Apps sind Wails (Go-Backend, React/TypeScript-Frontend, `tools/webkit`). Nach RU41 entfällt die Go-Engine (rund 59.000 Zeilen samt Tests); übrig bleiben Workbench rund 19.000, Studio rund 19.500, workkit 2.900, assetkit 4.000 Zeilen Go ohne Tests (Messung 2026-11-25).

## Ziel

Ein Spike zeigt, ob Tauri 2 (React-Frontend bleibt, Backend Rust) Wails ersetzen kann: Fenster, Commands/Events, MCP-Server über ein Rust-SDK (z. B. `rmcp`), Dev-Betrieb mit Hot Reload. Ergebnis ist die Entscheidung 014 (Annahme nur durch Ralf) mit Aufwand, Risiken und Reihenfolge der Folge-Sprints RU66 bis RU71. Fällt sie gegen Rust aus, werden RU66 bis RU71 verworfen.

## Beteiligte und Zielgruppen

Ralf entscheidet (Entscheidung 014); ein Agent baut den Spike und schreibt die Entscheidungsvorlage; die Review-Session prüft.

## Anforderungen

Spike in `.work/spike-tauri/` (nicht eingecheckt): Tauri 2 mit dem Frontend der Workbench (React, `tools/webkit`) und einem Rust-Backend, das drei Dinge zeigt: (1) ein Command und ein Event zwischen Frontend und Backend, (2) ein MCP-Server (Streamable HTTP) mit einem Beispiel-Tool über ein Rust-SDK, erreichbar für Claude Code, (3) Dev-Betrieb mit Hot Reload und Neustart bei Rust-Änderung. Gemessen werden Bauzeit kalt und warm, Größe der EXE, Speicher im Leerlauf. Die Entscheidungsvorlage `docs/decisions/014-rust-backend-fuer-werkzeuge.md` nennt Aufwand je Folge-Sprint, Risiken (WebView2, Windows-Fenster, Prozesse und Job-Objekte, Pipes) und die Alternative “Go bleibt”.

## Nicht-Ziele

Kein Port von Produktivcode (RU66 bis RU70). Keine Änderung am Frontend. Keine neue Abhängigkeit im Repo (der Spike liegt in `.work/`).

## Regeln und Einschränkungen

Entscheidung 014 nimmt nur 🧑 an; ohne Annahme starten RU66 bis RU71 nicht. Neue Crates und Abhängigkeiten nur nach Freigabe 🧑. Schreiben nur im Repo (`.work/`).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `docs/decisions/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Sperre:** –

## Beispiele

Der Spike startet ein Fenster mit dem Workbench-Frontend; ein Klick ruft ein Rust-Command, `claude mcp` verbindet sich mit dem Beispiel-Tool und bekommt eine Antwort.

## Ausnahme- und Fehlerfälle

Scheitert eine der drei Aufgaben (z. B. MCP-SDK reicht nicht für Streamable HTTP), steht das in der Vorlage als Ausschlussgrund; dann Empfehlung “Go bleibt” und RU66 bis RU71 werden verworfen.

## Akzeptanzkriterien

- **AC-01** Der Spike zeigt Command, Event, MCP-Tool und Hot Reload und liefert die genannten Messwerte (Datei `.work/spike-tauri/ERGEBNIS.md`, Zahlen in der Vorlage).
- **AC-02** `docs/decisions/014-rust-backend-fuer-werkzeuge.md` liegt mit Empfehlung, Aufwand je Folge-Sprint und Risiken vor.
- **AC-03** 🧑 hat die Entscheidung angenommen oder abgelehnt (steht in der Datei).

## Offene Fragen

Entscheidung 014 (🧑): Tauri 2 oder Wails/Go behalten. Mit Default entschieden (autonom, Claude, 2026-11-25; 🧑 kann widersprechen): Der Spike nutzt das Frontend der Workbench, nicht des Studios.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Stichpunkte (Reife Entwurf): 1. Spike Tauri (AC-01). 2. Entscheidungsvorlage 014 (AC-02). 3. Review und Annahme durch 🧑 (AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
