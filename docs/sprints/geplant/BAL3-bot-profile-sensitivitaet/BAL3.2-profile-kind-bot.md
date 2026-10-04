# BAL3.2 · Fünf Profile inklusive Kind-Bot mit Fehlern aus Daten

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** bal3/2-profile-kind-bot
- **Abhängig von:** BAL3.1
- **Tickets:** B-158
- **Kriterien:** AC-01, AC-02

## Ziel

Der Tester kennt die Profile „Wirtschaft zuerst“, „Mauern zuerst“, „Koop 2 Spieler“, „Koop 4 Spieler“ und „Kind-Bot“; der Kind-Bot macht Fehler mit Wahrscheinlichkeiten aus `data/`, deterministisch je Seed.

## Kontext

- **Beschluss aus BAL3.1** (Profile, Fehlerarten, Fehlerraten) ist die Quelle; im Code und in den Daten stehen nur diese Zahlen, keine eigenen. Fehlt ein Beschluss, Session `blockiert` und Ticket, nicht raten.
- **Entsteht in BAL1:** Paket `engine/balance` mit Läufer, Profil-Schnittstelle, den Profilen „passiv“ und „sparsam“ und den Kennzahlen (`BAL1.1-bots-kennzahlen.md`; Ort vor dem Start im Code nachsehen, Befehl `cmd/k3c-balance`, Task `balance:run`). **Entsteht in BAL2:** Korridor-Prüfung und `task balance` (`BAL2.1` bis `BAL2.3`).
- **Bots nur über `PlayerCommand`** (`engine/sim/types.go`: `MoveX`, `Sprint`, `Pay`, nach S1 auch Schlag und Skill), sie lesen nur den Zustand; Zufall nur über `engine/rng` (`rng.New(seed)`), nie `math/rand`; keine Wanduhr, keine Map-Iteration ohne Sortierung (Check aus F2/B-138).
- **Mehrspieler:** Ein Koop-Profil steuert alle Monarchen des Szenarios; fordert ein Profil mehr Spieler, als das Szenario hat, ist das ein Fehler beim Start (B-158 › Ausnahmefälle). Jede Regel gilt für 2+ Spieler.
- **Daten:** Fehlerwahrscheinlichkeiten stehen in `data/` (Datei und Format legt die Session fest; Vorschlag: neue Datei `data/balance-bots.json`; neue Felder sind SIM, die Werte kommen aus dem Beschluss BAL3.1). Wird die Datei eingebettet, `data/embed.go` und `data/embed_test.go` anpassen.
- Fehlt eine Quelle im Zustand, Kennzahl als `null` und Ticket (SIM), nicht im Sim-Code nachrüsten.

## Erlaubte Dateien

- `engine/balance/` (Profile, Tests), `cmd/k3c-balance/` (nur Profil-Auswahl)
- `data/` (nur neue Fehler-Daten für den Kind-Bot, Embed-Anpassung)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Sensitivität und Kurven (BAL3.3), lernende Bots, Änderung bestehender Werte in `data/`, Änderung von `engine/sim/`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Beschluss BAL3.1 und Profil-Schnittstelle aus BAL1 nachlesen.
2. Vier Profile (Wirtschaft zuerst, Mauern zuerst, Koop 2, Koop 4) nach der Profil-Schnittstelle; Fehler beim Start, wenn das Szenario zu wenige Spieler hat.
3. Fehler-Daten anlegen, Lader, Kind-Bot mit Fehlern über `engine/rng`.
4. Tests: je Profil gleicher Seed und gleiche Daten → byte-gleiche Kennzahlen; Kind-Bot mit einem Seed zweimal gerechnet → dieselbe Fehlerliste; Wahrscheinlichkeiten kommen aus den Daten (andere Daten in der Test-Option ändern das Verhalten); Profil mit zu wenigen Spielern → Fehler.
5. `task check:go`. Ergebnis mit Vergleich Kind-Bot gegen „sparsam“ (Überleben Welle 3, 100 Seeds, 2 Spieler; Erwartung nach B-158 › Beispiele: niedriger), `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: Test belegt für jedes der fünf neuen Profile byte-gleiche Kennzahlen bei gleichem Seed und gleichen Daten.
- [ ] AC-02: Test belegt dieselben Fehler des Kind-Bots je Seed; die Wahrscheinlichkeiten stehen in `data/`.
- [ ] `task check:go` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check:go
```

## Ergebnis

–
