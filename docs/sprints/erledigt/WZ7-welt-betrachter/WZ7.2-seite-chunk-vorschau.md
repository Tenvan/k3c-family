# WZ7.2 · Seite Welt-Betrachter mit Chunk-Vorschau im Studio

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz7/2-seite-chunk-vorschau
- **Abhängig von:** –
- **Tickets:** B-460
- **Kriterien:** AC-01, AC-02, AC-08

## Ziel

Im Asset-Studio gibt es die Seite Welt-Betrachter mit der Chunk-Vorschau im Umfang von `leveltest.html`, ohne Spielserver, beim Öffnen maximiert und verkleinerbar (AC-01, AC-02).

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Asset-Studio aus WZ6: `tools/k3c-studio` (Wails, Frontend mit Aliasen `@game`/`@data`, Modul `k3c` über `replace` wie `tools/k3c-dev`). Gestaltung der Seiten Grafik/Sound dort als Vorbild.
- Heute: `leveltest.html` mit `src/tools/leveltest.ts`, Logik in `levelView.ts`/`levelApi.ts` (mit Tests); Level kommt per `GET /api/level` vom Spielserver. Neu: Go-Binding auf `engine/level` (Quelle als Wert, in WZ7 das Biom; Schnittstelle kann später Insel + Stufe aufnehmen).
- Logik und Tests ziehen ins Studio-Frontend um (wie `assetvorschau` in WZ6); `leveltest.html` bleibt bis WZ7.5 bestehen.
- Fenster- und Browserprüfungen nur mit Freigabe von 🧑 für den Lauf.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `tools/k3c-studio/**` (Go-Binding, Frontend-Seite, Logik, Tests, Codemap)
- `src/tools/levelView.ts`, `src/tools/levelApi.ts` und deren Tests (nur lesen/kopieren; Entfernen erst WZ7.5)
- diese Session-Datei

## Nicht-Ziele

Kein Entfernen von `leveltest.html` (WZ7.5). Keine Live-Vorschau, kein Phaser (WZ7.3). Keine Änderung unter `src/scenes/`, `src/online/`, `src/model/`, `public/`, `data/`, `engine/`. Keine Chunk-Planung (B-461).

## Schritte

1. Go-Binding auf `engine/level`: Layout (Chunks) zu Seed und Quelle (Wert: Biom), Warnungen der Spielbarkeits-Prüfung; Seed-Prüfung wie `/api/level`.
2. Logik und Tests aus `levelView.ts`/`levelApi.ts` ins Studio-Frontend verschieben bzw. kopieren und auf das Binding umstellen.
3. Seite Welt-Betrachter mit Chunk-Streifen im Umfang von `leveltest.html`; beim Öffnen maximiert, per Schalter verkleinerbar (Live-Vorschau darunter folgt in WZ7.3).
4. Vitest und Go-Tests; Codemap nachziehen.

## Fertig, wenn

- [x] AC-01: Chunk-Vorschau zeigt zu Seed und Biom dasselbe wie `leveltest.html`, ohne Spielserver (Tests der umgezogenen Logik, Go-Test des Bindings).
- [x] AC-02: Chunk-Vorschau ist beim Öffnen maximiert und verkleinerbar (Frontend-Test).
- [x] AC-08: `check_run` `task:check`, `go:test` und `go:lint` grün.

## Prüfen

`task check` und `task check:go`; über k3c-dev `check_run` (mit `checkout`). Fenster-Beobachtung nur mit Freigabe von 🧑.

## Ergebnis

Umgesetzt: Go-Binding `WorldLevel(seed, WorldSource)` (`tools/k3c-studio/app_world.go`, Modul `k3c` über `replace => ../..`), Seite Welt-Betrachter (`frontend/src/world/`: `WorldPage`, `ChunkPreview`, `levelView.ts` samt Test aus `src/tools/levelView.*` umgezogen), Backend-Erweiterung (`api/types.ts`, `wails.ts`, `mock.ts`, `mockWorld.ts`), Navigation „Welt-Betrachter“ in `App.tsx`, `styles/world.css`, Codemap. `leveltest.html` und `src/tools/level*` bleiben bis WZ7.5 unverändert.

