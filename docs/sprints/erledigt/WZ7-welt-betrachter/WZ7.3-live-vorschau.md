# WZ7.3 · Live-Vorschau mit Phaser-Bühne im Studio

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz7/3-live-vorschau
- **Abhängig von:** WZ7.1, WZ7.2
- **Tickets:** B-460
- **Kriterien:** AC-04, AC-08

## Ziel

Die Live-Vorschau zeichnet den Snapshot aus WZ7.1 mit der Spielgrafik auf einer wiederverwendbaren Phaser-Bühne im Studio; Hub-Stufe aus `levels` und Gebäude ein/aus sind schaltbar (AC-04).

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Phaser kommt in der Version der Wurzel-`package.json` ins Studio-Frontend (Auflösung über Wurzel-`node_modules` oder Eintrag in `tools/k3c-studio/frontend/package.json` legt die Session fest); keine weitere Abhängigkeit.
- Die Bühne (Szene + Preload) ist die einzige im Studio; WZ10/WZ11 nutzen sie ohne Kopie wieder (`B-460 › Anforderungen`).
- Zeichnen mit `StageView`/`WorldRenderer` aus `src/scenes/` über `@game`, **ohne** Änderung dort. Braucht das eine Änderung unter `src/scenes/` oder `src/online/`, wird die Session `blockiert` und ein CLI-Ticket angelegt.
- Hub-Stufe: alle Einträge aus `data/hub.json` › `levels`; die Material-Stufen kommen reihum aus dem Snapshot (WZ7.1/WZ7.8), das Frontend rechnet keine Stufen. Gebäude ein/aus ist reine Auswahl für den Snapshot (`kind@x`). Keine Bürger, keine Gegner.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `tools/k3c-studio/**` (Binding auf WZ7.1, Frontend: Phaser, Bühne, Seite, Tests, Codemap)
- diese Session-Datei

## Nicht-Ziele

Keine Änderung unter `src/scenes/`, `src/online/`, `src/model/`, `public/`, `data/`, `engine/`. Keine Tageszeit, Hintergrund-Schichten oder „Im Spiel öffnen“ (WZ7.4). Kein Entfernen von `leveltest.html` (WZ7.5). Keine zweite Bühne.

## Schritte

1. Phaser im Studio-Frontend einführen (Version der Wurzel), Build und Typecheck.
2. Bühne bauen: Szene + Preload der Spielgrafik wie das Spiel (über `@game`), `StageView`/`WorldRenderer` zeichnen den Snapshot; Eingabe nur Snapshot, kein Spielserver.
3. Binding auf `net.Preview` (WZ7.1): Optionen Seed, Biom, Hub-Stufe, Gebäude-Auswahl.
4. Auswahl der Hub-Stufe aus `levels` (zusätzlicher Eintrag in einer Test-Fixture erscheint ohne Code-Änderung), Gebäude-Liste aus dem Snapshot, einzeln ein/aus; gleiche Eingaben → gleiches Bild.
5. Frontend-Tests; `git diff` ohne Änderung unter `src/scenes/`; Codemap.

## Fertig, wenn

- [x] AC-04: Live-Vorschau zeichnet den Snapshot auf der wiederverwendbaren Phaser-Bühne; Hub-Stufen aus `levels`, Gebäude ein/aus; gleiche Eingaben → gleiches Bild (Frontend-Tests).
- [x] AC-04: `git diff` ohne Änderung unter `src/scenes/`.
- [x] AC-08: `check_run` `task:check`, `go:test` und `go:lint` grün.

## Prüfen

`task check` und `task check:go`; über k3c-dev `check_run` (mit `checkout`). Fenster-Beobachtung nur mit Freigabe von 🧑.

## Ergebnis

Umgesetzt: Go-Binding `WorldPreview(WorldPreviewArgs)` (`tools/k3c-studio/app_preview.go`, ruft `net.Preview` aus WZ7.1), wiederverwendbare Phaser-Bühne und Live-Vorschau in `frontend/src/stage/` (`PreviewStage`, `stageLogic.ts`, `LivePreview`), Anbindung in `WorldPage` (Live-Vorschau unter der Chunk-Vorschau), Mock mit vom Server erzeugter Fixture (`api/mockWorld.ts`, `api/previewFixture.json`), Codemap. Unter `src/scenes/`, `src/online/`, `src/model/`, `public/`, `data/` und `engine/` gibt es keine Änderung (`git diff -- src engine data public` leer).

