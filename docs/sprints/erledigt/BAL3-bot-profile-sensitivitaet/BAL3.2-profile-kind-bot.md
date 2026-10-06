# BAL3.2 · Vier Profile ohne Kind-Bot

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** bal3/2-profile-kind-bot
- **Abhängig von:** BAL3.1
- **Tickets:** B-158
- **Kriterien:** AC-01, AC-02

## Ziel

Der Tester kennt die Profile „Wirtschaft zuerst“, „Mauern zuerst“, „Koop 2 Spieler“ und „Koop 4 Spieler“; der Kind-Bot ist nach der Spec-Prüfung 2026-10-04 nicht Pflicht (Q19, später; B-158/AC-02) und entsteht hier nicht.

## Kontext

- **Beschluss aus BAL3.1** (Profile, Fehlerarten, Fehlerraten) ist die Quelle; im Code und in den Daten stehen nur diese Zahlen, keine eigenen. Fehlt ein Beschluss, Session `blockiert` und Ticket, nicht raten.
- **Entsteht in BAL1:** Paket `engine/balance` mit Läufer, Profil-Schnittstelle, den Profilen „passiv“ und „sparsam“ und den Kennzahlen (`BAL1.1-bots-kennzahlen.md`; Ort vor dem Start im Code nachsehen, Befehl `cmd/k3c-balance`, Task `balance:run`). **Entsteht in BAL2:** Korridor-Prüfung und `task balance` (`BAL2.1` bis `BAL2.3`).
- **Bots nur über `PlayerCommand`** (`engine/sim/types.go`: `MoveX`, `Sprint`, `Pay`, nach S1 auch Schlag und Skill), sie lesen nur den Zustand; Zufall nur über `engine/rng` (`rng.New(seed)`), nie `math/rand`; keine Wanduhr, keine Map-Iteration ohne Sortierung (Check aus F2/B-138).
- **Mehrspieler:** Ein Koop-Profil steuert alle Monarchen des Szenarios; fordert ein Profil mehr Spieler, als das Szenario hat, ist das ein Fehler beim Start (B-158 › Ausnahmefälle). Jede Regel gilt für 2+ Spieler.
- **Daten:** Fehlerwahrscheinlichkeiten stehen in `data/` (Datei und Format legt die Session fest; Vorschlag: neue Datei `data/balance-bots.json`; neue Felder sind SIM, die Werte kommen aus dem Beschluss BAL3.1). Wird die Datei eingebettet, `data/embed.go` und `data/embed_test.go` anpassen.
- Fehlt eine Quelle im Zustand, Kennzahl als `null` und Ticket (SIM), nicht im Sim-Code nachrüsten.

## Erlaubte Dateien

- `tools/k3c-dev/internal/balance/` (Profile, Tests; `cmd/` nur Profil-Auswahl)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Sensitivität und Kurven (BAL3.3), lernende Bots, Änderung bestehender Werte in `data/`, Änderung von `engine/sim/`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Beschluss BAL3.1 und Profil-Schnittstelle aus BAL1 nachlesen.
2. Vier Profile (Wirtschaft zuerst, Mauern zuerst, Koop 2, Koop 4) nach der Profil-Schnittstelle; Fehler beim Start, wenn das Szenario zu wenige Spieler hat.
3. Entfällt (Kind-Bot nicht Pflicht, Q19): keine Fehler-Daten, kein Kind-Bot.
4. Tests: je Profil gleicher Seed und gleiche Daten → byte-gleiche Kennzahlen; Profil mit zu wenigen Spielern → Fehler.
5. `task check:dev`. Ergebnis mit Vergleich „Mauern zuerst“ gegen „sparsam“ (Überleben Welle 3, 100 Seeds, 2 Spieler), `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: Test belegt für jedes der vier neuen Profile byte-gleiche Kennzahlen bei gleichem Seed und gleichen Daten.
- [x] AC-02: Kein Kind-Bot, keine Fehler-Daten in `data/` (Q19, später).
- [x] `task check:dev` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check:dev
```

## Ergebnis

- **AC-01 umgesetzt und geprüft:** Profile `walls`, `economy`, `coop2`, `coop4` in `tools/k3c-dev/internal/balance/bots.go`
  (Verhalten nach dem Beschluss in `README.md`; `coop*`: gerader `p.Index` → `walls`, ungerader → `economy`). Mindest-Spieler
  `minPlayers`/`checkPlayers`: Fehler „profil X braucht N Spieler, Szenario hat M“ in `Matrix.Scenarios` und `Target.validate`.
  Tests in `profiles_test.go`: `TestNeueProfileGleicheBytes` (je Profil zwei Läufe, Seed 1, 1 Tag, 2 bzw. 4 Spieler →
  `Report.JSON()` byte-gleich und gültig), `TestProfilZuWenigeSpieler` (coop2 mit 1, coop4 mit 2 und 3 Spielern → Fehler,
  Matrix und Ziel), `TestMauernUndWirtschaftZuerst` (`walls` baut an Tag 1 eine Mauer, `economy` zahlt an Tag 1 ohne Mauer).
- **AC-02 geprüft:** kein Kind-Bot, kein Fehler-Profil, keine Datei und kein Wert in `data/` geändert.
- **Mauer-Ausbau:** aus öffentlichen Feldern nachgebildet (`Site.State/Level/Upgrade/UpgradePaid`, `World.HubLevel`,
  `buildings.json` › `wall.levels`), da `sim.upgradePayable` nicht exportiert ist. Mauer-Stufe 2 braucht Hub-Stufe 2, die kein
  Profil zahlt; der Ausbau greift in den Läufen daher nie.
- **Vergleich „Mauern zuerst“ gegen „sparsam“** (`task balance:run -- --seeds 100 --bots walls,saver --players 2 --days 5`,
  Wald, 100 Seeds, alle gültig): Welle 3 überlebt `walls` 97/100, `saver` 97/100; Burg hält bis Tag 6 je 94/100. Die
  Kennzahlen sind je Seed identisch, weil `walls` ohne erreichbaren Ausbau dasselbe tut wie `saver`.
- **Laufzeit:** 1 min 48 s für 200 Läufe à 5 Tage (inkl. Build); Tests des Pakets ≈ 6 s.
- `task check:dev` grün (0 Lint-Meldungen); größte Datei `bots.go` 210 Zeilen.
