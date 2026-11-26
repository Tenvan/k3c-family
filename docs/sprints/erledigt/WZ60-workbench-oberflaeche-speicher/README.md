# WZ60 · DEV · Workbench-Oberfläche bleibt bei langen Läufen stabil

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-895, B-909
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 (B-895)

## Ausgangslage

Der WebView2-Renderer der Workbench-EXE wächst bei laufenden Agenten auf über 1 GiB, bis das Fenster leer bleibt (B-895). Die Konsolenzeilen sind seit 8099fe57e gebündelt.

## Ziel

Die Workbench-Oberfläche bleibt bei stundenlangen parallelen Agenten-Läufen unter einer festen Speichergrenze und bedienbar.

Am Ende sichtbar: `workbench_resources` zeigt den Renderer unter 500 MiB; darüber steht ein 🤒-Eintrag im Log.

## Beteiligte und Zielgruppen

Ralf (🧑) nutzt die Workbench beim Entwickeln; Agenten rufen den MCP-Server auf.

## Anforderungen

`B-895 › Anforderungen`.

## Nicht-Ziele

Umbau des Wails-Transports (Ereignisse und RPC über ExecuteScript bleiben).

## Regeln und Einschränkungen

Oberflächen nach Atomic Design über `@webkit`.

- **Bereich:** `tools/k3c-workbench/frontend/src/**`, `tools/k3c-workbench/internal/resmon/**`, `tools/k3c-workbench/internal/mcpsrv/resources.go`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme (AC-02, Messung nach 1 h mit der neuen EXE)
- **Sperre:** exe

## Beispiele

Drei Agenten rufen 1 h lang plan_* und check_run auf, die Oberfläche steht auf MCP → Renderer bleibt unter 500 MiB.

## Ausnahme- und Fehlerfälle

Renderer über 500 MiB → 🤒-Warnung im Log, einmal je Schwelle.

## Akzeptanzkriterien

- **AC-01** Ein Test simuliert 10 000 Live-Ereignisse und prüft, dass die Puffer begrenzt bleiben (`B-895/AC-01`).
- **AC-02** Nach 1 h mit zwei parallelen check_run-Schleifen bleibt der Renderer unter 500 MiB (`B-895/AC-02`).
- **AC-03** Überschreitet der Renderer die Grenze, erscheint ein 🤒-Eintrag im Log (`B-895/AC-03`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ60.1 | `WZ60.1-webview-speicher.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-26, mit WZG geschlossen (🧑 „Rest verschieben“): offene Punkte (u. a. AC-02 Messung nach 1 h) verschoben nach WN3 (B-895), keine Abnahme erteilt; Abnahme-Ticket B-1286.
