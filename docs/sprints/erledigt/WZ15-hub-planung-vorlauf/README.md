# WZ15 · REG, INF, SIM · Hub-Planung Vorlauf: Regeltabelle erzeugt, Prüfung ohne globalen Zustand

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** REG, INF, SIM
- **Reife:** bereit
- **Tickets:** B-462
- **Start-Commit:** f6912198
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Ralf im Chat (Revision 1)

## Ausgangslage

Abgespalten aus WZ9 (Entscheidung 🧑 2026-10-17, wegen 8 Sessions über dem Budget von 3–6): Die Sessions, die nicht von WZ7 abhängen, laufen als Vorlauf. B-462 › Ausgangslage: Die Freischaltung je Hub-Stufe steht in `data/hub.json` und wird an drei Stellen von Hand gepflegt (JSON, Regeltabelle § 3, Test-Kopie `hubLevelRule`); die Platz-Regeln gibt es nur als Go-Tests in `engine/sim/site_data_test.go`. Eine Funktion, die eine geänderte `hub.json` prüft, fehlt. Der Gebäude-Planer (WZ9) braucht sie, den Generator der Tabelle und die Linien als Parameter.

## Ziel

Die Regeltabelle „Hub-Stufe → Gebäude“ in § 3 wird aus `data/hub.json` erzeugt (Go-Generator, Task `rules:update`, Test Block = JSON), und `engine/sim` bietet eine exportierte Prüfung einer Kandidaten-`hub.json` ohne globalen Zustand, mit Linien als Parameter in `engine/level`; keine feste 5 im Code.

Am Ende sichtbar: im Terminal `task rules:update` ohne Diff, grüne Go-Tests (Block = JSON, Prüfung, 7-Stufen-Fixture), `testdata/golden/` unverändert.

## Beteiligte und Zielgruppen

B-462 › Beteiligte und Zielgruppen. Im Sprint: Agenten setzen WZ15.1–WZ15.4 um (REG, INF, SIM); 🧑 gibt die Spec frei. Nutzer des Ergebnisses ist WZ9 (Gebäude-Planer) und später B-458.

## Anforderungen

B-462 › Anforderungen (Regeltabelle erzeugt, Prüfung über das Go-Backend, Erweiterbar, Deterministisch). Sprint-eigen:
- **Reihenfolge:** Erst legt REG das Zielformat der Tabelle von Hand fest (WZ15.1), dann erzeugt INF genau diesen Block (WZ15.2), dann stellt SIM die Prüfung bereit (WZ15.3), dann folgt das Review (WZ15.4).
- **Generator in Go:** `tools/regeltabelle` (Task, EXE `bin/regeltabelle`) und das Paket `tools/regeltabelle/tabelle` mit `Block`, `Ersetzen` und Gegenprobe (reine Funktionen); WZ9 importiert sie im Studio, hier schreibt nur der Task.
- **Kandidat ohne globalen Zustand:** Die Prüfung nimmt die Kandidaten-`hub.json` als Parameter; kein `sim.UseData`, keine globale Umschaltung.
- **Linien als Parameter in `engine/level` (darf WZ15.3 ändern):** Die Kandidaten-Linien erreichen Generator und Planungs-Prüfung nur über Parameter (Standard eingebettet). Hat WZ8 sie angelegt (WZ8.2, WZ8.3), baut WZ15.3 darauf auf; sonst legt WZ15.3 sie an (Zuschnitt wie B-461 › Anforderungen). `sim.CreateOptions` ist eine neue Struktur nur für die Erzeugung, getrennt vom vorhandenen `sim.IslandOptions` (im Spielstand gespeichert); sie wird nicht gespeichert, Spielstand-Format und `IslandSaveVersion` bleiben unverändert. Hat WZ8.3 sie angelegt, ergänzt WZ15.3 den Kandidaten als Feld.
- **Geplante Chunks (B-461, WZ8):** Die Prüfung prüft sie mit, sofern WZ8 erledigt ist, gegen die Kandidaten-Linien; sonst Hinweis „geplante Chunks nicht geprüft“ und ein Ticket.
- **Plätze und Pflichtplätze (B-462 Entscheidung 5, Frage 12):** nur bekannte `kind`-Arten; Pflichtplätze Werkstatt, Treppe runter, Treppe hoch, Farm je Seite (Abnahmepunkt für 🧑 in WZ9); Regel „ein Platz je Bau“ geteilt (Pflicht genau einer, optional höchstens einer).
- **Kein Datenwechsel im Sprint:** `data/hub.json` und `testdata/golden/` bleiben unverändert; die Regeltabelle ändert nur WZ15.1 (Form, Inhalt = JSON). Tests arbeiten auf Kopien. Die 7-Stufen-Fixture liegt unter `testdata/hub/`, ihre Werte gibt der Sprint nicht vor, sie muss die Prüfung bestehen.