- **AC-04 geprüft:** Go-Tests: das Binding liefert dasselbe wie `net.Preview`, deterministisch, JSON mit `level`, `state`, `error`, Gebäude-Abwahl entfernt genau den Platz, `dawn`, Fortschritt 1, Hub-Stufe 99, unbekanntes Biom und die Quelle Insel + Stufe → Hinweis ohne Zustand; die Mock-Fixture ist gleich zu `net.Preview(k3c, forest, Hub 5)`. Frontend-Tests: Hub-Stufen aus `data/hub.json` › `levels` (1 bis 5; ein sechster Eintrag in einer Fixture erscheint ohne Code-Änderung), Gebäude-Liste und Schlüssel `kind@x` wie in Go, Optionen, `worldOf` (Biom, Level, Zustand, ohne Bürger und Gegner; Hinweis, fehlender Zustand und unbekanntes Biom → keine Welt), DOM-Test der Live-Vorschau mit Attrappe der Bühne: Abfrage mit Seed, Biom, höchster Hub-Stufe, Bühne erst beim Sichtbarwerden, Abwählen und Wiederanwählen eines Gebäudes, Seed-Wechsel bei abgewählten Gebäuden (Auswahl beginnt neu), gleiche Eingaben → gleiche Welt (zweimal gemountet), Hinweis mit `state: null` ohne Absturz, Meldung nicht geladener Grafiken, Abbau beim Verlassen. `git diff` ohne Änderung unter `src/scenes/`.
- **Beobachtet (Agent, Browser-Pane, Vite-Dev mit Mock):** Die Bühne zeichnet im Studio Hintergrund, Boden, Fluss, Burg der Hub-Stufe 5 und alle Gebäude mit Spielgrafik, die Burg steht mittig, Gebäude-Schalter wirken, Pfeiltasten scrollen. Die Beobachtung im Studio-Fenster durch 🧑 steht aus (WZ7.7).
- **AC-08 geprüft:** `check_run task:check` grün (Root-Vitest 2342 Tests, Typecheck, `oxlint`), `golangci-lint run` im Studio 0 Befunde, Studio-Vitest (11 Dateien, 87 Tests), `tsc --noEmit`, `vite build`, Go-Tests des Studio-Moduls (kompiliertes Binary in `.work/`) grün. `task check:studio` selbst läuft im Worktree nicht über `check_run`; seine Bestandteile habe ich einzeln ausgeführt.
- **Entscheidungen:** Phaser kommt aus der Wurzel-`node_modules` (Version der Wurzel-`package.json`, 4.2.1); Studio und `src/scenes/` lösen `phaser` beide dorthin auf, es gibt eine Instanz und keinen Eintrag im Studio-`package.json` (CI und `task install` installieren die Wurzel zuerst). Phaser wird erst beim ersten Sichtbarwerden der Vorschau geladen. Fehlende Grafiken (z. B. der erzeugte Figuren-Atlas `public/atlas/`, in diesem Worktree nicht vorhanden) meldet die Bühne als Hinweis und zeichnet weiter; die Vorschau zeigt keine Figuren.
- **Review (getrennt, Subagent code-reviewer):** eingearbeitet: Absturz bei Hinweis des Servers (`state` ist dann `null`, Typ, Mock und Seite angepasst), Race zwischen den beiden Anfragen (Antworten tragen ihren Kontext, Auswahl gehört zu Seed, Biom und Hub-Stufe), Kamera (`centerOnX` statt eigener Scroll-Rechnung, nur bei anderem Level neu zentriert), Meldung nicht geladener Grafiken, Pfeiltasten nur mit Fokus auf der Bühne, `ResizeObserver`, Fixture-Abgleich mit Go, Determinismus-Test über zwei Mounts.
- **Hinweis:** `go.mod` des Studios bekommt `github.com/coder/websocket` (indirekt), weil `engine/net` als Ganzes in die EXE kommt; ein kleineres Paket nur für `Preview` wäre ein Ticket für SRV. `wails build` habe ich nicht gestartet.
- Neue Tickets: keine.
