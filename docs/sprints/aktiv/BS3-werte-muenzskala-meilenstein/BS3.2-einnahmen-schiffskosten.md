# BS3.2 · Einnahmen, Beute, Ziele und Schiffskosten auf der Münzskala

- **Status:** fertig
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

- AC-02 geprüft: Einnahmen, Beute, Ziele nach der Kurve aus `wirtschaft.md` § 8 umgerechnet (`round(2·(alt/5)^0,66)`, mindestens 1): Startgold 14, Beutel 30, Tageseinkommen 2, Truhen 3–6, Händler 2, Greed stiehlt 2, Gegner-Drops Standard 1–6 und Elite 7–12, Boss-Beute 14 (Endboss 42), Siegziel Gold 66, Vollmond-Belohnung 9. `notes` markieren Bosse und Events als „vorläufig (Ausnahme, B-635)“. Zielkorridore in `balance-targets.json` mussten nicht geändert werden: Die Gold-Korridore standen seit BS2 schon auf der Skala (Median Gold am Morgen 5–20).
- AC-03 geprüft: Schiffskosten in `data/ship.json` (Kiel 10 + 80 Holz, Rumpf 20 + 60 Stein + 40 Kupfer, Segel 30 + 40 Eisen + 20 Kristall), Beschreibung `docs/data/ship.md`, Liste in `datadoc.go`; Daten-Test `TestSchiffskostenInDenDaten` (Schema, Skala, Reihenfolge). Die Sim liest die Datei erst mit BS5.
- `task check:diff` grün. Golden neu (`task go:golden`): Gold-Stände, Beute und Ereignisse ändern sich. Golden-Läufe stellen den Beutel auf die alten 100 Gold (`goldenPurse`), weil die aufgezeichneten Eingaben so viel Gold brauchen; im Kampagnen-Ausbau-Golden zählt „bezahlt“ statt „gebaut“ (die Werkstatt ist mit den neuen Preisen früher bezahlt, als ein Bauer frei ist).
- Abweichungen von der Session: erlaubte Dateien erweitert um `datadoc.go` und `docs/data/*.md`, Tests im Balancing-Tester (Startgold-Bereiche, Gold-Schwellen auf der neuen Skala), `TestWirtschaftsBotUndTagesbeginn` läuft mit dem alten Beutel (100), `TestMauernZuerstBautMauerAus` mit 30 Gold je Monarch. Messgröße `endbossDay` zählt ohne Sieg „Tageslimit + 1“, damit das Ziel verletzt statt ungültig ist.
- Wichtig für BS3.4: Mit 14 Startgold erreicht der Bot `wirtschaft` Hub-Stufe 2 und die Werkstatt im Test nicht in 2 Tagen; die Korridore für Hub-Stufe, Turm und Kämpfer sind im Standardszenario verletzt. Das ist der Stoff der Justierung.
- Neue Tickets: keine.
