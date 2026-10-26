# BS3.2 · Einnahmen, Beute, Ziele und Schiffskosten auf der Münzskala

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bs3/2-einnahmen-schiffskosten
- **Abhängig von:** BS3.1, BS2.2
- **Tickets:** B-635, B-638
- **Kriterien:** AC-02, AC-03

## Ziel

Einnahmen, Beute, Markieren, Ziele und Zielkorridore folgen der Münzskala, und die Schiffskosten je Abschnitt stehen in `data/`.

## Kontext

- Regeln: `docs/rules/wirtschaft.md`, `zielkorridore.md` (BS2), Schiff in `stufen.md`/`wirtschaft.md` (BS2.2).
- Ist: `data/economy.json` (Einnahmen, `markCost`), `data/bosses.json` (Beute bis 500), `data/goals.json` (1000), `data/events.json`, `data/ruins.json`, `data/balance-targets.json` (Korridore).
- Ausnahmen (Events, geplante Chunks, Bosse, Ruinen) proportional mitziehen, `notes`: „vorläufig, eigenes Balancing später“.
- Schiff: Vorschlag Eintrag `ship` mit `stages` (Kiel, Rumpf, Segel; Münzen + Material). Die Sim kennt das Schiff erst in BS5 – die Daten dürfen die Validierung (`engine/sim/data.go`, `-check-data`) nicht brechen; notfalls eigene Datei `data/ship.json`, die nur der Bericht liest.

## Erlaubte Dateien

- `data/economy.json`, `data/bosses.json`, `data/goals.json`, `data/events.json`, `data/ruins.json`, `data/balance-targets.json` (nur Zielwerte, nicht `seeds`/`seedMode`), `data/ship.json` (neu) oder `data/buildings.json` › `ship`
- `engine/sim/*_data_test.go`, `data/embed.go`/`data/embed_test.go` (falls neue Datei)
- `testdata/golden/**`

## Nicht-Ziele

Schiff-Mechanik (BS5), Bericht (BS3.3).

## Schritte

1. Einnahmen-Verhältnis aus der Regel anwenden (Bürger, Beute, Markieren).
2. Ausnahmen proportional, `notes` markieren.
3. Korridore in `balance-targets.json` umrechnen.
4. Schiffskosten anlegen, Daten-Test (Skala, Schema).
5. Golden, `check_run task:check:diff`, Commit.

## Fertig, wenn

- [ ] AC-02: Einnahmen, Beute, Ziele und Korridore umgerechnet; Tests grün.
- [ ] AC-03: Schiffskosten je Abschnitt in `data/`, Daten-Test grün.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
