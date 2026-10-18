# WZ7 · SIM, SRV, DEV · Welt-Betrachter im Asset-Studio mit Chunk- und Live-Vorschau

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** SIM, SRV, DEV
- **Reife:** bereit
- **Tickets:** B-460
- **Start-Commit:** dd2ee75d
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Chat (Ralf: „Ja, freigegeben“)

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
- Die Logik aus `levelView.ts`/`levelApi.ts` zieht mit ihren Tests ins Studio-Frontend um (wie die Asset-Vorschau in WZ6), erst danach wird `leveltest.html` entfernt.
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
| WZ7.8 | `WZ7.8-sim-vorschau-regeln.md` | Umsetzung | autonom | fertig |
| WZ7.1 | `WZ7.1-vorschau-snapshot.md` | Umsetzung | autonom | fertig |
| WZ7.2 | `WZ7.2-seite-chunk-vorschau.md` | Umsetzung | autonom | fertig |
| WZ7.3 | `WZ7.3-live-vorschau.md` | Umsetzung | autonom | fertig |
| WZ7.4 | `WZ7.4-live-optionen-im-spiel.md` | Umsetzung | autonom | fertig |
| WZ7.5 | `WZ7.5-leveltest-entfernen.md` | Umsetzung | autonom | fertig |
| WZ7.6 | `WZ7.6-review.md` | Review | autonom | fertig |
| WZ7.7 | `WZ7.7-abnahme.md` | Review | Mensch | fertig |


Domänen-Reihenfolge: SIM (WZ7.8, zuerst) → SRV (WZ7.1) → DEV (WZ7.2–WZ7.7). WZ7.8 trägt die höchste Nummer, weil sie später aufgenommen wurde; die Nummern von WZ7.1–WZ7.7 bleiben stabil, weil WZ8–WZ11 und B-463 auf sie verweisen.

## Abnahme

2026-10-18 (Abnahme WZ7.7 durch 🧑 am PC): AC-01 bis AC-08 mit Nachweis aus WZ7.6; AC-09 verschoben nach B-477 (Mängel der Ansicht, Sprint WZ16). B-092 verworfen, B-426 Punkt 1 und 4 verworfen. Neue Tickets: B-477.
Version: v0.19.0 gesetzt (2026-10-18, Minor wegen Wirkung im Werkzeug, zusammen mit WZ16; Tag auf `ea750844`, Release-Lauf grün, `/api/health` und Landingpage im Nachbau mit den Flags des Release-Workflows v0.19.0).
