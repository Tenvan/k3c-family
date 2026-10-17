# WZ7.4 · Live-Optionen und Im Spiel öffnen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz7/4-live-optionen-im-spiel
- **Abhängig von:** WZ7.3
- **Tickets:** B-460
- **Kriterien:** AC-05, AC-06

## Ziel

Die Live-Vorschau bietet Tageszeit mit vier Einträgen und Hintergrund-Schichten mit Parallaxe; „Im Spiel öffnen“ startet das Level neu oder voll ausgebaut im Browser über die Landingpage (AC-05, AC-06).

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Tageszeit: Oberfläche mit Tag, Dämmerung, Nacht, Morgengrauen (alle wählbar, Fortschritt p in [0, 1)). Das Frontend bildet Morgengrauen als Sim-Phase `night` mit Fortschritt 0,9 + 0,1·p ab (so zeichnet es `daylight()` in `src/scenes/viewRules.ts`); p = 1 gibt es nicht. Reine Abbildungsfunktion, Test gegen `daylight()` mit `toBeCloseTo`. Die Setzfunktion in Go (`sim.PreviewCycle`) kennt nur die drei Sim-Phasen.
- Abhängigkeit B-472: ändert `daylight()` (dunklere Nacht); wer später landet, zieht Test und Vorschau nach.
- Hintergrund-Schichten ein/aus mit Parallaxe beim Scrollen (Scroll-Faktoren wie im Spiel); Biom ohne Schichten → Schalter ausgegraut.
- „Im Spiel öffnen“: neu oder voll ausgebaut (B-315, `engine/net/dev.go` › `/api/dev/fullsave`, nur Dev-Mode) im Browser über die Landingpage; nur Loopback. Hub-Stufe, Gebäude-Auswahl und Tageszeit gehen nicht ins Spiel.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `tools/k3c-studio/**` (Frontend, Go-Anbindung für „Im Spiel öffnen“, Tests, Codemap)
- diese Session-Datei

## Nicht-Ziele

Keine Änderung unter `src/scenes/`, `src/online/`, `src/model/`, `public/`, `data/`, `engine/`. Kein Entfernen von `leveltest.html` (WZ7.5). Keine Bürger- oder Gegner-Schalter. Keine Änderung der Nachtdarstellung (B-472).

## Schritte

1. Test zuerst für die Abbildungsfunktion der vier Tageszeit-Einträge auf Phase und Fortschritt (Morgengrauen mit p = 0, 0,5, 0,9 → `night` mit 0,9, 0,95, 0,99; Helligkeit gleich `daylight()`; `toBeCloseTo`).
2. Tageszeit-Auswahl in der Live-Vorschau, Binding auf `sim.PreviewCycle`.
3. Hintergrund-Schichten ein/aus mit Parallaxe.
4. „Im Spiel öffnen“ (neu / voll ausgebaut) über die Landingpage, nur Loopback; Fehlerfälle (Dev-Mode aus, Raum belegt) anzeigen.
5. Tests, Codemap.

## Fertig, wenn

- [x] AC-05: vier Tageszeit-Einträge, Abbildung von Morgengrauen und Helligkeit getestet (`toBeCloseTo`); Hintergrund-Schichten mit Parallaxe schaltbar.
- [x] AC-06: „Im Spiel öffnen“ neu und voll ausgebaut über die Landingpage, nur Loopback (Test der Anbindung).
- [x] CI: `check_run` `task:check`, `go:test` und `go:lint` grün.

## Prüfen

`task check` und `task check:go`; über k3c-dev `check_run` (mit `checkout`). Fenster- oder Browser-Beobachtung nur mit Freigabe von 🧑.

## Ergebnis

Umgesetzt: Tageszeit mit vier Einträgen samt Fortschritt und Helligkeit (`frontend/src/stage/timeOfDay.ts`, `LivePreview.tsx`, `PreviewStage.setLook`), Hintergrund-Schichten schaltbar mit Parallaxe (`stageLogic.ts` › `backgroundToggles`), „Im Spiel öffnen“ neu und voll ausgebaut (`tools/k3c-studio/app_game.go`, `frontend/src/world/GameLauncher.tsx`, `levelView.ts` › `openGameReason`), Mock, Codemap. Unter `src/`, `engine/`, `data/` und `public/` keine Änderung.

