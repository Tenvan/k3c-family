# WZ7 · SIM · Welt-Betrachter im Asset-Studio mit Chunk- und Live-Vorschau

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-460
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`B-460 › Ausgangslage`: `leveltest.html` zeigt das Level als kleinen Chunk-Streifen vom laufenden Spielserver (B-092) und startet es neu oder voll ausgebaut im Spiel (B-315); eine Ansicht mit Spielgrafik gibt es nur im Spiel, der Snapshot dafür entsteht nur im Raum. Voraussetzung: WZ6 ist erledigt (Asset-Studio `tools/k3c-studio` mit Asset-Handler für `public/`, Aliasen `@game`/`@data` und Glossar-Einträgen aus WZ6.1).

## Ziel

Der Welt-Betrachter läuft im Asset-Studio statt in `leveltest.html`: Chunk-Vorschau beim Öffnen maximiert, darunter die Live-Vorschau mit Spielgrafik, umschaltbarer Hub-Stufe, Gebäuden, Tageszeit, Hintergrund-Schichten (ohne Bürger und Gegner), dazu „Im Spiel öffnen“.

Am Ende sichtbar: im Studio-Fenster die Seite Welt-Betrachter mit beiden Vorschauen; auf der Entwicklerseite keine Kachel „Level“ mehr; `task check:all` und CI grün; von 🧑 abgenommen.

## Beteiligte und Zielgruppen

`B-460 › Beteiligte und Zielgruppen`; Agenten setzen die Sessions um, ein getrennter Reviewer prüft (WZ7.6), 🧑 nimmt am PC ab (WZ7.7).

## Anforderungen

`B-460 › Anforderungen`. Sprint-eigen:
- Die Vorschau-Funktion entsteht in SRV vor der Oberfläche: Insel-Erzeugung als neue Datei in `engine/room/` (aus `fullIsland`), exportierte Funktion in `engine/net/` über `stateOf`, damit Level und Zustand genau das Format des Spiels haben. Sie nimmt eine **Options-Struktur** (Seed, Quelle Biom bzw. Insel + Stufe, Hub-Stufe, Tageszeit als Sim-Phase + Fortschritt, Gebäude-Auswahl; keine Bürger und Gegner), setzt **keinen globalen Zustand** (kein `sim.UseData`) und bleibt für spätere Felder offen: WZ8.4 ergänzt „geplante Chunks“, WZ9 ergänzt in einer eigenen SRV-Session „Kandidaten-`hub.json`“.
- Die Tageszeit setzt eine eigene Setzfunktion, nicht `sim.DevSetPhase` (springt nur bis kurz vor den Wechsel, `engine/sim/dev.go` Z. 46–48). Sie setzt `w.Cycle` (Phase und Fortschritt) **direkt** und leitet `w.Time` passend daraus ab; der Fortschritt liegt in **[0, 1)** (1 wäre der Beginn der nächsten Phase und wird abgelehnt). Die Sim kennt **drei Phasen** (`day`, `dusk`, `night`; `dawn` ist nur ein Ereignis beim Tagesbeginn, `engine/sim/cycle.go`); die Setzfunktion setzt diese drei. Die Oberfläche bietet **vier Einträge** (Tag, Dämmerung, Nacht, Morgengrauen, keiner ausgegraut); das Studio-Frontend bildet Morgengrauen ab als Phase `night` mit Fortschritt **0,9 + 0,1·p für p in [0, 1)**, so zeichnet der Client es heute (`daylight()` in `src/scenes/viewRules.ts`: letzte 10 % der Nacht). p = 1 entspräche dem Tagesbeginn (Phase `day`, Fortschritt 0) und wird nicht angeboten. Der Vitest vergleicht die Fortschrittswerte wegen Gleitkomma mit `toBeCloseTo`. Eine echte Phase Morgengrauen baut B-470; dann entfällt die Abbildung.
- Das Studio bindet `engine/level` (Chunk-Vorschau) und die Vorschau-Funktion als Wails-Bindings an, Modul `k3c` über `replace` wie `tools/k3c-dev`. Die Quelle der Chunk-Vorschau ist in WZ7 das Biom, die Schnittstelle trägt sie als Wert und kann Insel + Stufe aufnehmen (WZ8.5 stellt um).
- Die Logik aus `levelView.ts`/`levelApi.ts` zieht mit ihren Tests ins Studio-Frontend um (wie `assetvorschau` in WZ6), erst danach wird `leveltest.html` entfernt.
- Die Live-Vorschau lädt die Grafiken wie das Spiel (Preload aus `src/scenes/` über `@game`) und zeichnet mit `StageView`/`WorldRenderer` in einer Phaser-Szene des Studios. WZ7.3 führt Phaser im Studio-Frontend ein und baut diese Szene samt Preload als **wiederverwendbare Bühne**; WZ10 nutzt sie wieder, eine zweite Bühne entsteht nicht.

## Nicht-Ziele

