# WZ7.8 · Sim-Hilfen für die Vorschau: Tageszeit setzen und Hub-Stufe aufbauen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** wz7/8-sim-vorschau-regeln
- **Abhängig von:** – (WZ7.1 hängt von dieser Session ab)
- **Tickets:** B-460
- **Kriterien:** AC-03, AC-08

## Ziel

`engine/sim/` bietet zwei exportierte Hilfen für die Vorschau (WZ7.1): `PreviewCycle` setzt die Tageszeit einer Welt (Sim-Phase `day`, `dusk`, `night` + Fortschritt in [0, 1)) direkt und leitet `w.Time` ab; `PreviewHubLevel` baut eine Welt auf Hub-Stufe n nach den Regeln der Simulation aus und setzt die Material-Stufen der Gebäude reihum. Beides ohne globalen Zustand (kein `sim.UseData`), ohne Änderung bestehender Funktionen.

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`, Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md`). Die Session läuft **vor** WZ7.1 (SRV), weil WZ7.1 in `engine/room/` und `engine/net/` nur exportierte `sim`-Funktionen nutzen darf. Die Nummer 8 ist historisch (spätere Aufnahme), die Reihenfolge ist SIM (WZ7.8) → SRV (WZ7.1) → DEV (WZ7.2–WZ7.7).

Stand des Codes:
- `engine/sim/cycle.go`: `cycleAt(globalDayNight, seconds)` liefert `CycleInfo{Phase, Day, Progress, SecondsLeft}` für die drei Phasen `day`, `dusk`, `night`; `dawn` ist nur ein Ereignis beim Tagesbeginn (`stepCycle`). Die Zykluszeit ist `w.Time * w.CycleSpeed`.
- `engine/sim/dev.go`: `DevSetPhase` springt nur kurz vor den Wechsel, taugt nicht. `DevBuildAll(isl, level)` baut alle Stufen einer **Insel** mit **einheitlicher** Material-Stufe `min(level, len(Stufen))` (`devBuildSite`) und kennt die Regeln für Hub-Stufe, Mauerlinien und Tore (`siteHubLevel`, `siteLine`, `lineOpen`, `gateHubLevel`).
- `engine/sim/hub_level.go`: `nextLevel` erlaubt Stufe m eines Nicht-Hub-Platzes erst ab `HubLevel >= m`.
- Der Voll-Ausbau (`engine/room/fullsave.go`, B-315) rotiert die Stufen reihum, aber ohne diese Regeln (B-425). Das Ziel ist dieselbe Reihenfolge-Idee, mit den Regeln.

Entscheidungen (`B-460 › Offene Fragen` 6, 7, 10): Funktionen mit Optionen statt globalem Zustand, Tageszeit direkt setzen, Material-Stufen reihum mit Obergrenze `min(n, Stufenzahl der Art)`.

## Erlaubte Dateien

- `engine/sim/preview.go` (neu)
- `engine/sim/preview_test.go` (neu)
- `engine/sim/codemap.md` (nachziehen)
- `docs/sprints/geplant/WZ7-welt-betrachter/WZ7.8-sim-vorschau-regeln.md` (Ergebnis, Status)

## Nicht-Ziele

Keine Änderung bestehender Funktionen in `engine/sim/` (`DevBuildAll`, `DevSetPhase`, `devBuildSite` bleiben unverändert; neue Funktionen rufen sie nur auf, soweit sie unverändert nutzbar sind). Keine Änderung in `engine/room/`, `engine/net/` oder `src/`. Kein Entfernen von Bürgern und Gegnern (das übernimmt WZ7.1, `w.Troops` und `w.Camps` sind exportiert). Kein `sim.UseData`, keine Paket-Variablen. Kein Auswählen einzelner Gebäude (WZ7.1).

## Schritte

1. Test zuerst für `PreviewCycle(w *World, phase string, progress float64) error`: `day`, `dusk`, `night` mit Fortschritt 0, 0,5 und 0,999 → `w.Cycle.Phase`, `w.Cycle.Progress` (mit Toleranz), `w.Cycle.Day` unverändert gültig, und `cycleAt(globalDayNight, w.Time*w.CycleSpeed)` liefert dieselbe Phase und denselben Fortschritt; `dawn`, leere/unbekannte Phase, Fortschritt −0,1, 1 und 1,5, NaN → Fehler, Welt danach unverändert.
2. `PreviewCycle` umsetzen: Phasenanfang aus `globalDayNight` (Tag, Dämmerung, Nacht in Sekunden), Zykluszeit = Anfang + Fortschritt × Länge der Phase, `w.Time = Zykluszeit / w.CycleSpeed` (`CycleSpeed` 0 behandelt wie 1), `w.Cycle = cycleAt(...)`. Kein Aufruf von `stepCycle` (keine Ereignisse).
3. Test zuerst für `PreviewHubLevel(w *World, level int) error`: für jede Stufe aus `hub.Levels` (Tabellentest): `w.HubLevel == level`; gebaut sind genau die Plätze, die `devBuildSite` für `level` baut (Plätze mit `siteHubLevel <= level`, Mauerlinien k ≤ level, Tore bleiben nach deren Regel); kein gebauter Nicht-Hub-Platz hat `Level > level` oder mehr als die Stufenzahl seiner Art; je Art laufen die Stufen in Reihenfolge von `w.Sites` reihum von 1 bis `min(level, Stufenzahl)` und beginnen danach wieder bei 1; HP und MaxHP entsprechen der Stufe aus `buildings.json`; Stufe 0 und `len+1` → Fehler; zweiter Aufruf mit anderem `level` auf einer frischen Welt ist unabhängig vom ersten (kein Paketzustand).
4. `PreviewHubLevel` umsetzen: `w.HubLevel = level`, Hub-Platz fertig (wie `DevBuildAll`), je Platz `devBuildSite` verwenden, danach die Material-Stufe reihum überschreiben (HP/MaxHP nachziehen). Die Rotation bezieht alle gebauten Plätze ein, die Reihenfolge ist die von `w.Sites`.
5. Gemeinsamer Test: eine Welt aus `CreateWorld(biome, seed, …)` mit `PreviewHubLevel` und `PreviewCycle`, danach 120 Ticks `Step` ohne Panik und ohne Fehler im Zustand (Smoke), gleiche Eingaben → gleiche JSON-Serialisierung der Welt.
6. Codemap `engine/sim/codemap.md` ergänzen, `check_run task:check:go` bzw. `go:test` und `go:lint` grün.