## Nicht-Ziele

Der Gebäude-Planer im Asset-Studio, die Snapshot-Option (WZ9), Werte und Kosten (`data/buildings.json`, `levels`, REG), neue Gebäudearten und Felder (SIM), mehr Hub-Stufen selbst (B-458), Chunk-Planung (B-461, WZ8), Golden-Update im Sprint.

## Regeln und Einschränkungen

- **Domänen nacheinander:** REG (WZ15.1: nur `docs/rules/materialien-gebaeude.md` § 3) → INF (WZ15.2: `Taskfile.yml`, `tools/regeltabelle/`, `tests/projectRules.test.ts`, `testdata/hub/`) → SIM (WZ15.3: `engine/sim/`, bei Bedarf `engine/level/`, keine Golden-Änderung). Die Review-Session trägt die Domäne der letzten Umsetzung (SIM) und prüft den Diff aller drei Domänen.
- **Schichten:** `engine/level` importiert weiter nichts aus `engine/sim`, `engine/room`, `engine/net`; die Golden-Daten bleiben byte-gleich (Standard-Linien = heutiges Verhalten).
- Keine neue Abhängigkeit; Datei ≤ 400, Funktion ≤ 60 Zeilen; Firewall-Regel des Asset-Studios (B-459): Generator nur als gebaute EXE.
- Glossar: WZ15 trägt keinen Begriff ein.

## Beispiele

B-462 › Beispiele. Sprint-eigen:
- Nach WZ15.2: `task rules:update` auf `develop`-Stand → kein Diff in `docs/rules/materialien-gebaeude.md`; Go-Test und Marker-Test grün.
- Nach WZ15.3: `go test ./engine/sim` grün, `testdata/golden/` unverändert, `hubLevelRule` nicht mehr im Code; die 7-Stufen-Fixture besteht die Prüfung; Heilplatz entfernen → kein Befund, Werkstatt entfernen → Befund.

## Ausnahme- und Fehlerfälle

- WZ15.1 findet eine Abweichung zwischen Tabelle und `data/hub.json` → nicht still angleichen: 🧑 entscheidet, welche Seite gilt; bis dahin `blockiert`.
- WZ15.3 lässt einen Golden-Test brechen → Fehler im Umbau, kein `task golden:update`.
- WZ8 noch nicht erledigt (WZ15.3 an der Reihe) → die Prüfung lässt geplante Chunks aus, Ticket zur Nachrüstung; der Sprint läuft weiter.
- WZ15.3 sprengt das Budget (Variante der Sim-Funktionen ohne globalen Zustand) → Session teilen (Prüfung / Bau aus Kandidat), Kriterien nicht umformulieren.

## Akzeptanzkriterien