`B-460 › Nicht-Ziele`, insbesondere: keine Änderung an B-092 (wird umgeplant), keine Änderung unter `src/scenes/`, kein Protokoll- oder Spielstand-Wechsel, kein HTTP-Endpunkt für die Vorschau. Chunk-Planung (WZ8, B-461) und Gebäude-Planer (WZ9, B-462) folgen.

## Regeln und Einschränkungen

`B-460 › Regeln und Einschränkungen`. Domänen nacheinander, je Session eine: **SIM** (WZ7.8: neue Datei `engine/sim/preview.go` samt Test und Codemap, bestehende Funktionen unverändert) → **SRV** (WZ7.1: `engine/room/`, `engine/net/`) → **DEV** (WZ7.2–WZ7.7: `tools/k3c-studio/**`, `src/tools/**`); die Review-Session trägt die Domäne der letzten Umsetzung (DEV) und prüft den Diff beider Domänen. Das Entfernen von `leveltest.html` samt Kacheln, Texten, Tests, Codemaps und Sprint-Übersicht fällt als Grenzfall in DEV (wie WZ6.6/B-299). `src/scenes/`, `src/online/`, `src/model/`, `public/` und `data/` nur lesend (Ausnahme: die Codemaps unter `src/`). Die Snapshot-Funktion setzt keinen globalen Zustand und ihre Options-Struktur bleibt erweiterbar. Abhängigkeit: WZ6 erledigt. Richtwert je Session ≤ ~400 geänderte Code-Zeilen; Fenster- und Browserprüfungen nur mit Freigabe von 🧑 für den Lauf. Mit 8 Sessions liegt der Sprint wie WZ6 über dem Richtwert 3–6, weil Review und Abnahme getrennt sind.

## Beispiele

`B-460 › Beispiele`. Sprint-eigen: Ralf baut das Studio neu, während die Workbench läuft → MCP und Planung bleiben verbunden (B-459); die Entwicklerseite `dev.html` zeigt keine Kachel „Level“ mehr, `testing.html` keinen Abschnitt „Level“.

## Ausnahme- und Fehlerfälle

`B-460 › Ausnahme- und Fehlerfälle`. Sprint-eigen: Braucht die Live-Vorschau eine Änderung unter `src/scenes/` oder `src/online/` (z. B. weil `StageView` ohne `GameScene` nicht zeichnet), wird die Session `blockiert` und ein Ticket für CLI angelegt, statt dort zu ändern.

## Akzeptanzkriterien

- **AC-01** `B-460/AC-01` Chunk-Vorschau im Asset-Studio mit dem Umfang von `leveltest.html`, ohne Spielserver; Quelle in WZ7 das Biom, Schnittstelle offen für Insel + Stufe.
- **AC-02** `B-460/AC-02` Chunk-Vorschau beim Öffnen maximiert, verkleinerbar.
- **AC-03** `B-460/AC-03` exportierte Go-Funktion (SRV) mit Options-Struktur, ohne globalen Zustand, liefert Level und Zustand deterministisch zu Seed, Quelle, Hub-Stufe, Gebäude-Auswahl, Tageszeit (Sim-Phase `day`/`dusk`/`night` + Fortschritt in [0, 1)), ohne Bürger und Gegner; eine andere Phase (auch `dawn`) und ein Fortschritt außerhalb von [0, 1) werden abgelehnt.
- **AC-04** `B-460/AC-04` Live-Vorschau mit Spielgrafik auf wiederverwendbarer Phaser-Bühne, Hub-Stufe aus `levels`, Gebäude ein/aus, gleiches Bild bei gleichen Eingaben.
- **AC-05** `B-460/AC-05` Tageszeit (vier Einträge Tag, Dämmerung, Nacht, Morgengrauen; Setzfunktion setzt `w.Cycle` direkt und leitet `w.Time` ab; Morgengrauen als Sim-Phase `night` mit Fortschritt 0,9 + 0,1·p für p in [0, 1) abgebildet, Vitest mit `toBeCloseTo`), Hintergrund-Schichten mit Parallaxe.
- **AC-06** `B-460/AC-06` „Im Spiel öffnen“ neu oder voll ausgebaut über die Landingpage, nur Loopback.
- **AC-07** `B-460/AC-07` `leveltest.html` mit Skript, Logik, Tests, Kacheln und Texten entfernt; `git grep` auf `leveltest` ist leer bis auf `.codemap/state.json`, `docs/backlog`, `docs/sprints/erledigt` und die Sprint-Ordner WZ6/WZ7; Codemaps und Sprint-Übersicht nachgezogen.
- **AC-08** `B-460/AC-08` Tests in `task check:all` und CI.
- **AC-09** 🧑 nimmt den Welt-Betrachter am PC ab und entscheidet über die Umplanung von B-092 und die Punkte 1 und 4 von B-426 (Abnahme-Session).

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat), siehe `B-460 › Offene Fragen` 1–5: `leveltest.html` entfernen, Snapshot aus Go, Gebäude nur Darstellung, Hub-Stufe über `levels`, Live-Optionen, Chunk-Vorschau maximiert.