- **AC-05 geprüft:** Vitest: vier Einträge Tag, Dämmerung, Nacht, Morgengrauen (keiner ausgegraut); Tag, Dämmerung, Nacht gehen unverändert an die Sim-Phase, Morgengrauen mit p = 0, 0,5, 0,9 → Phase `night` mit 0,9, 0,95, 0,99 (`toBeCloseTo`); Helligkeit gegen `daylight()` aus `src/scenes/viewRules.ts` (Morgengrauen gleich p, Dämmerung 1 − p, Tag 1, Nacht 0); Fortschritt auf [0, 0,99] begrenzt (Go lehnt 1 ab); DOM-Test der Live-Vorschau: gewählte Tageszeit und Fortschritt gehen als Sim-Phase an die Vorschau, die Kamerahelligkeit folgt (0,45 + 0,55·daylight). Hintergrund-Schichten: jedes Biom hat drei Schichten von fern nach nah mit steigenden Scroll-Faktoren unter 1 (Parallaxe), Abwählen blendet genau diese Textur aus (die Fluss-Spiegelung nutzt dieselbe und folgt), ein Biom ohne Zuordnung zeigt einen ausgegrauten Schalter. Bürger und Gegner gibt es nicht (Entscheidung 11). Beobachtet (Agent, Browser-Pane mit Mock): Nacht dunkelt das Bild ab, die nahe Schicht lässt sich ausblenden. Beobachtung im Studio-Fenster durch 🧑 steht aus (WZ7.7).
- **AC-06 geprüft:** Go-Tests mit einem Spielserver auf Loopback (httptest): neu öffnet `<Adresse>/#game.html%3Fautostart%3D1%26save%3D<seed>` (nur Wald, Seed im Namensformat, höchstens 32 Zeichen), voll ausgebaut legt über `POST /api/dev/fullsave` den Spielstand `voll-<seed>` an (höchstens 27 Zeichen, jedes Biom) und lädt ihn; Hinweise: kein Dev-Mode (403), Raum belegt (409), Serverfehler, falsche Antwort, Server aus oder kein Spielserver (`/api/health` ≠ 200) → „Spielserver läuft nicht“, in allen Fällen wird kein Browser geöffnet; nur Loopback (Negativliste: fremde Hosts, `localhost.evil.com`, `127.0.0.1.evil.com`, Zugangsdaten, andere Schemata, leere und halbe Adressen) ohne eine Anfrage; der Client folgt keiner Weiterleitung (Test mit 307 und 302 auf eine fremde Adresse). DOM-Test der Knöpfe: Binding mit Adresse, Seed, Biom und Art, ausgegraut mit Grund (Höhle, Seed im falschen Format), Hinweis des Servers, Adresse gemerkt (Standard `http://127.0.0.1:5183`). Der Start im Browser selbst (`runtime.BrowserOpenURL`) und die Landingpage-Öffnung hat der Agent nicht geprüft: Beobachtung durch 🧑 (WZ7.7).
- **AC-08 / CI:** `check_run task:check` grün, `golangci-lint run` im Studio 0 Befunde, Studio-Vitest (14 Dateien, 113 Tests), `tsc --noEmit`, `vite build`, Go-Tests des Studio-Moduls (kompiliertes Binary in `.work/`) grün.
- **Review (getrennt, Subagent code-reviewer):** keine HIGH-Befunde; eingearbeitet: Standardadresse 5183 statt 8080 (Spec), Erreichbarkeitsprüfung vor dem Öffnen auch bei „neu“ mit dem Text „Spielserver läuft nicht“, keine Weiterleitung des HTTP-Clients, Name des Spielstands aus der Serverantwort im Namensformat, Schalter bei Biom ohne Schichten ausgegraut, Filter der Kamera vor dem Anhängen leeren, Hinweis des Startknopfs gilt nur für seinen Seed, Test ohne Data Race, Mock mit der Grenze von 27 Zeichen.
- **Offen für 🧑 (WZ7.7):** `POST /api/dev/fullsave` nimmt nur den Seed und baut immer im Wald; für ein anderes Biom öffnet „Voll ausgebaut“ daher einen Wald-Spielstand (B-426 Punkt 1, Biom-Wahl beim Voll-Ausbau). Die Nacht-Helligkeit (0,45) ist eine Kopie aus `GameScene` (nicht exportiert); B-472 macht die Nacht dunkler, wer zuletzt landet, zieht `NIGHT_BRIGHTNESS` und den Test nach.
- Neue Tickets: keine.