- **AC-01** `B-462/AC-07` Regeltabelle in § 3 aus `data/hub.json` erzeugt (Go-Generator, Task `rules:update`, Funktionen im Paket `tools/regeltabelle/tabelle`), zweiter Lauf byte-gleich, Go-Test Block = JSON, Marker-Test.
- **AC-02** `B-462/AC-02` (Prüfung) Eine Funktion in `engine/sim` prüft eine Kandidaten-`hub.json` ohne globalen Zustand mit den Regeln aus `site_data_test.go` (eine Quelle der Regeln): Platz eindeutig, Abstand zu Zahlzielen bei voller Streuung, Treppen-Abstand, Portal-Mindestabstand, Stufe im Bereich, nur bekannte Arten, Pflichtplätze, geteilte Regel „ein Platz je Bau“; mit WZ8 zusätzlich geplante Chunks gegen die Kandidaten-Linien (Linien als Parameter in `engine/level`); `hubLevelRule` entfällt.
- **AC-03** `B-462/AC-08` (Prüfung, Generator) Mehr Hub-Stufen ohne Umbau: Eine Fixture mit 7 `levels` unter `testdata/hub/` besteht die Prüfung, ohne vorgegebene Werte; Prüfung und Generator zeigen 7 Stufen (Tests), keine feste 5 im Code.
- **AC-04** Der Sprint ändert weder `data/hub.json` noch `testdata/golden/`: `git diff --stat origin/develop -- data/hub.json testdata/golden` ist leer.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): B-462 › Offene Fragen 1–4, 5, 6, 8, 9, 10 (siehe Ticket); Zuschnitt: REG, INF und SIM als Vorlauf-Sprint WZ15, damit WZ9 unter dem Budget bleibt.

Aus dem Code festgelegt, 🧑 bestätigt bei der Abnahme in WZ9: Pflichtplätze (B-462 › Offene Fragen 12).

Offen: keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ15.1 | `WZ15.1-regeltabelle-form.md` | Umsetzung | autonom | fertig |
| WZ15.2 | `WZ15.2-regeltabelle-generator.md` | Umsetzung | autonom | fertig |
| WZ15.3 | `WZ15.3-pruefung-kandidat.md` | Umsetzung | autonom | fertig |
| WZ15.4 | `WZ15.4-review.md` | Review | autonom | fertig |

- WZ15.1 (REG) Regeltabelle „Hub-Stufe → Gebäude“ in § 3 auf die erzeugbare Form umstellen: Marker, Spalten, Inhalt = heutige `data/hub.json`, Abgleich ohne Abweichung (AC-01, AC-04).
- WZ15.2 (INF) Generator in Go (`tools/regeltabelle`, Funktionen `Block`, `Ersetzen`, Gegenprobe in `tools/regeltabelle/tabelle`) und Task `rules:update`; Go-Test Block = JSON, Marker-Test, 7-Stufen-Fixture unter `testdata/hub/` (AC-01, AC-03, AC-04).
- WZ15.3 (SIM) Exportierte Prüfung einer Kandidaten-`hub.json` ohne globalen Zustand, nur bekannte Arten, Pflichtplätze (Werkstatt, Treppe runter, Treppe hoch, Farm je Seite), geteilte Regel „ein Platz je Bau“, mit WZ8 geplante Chunks; Linien als Parameter in `engine/level`; `sim.CreateOptions` mit Kandidat; `hubLevelRule` entfernt; Fixture besteht die Prüfung (AC-02, AC-03, AC-04).
- WZ15.4 (SIM) Review: Kriterien, Diff REG, INF, SIM, Begriffs-Treue, PR (alle).

Domänen-Reihenfolge: REG → INF → SIM.

## Abnahme

2026-10-18, Review WZ15.4 (autonom, getrennte Reviewer): AC-01 bis AC-04 geprüft (`task:check`, `task:check:go`, `go:lint` grün, Golden-Diff leer).
Behoben: Befunde der Reviews von WZ15.1 bis WZ15.3 (u. a. Tor Linie 1, float64/hubLevel im Generator, Kandidaten-Kopie, Linienzahl ≤ Hub-Stufen). Neue Tickets: B-479 (geplante Chunks nach WZ8 prüfen, `CreateOptions` vereinen).
Version: v– vorgeschlagen (Vorlauf für WZ9, kein Spielinhalt).