Querschnitts-Entscheidungen WZ7–WZ10 (Planung, gelten mit der Freigabe), siehe `B-460 › Offene Fragen` 6–9: Snapshot mit Options-Struktur ohne globalen Zustand, eigene Tageszeit-Setzfunktion für die drei Sim-Phasen mit vier Einträgen in der Oberfläche (Morgengrauen als `night` 0,9 + 0,1·p, bis B-470 eine echte Phase baut), Phaser-Bühne aus WZ7.3 für WZ10, Chunk-Quelle Biom oder Insel + Stufe.

Beantwortet von 🧑 am 2026-10-17 (Chat), siehe `B-460 › Offene Fragen` 10–13: Hub-Stufe reihum wie im Voll-Ausbau; keine Bürger und keine Gegner, dafür die Umgebung; kein HTTP-Endpunkt (Wails-Binding); Nacht und Morgengrauen generell dunkler mit Licht von Fackeln und Lagerfeuern, ausgelagert in B-472 (nicht Teil von WZ7; ändert `daylight()`, auf das WZ7.4/AC-05 testet, wer später landet zieht nach).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ7.8 | `WZ7.8-sim-vorschau-regeln.md` | Umsetzung | autonom | offen |

- WZ7.8 (SIM) läuft vor WZ7.1: Sim-Hilfen für die Vorschau in neuer Datei `engine/sim/preview.go`: `PreviewCycle` setzt `w.Cycle` (Sim-Phase `day`, `dusk`, `night` und Fortschritt in [0, 1)) direkt und leitet `w.Time` ab (nicht `sim.DevSetPhase`; `dawn`, unbekannte Phase und Fortschritt außerhalb von [0, 1) → Fehler); `PreviewHubLevel` baut eine Welt auf Hub-Stufe n nach den Regeln der Simulation aus (Plätze wie `devBuildSite`) und setzt die Material-Stufen je Art reihum von 1 bis `min(n, Stufenzahl)`; Go-Tests, kein globaler Zustand (AC-03, AC-08)
- WZ7.1 (SRV) Vorschau-Snapshot als exportierte Go-Funktion mit Options-Struktur, ohne globalen Zustand (kein `sim.UseData`): Insel aus Seed, Quelle Biom bzw. Insel + Stufe, Hub-Stufe, Gebäude-Auswahl (Snapshot ohne Bürger und Gegner), Tageszeit und Hub-Stufe über `sim.PreviewCycle` und `sim.PreviewHubLevel` aus WZ7.8 (Abhängigkeit; Fehler der Optionen wie dort), Entfernen von Bürgern und Gegnern, Level und Zustand über `stateOf`; Struktur offen für WZ8.4 und WZ9; Go-Tests für Determinismus, Hub-Stufen, Tageszeit der drei Sim-Phasen und Fehlerfälle (AC-03, AC-08)
- WZ7.2 (DEV) Seite Welt-Betrachter im Studio: Go-Binding auf `engine/level` (Quelle als Wert, in WZ7 das Biom), Logik und Tests aus `levelView.ts`/`levelApi.ts` umziehen, Chunk-Vorschau maximiert und verkleinerbar (AC-01, AC-02, AC-08)
- WZ7.3 (DEV) Live-Vorschau: Phaser im Studio-Frontend einführen, wiederverwendbare Bühne (Szene + Preload) mit `StageView`/`WorldRenderer` über `@game`, Binding auf WZ7.1, Hub-Stufe aus `levels` (Material-Stufen der Gebäude reihum laut Snapshot, das Frontend rechnet keine Stufen), Gebäude ein/aus (AC-04, AC-08)
- WZ7.4 (DEV) Live-Optionen und „Im Spiel öffnen“: Tageszeit mit vier Einträgen (Tag, Dämmerung, Nacht, Morgengrauen, mit Fortschritt p in [0, 1)) und reiner Abbildungsfunktion im Frontend (Morgengrauen = `night` mit 0,9 + 0,1·p; Test gegen `daylight()`, Gleitkomma mit `toBeCloseTo`), Hintergrund-Schichten mit Parallaxe, Spiel neu oder voll ausgebaut im Browser über die Landingpage (AC-05, AC-06)
- WZ7.5 (DEV) `leveltest.html` entfernen (Grenzfall wie WZ6.6): Seite, Skript, Logik, Tests, Kacheln in `devTiles.ts` und `testTiles.ts`, Texte `level.*`, Codemaps, `docs/sprints/README.md`; `git grep` auf `leveltest` mit den Ausnahmen aus AC-07 leer (AC-07)
- WZ7.6 (DEV) Review: `task check:all`, Diff SRV und DEV, nur schwere Befunde, PR des Sprints (alle)
- WZ7.7 (DEV) Abnahme durch 🧑 am PC, Agent Mensch: Welt-Betrachter, Umplanung B-092, B-426 Punkte 1 und 4 (AC-09)

Domänen-Reihenfolge: SIM (WZ7.8, zuerst) → SRV (WZ7.1) → DEV (WZ7.2–WZ7.7). WZ7.8 trägt die höchste Nummer, weil sie später aufgenommen wurde; die Nummern von WZ7.1–WZ7.7 bleiben stabil, weil WZ8–WZ11 und B-463 auf sie verweisen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
