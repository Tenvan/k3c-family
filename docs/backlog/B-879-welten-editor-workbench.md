# B-879 · Die Workbench hat den Welten-Editor aus dem alten Studio wieder

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ58
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14 (Auftrag Ralf im Chat)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ralf, 2026-11-14: Beim Wegfall des Asset-Studios (Entscheidung 005, WB1, Commit `616b5afcb`) ist der Welten-Editor nicht in die Workbench gewandert. Der alte Code liegt in der Historie unter `tools/k3c-studio/frontend/src/world/` (u. a. `ChunkInspector.tsx`, `ChunkPreview.tsx`, `ConvertButton.tsx`, `FixInChat.tsx`), dazu `api/world*.ts`, `api/mockWorld*.ts`, `styles/world.css` und das Go-Backend des Studios. Ebenfalls nicht übernommen wurden die Studio-Bereiche `enemies`, `monarch`, `planner` und `playback` (Prüfung siehe Offene Fragen).

## Ziel

Die Workbench hat eine Seite Welten-Editor: Welten und ihre Chunks ansehen, Werte übersteuern (B-680, Welt-Dateien unter `data/`) und Änderungen per Prompt an Claude geben, wie im alten Studio.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Portierung aus `616b5afcb^:tools/k3c-studio/` nach `tools/k3c-workbench` (Frontend-Seite, Backend-Bindings), Bausteine über `@webkit` (Atomic Design).
- Welt-Dateien ändert die Seite nur über vorhandene oder neue Workbench-Bindings mit Test; Sim und Balancing bleiben gesperrt (keine Läufe).
- Mock-Daten für den Browser-Test (`?mock=1`).

## Nicht-Ziele

Neue Welt-Mechaniken; Simulationsläufe aus dem Editor (Sperre 2026-11-11).

## Regeln und Einschränkungen

Atomic Design (`tests/atomicDesign.test.ts`), Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

- **Bereich:** `tools/k3c-workbench/frontend/src/world/**`, `tools/k3c-workbench/app_world*.go`, `tools/k3c-workbench/internal/world/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Workbench zeigt eine Seite Welten-Editor mit Liste der Welten, Chunk-Vorschau und Inspektor (Test).
- **AC-02** Werte einer Welt lassen sich übersteuern und landen in der Welt-Datei (Test gegen Fixture).
- **AC-03** Sichtprüfung durch Ralf im Fenster.

## Offene Fragen

Sollen auch die Studio-Bereiche `enemies` (Gegner), `monarch`, `planner` und `playback` zurück? Vorschlag Claude: erst Welten-Editor, die anderen als eigene Tickets nach Rückmeldung.

## Notizen

Auftrag Ralf im Chat (2026-11-14): „Es fehlen auch noch die alten Seiten aus dem Studio in der Workbench: WeltenEditor“.

WZ58.1 (2026-11-14): AC-01 und AC-02 umgesetzt und getestet (Session-Ergebnis WZ58.1). **AC-03 offen: Sichtprüfung durch Ralf im Fenster** (Workbench neu bauen, vorher `task k3c-sim:build`); das Ticket bleibt bis dahin eingeplant. Offene Frage zu enemies/monarch/planner/playback unverändert.