- **AC-01 geprüft:** Go-Test: für Seed `k3c` und jedes eingebettete Biom (5 Dateien aus `data/biomes`) dasselbe Layout und dieselben Warnungen wie `level.Generate`/`level.Validate`, zwei Aufrufe gleich, JSON mit allen Feldern der Seite und `warnings` nie `null`, ungültiger Seed (leer, zu lang), Biom (unbekannt, mit Pfad, fehlend) und Quelle Insel + Stufe → Hinweis statt Layout. Die Quelle ist ein Wert (`WorldSource{Biome, Island, Stage}`, in WZ7 nur `Biome` ausgewertet), die Schnittstelle trägt Insel + Stufe schon. Die umgezogenen Tests am Golden-Level (`levelView.test.ts`: Abschnitte lückenlos, Objekte nach x, Anzahl je Art, Warnungen, unvollständige Antwort, Zufalls-Seed, Zoom, Farben) laufen im Studio-Vitest. DOM-Test: die Seite zeigt Abschnitte, Objekte, Hub-Linie, Legende und Zusammenfassung ohne Spielserver; eine abgelehnte Anfrage zeigt den Hinweis statt eines Streifens, Warnungen stehen unter dem Streifen, bei zwei schnellen Anfragen gilt nur die letzte.
- **AC-02 geprüft:** Frontend-Tests: Start `maximiert` (`data-maximized`, Klasse `wd-max`, Live-Vorschau eingeklappt, Knopf „Verkleinern“), Knopf und Taste M verkleinern und maximieren wieder (nicht beim Tippen im Eingabefeld), Zoom per Knöpfe und Tasten + / − bis zur Grenze, Pfeiltasten scrollen ±120 px. **Beobachtung durch 🧑 im Studio steht aus** (WZ7.7); der Agent hat kein Fenster geöffnet.
- **AC-08 geprüft:** `check_run task:check` grün (Root-Vitest, Typecheck, `oxlint`), `golangci-lint run` im Studio mit 0 Befunden, Studio-Vitest (9 Dateien, 69 Tests), `tsc --noEmit`, `vite build` und `go test` des Studio-Moduls (kompiliertes Binary in `.work/`) grün. `task check:studio` selbst läuft im Worktree nicht über `check_run` (`task_*` nimmt nur die Repo-Wurzel); seine Bestandteile habe ich einzeln ausgeführt.
- **Review (getrennt, Subagent code-reviewer):** keine kritischen Befunde, eingearbeitet: Taste M und Tastatur-Zoom/-Scroll (AC-02 „Knopf und Taste“), Quelle als Wert, Go-Test über alle Biome mit Warnungen, Codemap, Zoom-Knöpfe „Herauszoomen“/„Hineinzoomen“ (zweimal „Verkleinern“ in der Leiste), Test der Anfragen-Reihenfolge.
- **Abweichungen:** „Im Spiel starten“ und „Voll ausgebaut“ (Taste V) folgen in WZ7.4 (AC-06), Controller-Bedienung (Stick, LB/RB) entfällt, das Studio ist ein PC-Werkzeug (B-460, B-092 wird umgeplant). Ein leerer Seed ergibt jetzt einen Hinweis; `GET /api/level` setzte stattdessen `k3c` ein. `go.mod` des Studios: `k3c` und `replace k3c => ../..` ergänzt, `golang.org/x/sys` v0.46 → v0.47 (Wurzel und k3c-dev nutzen v0.47, `go mod tidy` hob es an); `wails build` habe ich nicht gestartet.
- **Hinweis:** Auf diesem Worktree fehlten die `node_modules`; ich habe `npm ci` in der Wurzel, `tools/k3c-studio/frontend` und `tools/k3c-dev/frontend` ausgeführt (nicht versioniert).
- Neue Tickets: keine.
