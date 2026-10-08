# SP05 · SIM · Port I – Welt, Zyklus, Truppen, Wirtschaft

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-043
- **Start-Commit:** f015b3c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (Truppen nach SP05, Golden-Lauf ohne Spieler, 4 Sessions)

## Ausgangslage

Die Simulation gibt es nur in TypeScript (`src/world/sim/`). Seit SP04 liegen RNG und Level-Generator in Go
(`engine/rng`, `engine/level`) und drei Golden-Läufe in `testdata/golden/sim-*.json`.

## Ziel

Die Go-Simulation (`engine/sim`) rechnet Welt, Tag/Nacht, eigene Truppen und Wirtschaft Tick für Tick wie TypeScript.
Am Ende sichtbar: `go test ./engine/sim/` vergleicht die Golden-Läufe ohne Gegner Feld für Feld und ist grün.

## Beteiligte und Zielgruppen

Entwickler oder Agent; Spieler profitieren erst ab SP08. 🧑 gibt die Spec frei.

## Anforderungen

B-043 › Anforderungen. Sprint-eigen:

- Zustand (`types.ts`), Balancing-Daten (`data.ts`), `createWorld`, `addPlayer` und Tag/Nacht (`cycle.ts`) in Go.
- Eigene Truppen (`units.ts`: Landstreicher, Camps, Bauern, Bogenschützen-Posten) und `common.ts` in Go.
  Grund: Jeder Golden-Lauf hat ab Tick 0 Landstreicher, einen Bauern und zwei Bogenschützen (`data/hub.json`
  › `startTroops`), die den gemeinsamen RNG nutzen. Ohne Truppen ist kein Lauf vergleichbar.
- Wirtschaft (`economy.ts`: Monarchen, Münzen, Bezahlen, Truhen, Bauplätze, Morgen-Gold) in Go.
- `step()` in Go mit den Teilschritten dieses Sprints in der Reihenfolge aus `world.ts`. Die Gegner-Teilschritte
  folgen in SP06.
- Neuer Golden-Lauf `sim-forest-ohne-spieler.json` (forest, Seed `golden-1`, 0 Spieler, `cycleSpeed` 1, 1800 Ticks),
  damit die Truppen vor der Wirtschaft prüfbar sind.

## Nicht-Ziele

Gegner, Wellen (`planWave`), Projektile, Burg-Fall, Reisen (`travel.ts`), Kampagne und Spielstände: alles SP06.
Der Feuer-Teil der Bogenschützen wird portiert, geprüft wird er erst in SP06 (ohne Gegner schießt niemand).

## Regeln und Einschränkungen

- `src/world/` und `src/core/rng.ts` werden nur gelesen (Grenzfall Portierung).
- Ausnahme außerhalb der Domäne (INF): `tests/golden.test.ts` bekommt den neuen Lauf (nur Eintrag in `SIM_RUNS`).
- Fallen wie in SP04: stabile Sortierung (`sort.SliceStable`), Produkte vor einer Addition mit `float64(…)` runden
  (FMA), keine `map`-Iteration mit Einfluss auf das Ergebnis, Reihenfolge der RNG-Aufrufe exakt wie TS.
- Schichtgrenze: `engine/sim` importiert nichts aus `engine/room`, `engine/net`, `cmd/` (depguard prüft das).
- Werte nur aus `data/` (`data.Files`), keine neuen Felder, keine neue Abhängigkeit.

## Beispiele

- `sim-forest-tag.json` (2 Spieler, laufen, bezahlen, rekrutieren) → Go erzeugt alle 61 Snapshots identisch.
- `sim-forest-nacht.json` → bis Tick 960 identisch (bei Tick 990 steht die erste Welle in der `spawnQueue`).

## Ausnahme- und Fehlerfälle

Abweichung → der Test nennt Lauf, Tick und Feldpfad (z. B. `sim-forest-tag Tick 240: troops[3].x = …, erwartet …`).

## Akzeptanzkriterien

- **AC-01** Zustand, `createWorld`, `addPlayer` und Tag/Nacht: Der Snapshot bei Tick 0 aller Golden-Läufe stimmt
  vollständig, `time` und `cycle` stimmen in jedem Snapshot aller Läufe.
- **AC-02** Die Golden-Läufe ohne Gegner sind grün: `sim-forest-tag` vollständig, `sim-forest-nacht` und
  `sim-cave-aggression` bis zum letzten Snapshot ohne Gegner und ohne `spawnQueue`-Eintrag (B-043/AC-02 für diesen Teil).
- **AC-03** Eigene Truppen: `sim-forest-ohne-spieler` ist in allen Snapshots grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP05.1 | `SP05.1-welt-zyklus.md` | Umsetzung | autonom | fertig |
| SP05.2 | `SP05.2-truppen.md` | Umsetzung | autonom | fertig |
| SP05.3 | `SP05.3-wirtschaft.md` | Umsetzung | autonom | fertig |
| SP05.4 | `SP05.4-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-01, leichtes Review über `f015b3c..main` per unabhängigem Subagent (auf Auftrag von 🧑 im selben Lauf).
  Kriterien: AC-01 (SP05.1), AC-03 (SP05.2), AC-02 (SP05.3), alle mit Gegentest belegt; B-043/AC-02 folgt in SP06.
- Behobene Befunde: zwei Produkte ohne `float64(…)` (FMA) in `movePlayer` und `cycleAt`; `golden.Diff` übersah
  Schlüssel, die nur auf einer Seite stehen.
- Neue Tickets: B-074 (Golden-Lücken der Wirtschaft, eingeplant für SP06).