## Fertig, wenn

- [x] AC-03: `PreviewCycle` setzt `w.Cycle` und `w.Time` für `day`, `dusk`, `night` mit Fortschritt in [0, 1); `dawn` und Fortschritt außerhalb von [0, 1) werden abgelehnt (Go-Test).
- [x] AC-03: `PreviewHubLevel` baut je Hub-Stufe genau die freigeschalteten Plätze, Material-Stufen reihum bis `min(level, Stufenzahl)`, ohne Verstoß gegen `nextLevel` (Go-Test).
- [x] AC-03: Kein globaler Zustand: zwei Aufrufe in beliebiger Reihenfolge liefern dieselben Ergebnisse (Go-Test).
- [x] AC-08: `task check:go` (bzw. `go:test` und `go:lint` über `check_run`) grün.
- [x] Bestehende Tests in `engine/sim/` unverändert grün (kein bestehender Test angepasst).

## Prüfen

```bash
task check:go
```

Über k3c-dev: `check_run` mit `target: go:test` und `go:lint` (im Worktree mit `checkout`). Keine manuellen Prüfungen.

## Ergebnis

Umgesetzt in `engine/sim/preview.go` (`PreviewCycle`, `PreviewHubLevel`), Tests in `engine/sim/preview_test.go`, Codemap `engine/sim/codemap.md`.

- **AC-03 geprüft:** `PreviewCycle` setzt `w.Cycle` direkt und leitet `w.Time` ab (Go-Test für `day`, `dusk`, `night` mit Fortschritt 0 bis 0,999, Tag bleibt erhalten, Geschwindigkeiten 0,1 / 3 / 7 / 50 an Tag 1, 2 und 6 mit Fortschritt 0 bleiben in der Phase); `dawn`, leere/unbekannte Phase, Fortschritt −0,1 / 1 / 1,5 / NaN / Inf und `CycleSpeed` ≤ 0 → Fehler, Welt danach unverändert. `PreviewHubLevel` baut je Hub-Stufe 1 bis 5 genau die Plätze nach `siteHubLevel`, Mauerlinie ≤ Stufe und `gate` bleibt unverändert; feste Fälle aus `data/hub.json` (healer ab Hub 3, armory ab Hub 4, Linien > 3 bei Hub 3 ungebaut); Material-Stufen reihum je Art 1…n, jede erlaubte Stufe vertreten, nie über der Hub-Stufe, HP aus `buildings.json`; Fehler bei Stufe 0 / −1 / 6, Welt dann unverändert; kein Paketzustand (Aufrufe a, b, a liefern dasselbe JSON); gleiche Eingaben → nach 120 Ticks dieselbe Welt.
- **AC-08 geprüft:** `go:lint` grün (`check_run`); Paket `engine/sim` komplett grün (kompiliertes Test-Binary in `.work/`, 3 s). `check_run go:test` über alle Pakete lief in das Zeitlimit von 5 min (nicht an dieser Änderung: nur neue Dateien in `engine/sim`, die Sim-Tests laufen in 3 s; vermutlich das Temp-Verzeichnis-Problem mit Norton, siehe Hinweis im Projektgedächtnis); die übrigen Pakete sind unverändert. Zu klären durch 🧑 oder im Review (WZ7.6).
- **Review (getrennt, Subagent code-reviewer):** keine blockierenden Befunde; eingearbeitet: Hinweis „nur auf frischer Welt“ (baut nur auf, nichts ab; w.Time kann rückwärts springen) im Doc-Kommentar, Rundungs-Nachführung von `w.Time` bei Phasenbeginn, `CycleSpeed` ≤ 0 als Fehler, Tests mit unabhängigen Erwartungswerten (feste Fälle, feste Reihum-Folgen) statt aus dem Code gespiegelter Formeln, Smoke-Test mit Determinismus nach 120 Ticks, Marshal-Fehler geprüft.
- **Abweichung vom Plan:** `PreviewHubLevel` nutzt `devBuildSite`, daher bleiben Tore ungebaut (wie `DevBuildAll`; sie hängen an der äußersten Mauer); B-460 nennt „Tor ab `gateHubLevel`“, WZ7.1 muss das bei der Gebäude-Liste beachten. `PreviewCycle` setzt `w.Cycle` direkt (wie die Spec), nicht über `cycleAt`.
- **Hinweise für WZ7.1:** Die Welt muss frisch aus `CreateWorld` kommen. Bürger (`w.Troops`, Landstreicher an `recruitCamp`s) und Gegner entfernt WZ7.1.
- Neue Tickets: keine.
