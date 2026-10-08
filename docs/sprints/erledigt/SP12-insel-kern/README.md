# SP12 · SIM · Insel-Kern

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-100
- **Start-Commit:** d155f3f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1

## Ausgangslage

Die Simulation kennt eine Welt je Stufe (`engine/sim/world.go`), die `Campaign` (`campaign.go`) hält mehrere Welten, tickt aber nur die aktuelle, und alle Spieler wechseln die Stufe gemeinsam (`travel.go`, `Campaign.Travel`). Entscheidung 003 und `docs/rules/stufen.md` verlangen: ein Level (Insel) mit n Stufen, alle laufen weiter, jeder Spieler wechselt einzeln, das Material gehört der Insel. Raum, Protokoll und Client benutzen heute die `Campaign` (`engine/room/`).

## Ziel

Die Simulation kann eine **Insel** mit mehreren Stufen als Einheit rechnen: alle Stufen ticken, Spieler sind an eine Stufe gebunden und wechseln einzeln, das Material ist ein Vorrat je Insel, ein Spielstand speichert Insel und Stufe je Spieler. Am Ende sichtbar: `go test ./engine/sim` mit Insel-Tests (2–3 Stufen, 2–4 Spieler), Determinismus-Test und Benchmark der Tick-Dauer; Raum, Protokoll und Client laufen unverändert weiter auf der `Campaign`.

## Beteiligte und Zielgruppen

Entwickler und Agenten (SIM); 🧑 entscheidet über die Reihenfolge der Folge-Sprints; Familie profitiert erst nach der Umstellung des Raums (B-133) und des Protokolls (B-104).

## Anforderungen

B-100 › Anforderungen. Sprint-eigene Abgrenzung: Die neue Insel-API (`engine/sim/island*.go`) steht **neben** der `Campaign`; die `Campaign` bleibt unverändert funktionsfähig, bis der Raum umgestellt wird (B-133, SRV). Die Wellenstärke je Spieleranzahl und die Raum-Optionen (B-101) sind **nicht** Teil dieses Sprints.

## Nicht-Ziele

Raum, Protokoll und Client (B-133, B-104, B-106), Wellenfaktor und Schwierigkeitsgrade (B-101), Lager-Maximum und fünf Materialien (B-113), Skill-Pool je Insel (B-118), Inselwechsel und Bosse (B-103), neue Stufen (B-115).

## Regeln und Einschränkungen

**Parallel zu SP11:** SP11 (Raspberry Pi) wartet auf 🧑 (Sessions am Pi) und ist blockiert. Nach `docs/arbeitsweise.md` › Sprint-Lebenslauf (Blockade) darf der nächste unabhängige Sprint vorgezogen werden; SP12 ist von SP11 unabhängig (SIM gegen SRV/Pi). Deshalb steht `Einschiebbar: ja`. Domäne SIM: nur `engine/sim/` (und `engine/level/`, falls nötig), Tests und Daten dazu. Deterministisch (nur `engine/rng`), Komplexitäts-Budget (Datei ≤ 400 Zeilen, Funktion ≤ 60). Schichtgrenzen: `engine/sim` importiert nichts aus `room`, `net`, `cmd`. Bestehende Golden-Tests (`testdata/golden/`) bleiben unverändert grün. Fließkomma: Produkte, die in Summen gehen, mit `float64(…)` runden (siehe Paketkommentar in `world.go`).

## Beispiele

Zwei Spieler, Spieler 0 im Wald, Spieler 1 in der Höhle: Beide Stufen ticken, im Wald läuft die Nacht-Welle, in der Höhle der Aggressionspool; Spieler 1 steht 2 s am Ausgang der Höhle und ist danach allein in der Mine; Holz, das Spieler 0 einlagert, ist für Spieler 1 sofort im Vorrat.

## Ausnahme- und Fehlerfälle

Spieler fällt in einer Stufe → Respawn an der Burg dieser Stufe. Stufe ohne Spieler → läuft weiter, Hub ohne Verteidiger kann fallen (Niederlage wie bisher, Modi folgen mit B-102). Spielstand im alten Format → wird geladen und in eine Insel überführt.

## Akzeptanzkriterien

- **AC-01** Zwei Stufen ticken gleichzeitig, ein Spieler in jeder; Wellen und Zyklus laufen unabhängig weiter (B-100/AC-01).
- **AC-02** Ein Spieler wechselt einzeln über Eingang und Treppe; Reihenfolge der Spieler bleibt, andere Spieler bleiben stehen (B-100/AC-02).
- **AC-03** Gleicher Seed und gleiche Eingaben ergeben gleiche Zustände (B-100/AC-03).
- **AC-04** Ein Spielstand speichert Insel und Stufe je Spieler; ein alter Stand wird geladen (B-100/AC-04).
- **AC-05** Die Rechenzeit je Tick mit 3 aktiven Stufen und 4 Spielern ist gemessen und im Ergebnis genannt (B-100/AC-05).
- **AC-06** Das Material ist ein Vorrat je Insel (alle Stufen teilen ihn), Gold bleibt je Spieler (B-100 › Anforderungen, B-108).
- **AC-07** Die `Campaign` bleibt unverändert funktionsfähig: Raum, Protokoll und Golden-Tests laufen, `task check:go` ist grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP12.1 | `SP12.1-insel-geruest.md` | Umsetzung | autonom | fertig |
| SP12.2 | `SP12.2-einzelwechsel.md` | Umsetzung | autonom | fertig |
| SP12.3 | `SP12.3-spielstand.md` | Umsetzung | autonom | fertig |
| SP12.4 | `SP12.4-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-02, Review SP12.4 (Agent, im selben Gespräch wie die Umsetzung): AC-01, AC-03, AC-06, AC-07 belegt (SP12.1 › Ergebnis), AC-02 und AC-05 (SP12.2 › Ergebnis), AC-04 (SP12.3 › Ergebnis).
- Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: keine (Folge: B-133 Raum auf Insel, B-104 Protokoll).
- Hinweis: Tick-Dauer nur auf dem Entwickler-Rechner gemessen; Pi 3 offen (SP11).
