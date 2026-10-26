# BS3.4 · Meilenstein-Lauf coop2 standard läuft durch

- **Status:** blockiert
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** bs3/4-meilenstein-lauf
- **Abhängig von:** BS3.3
- **Tickets:** B-635, B-638
- **Kriterien:** AC-05

## Ziel

**Meilenstein 1:** Der Lauf `coop2`, Welt standard, Seed `k3c-dev` läuft mit den neuen Werten durch (Endboss Insel 1 besiegt); der Bericht liegt unter `reports/simtest-<id>/`.

## Kontext

- Lauf nur über MCP: `sim_test {action: start, bots: coop2, world: standard, days: 40}` (offline, Seed-Modus `dev`, Insel-Lauf aus BS11), dann `sim_test status`. Nie Shell oder Binaries (B-348).
- Vorher prüfen, dass die Workbench nach BS3.3 neu gestartet ist (`workbench_status`, Bericht kennt Schiff-Zeile); sonst `Status: blockiert` und 🧑 deutlich um Neustart bitten.
- Vergleich: Referenzlauf mit den alten Werten aus BS11.4 (Bericht-ID in B-642 › Notizen).
- Justieren nur in `data/` nach den Regeln aus BS2; Tuning-Hinweise des Berichts nutzen.
- „Durchlaufen“ = Endboss der Insel 1 im Insel-Lauf besiegt (`zielkorridore.md` § 6, B-642 Variante A).

## Erlaubte Dateien

- `data/*.json` (Werte, keine `seeds`/`seedMode`)
- `testdata/golden/**`
- `docs/backlog/B-635-*.md`, `B-638-*.md` (Notizen über Planungs-Tools)

## Nicht-Ziele

Mechanikänderungen; Läufe über alle Seeds (BS6).

## Schritte

1. Neustart prüfen.
2. Lauf starten, Ergebnis lesen (`report_read`).
3. Rot → Werte justieren (kleine Schritte, Regel einhalten), erneut laufen; höchstens ~5 Runden, sonst Befund-Ticket und 🧑 fragen.
4. Grüner Lauf: Bericht-ID, Endboss-Tag, Schiff-Zeile in B-635/B-638 › Notizen.
5. `check_run task:check:diff`, Commit.

## Fertig, wenn

- [ ] AC-05: Bericht `coop2` standard `k3c-dev` mit Endboss Insel 1 besiegt, Bericht-ID notiert.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Balancing nur über `sim_test`.

## Ergebnis

**Blockiert, Entscheidung 🧑 nötig (B-645).** Meilenstein-Lauf nicht grün, nach fünf Diagnose-Läufen kein Justieren der Werte sinnvoll: Das Problem liegt bei den Bots, nicht bei den Preisen.

- Läufe über `sim_test` auf der neu gestarteten Workbench (`wait: true` funktioniert): `run-1` bis `run-5` (coop2 und wirtschaft, standard, 40 Tage, Seed `k3c-dev`, 1:35 bis 3:43 min). Alle rot; Endboss 0 %.
- Messwerte (Details B-645): Hub-Stufe am Ende 1 auf jeder Stufe, Kämpfer 0, Leerlauf der Bauern 94–100 %, Burg der Startstufe fällt ab Nacht 3–5, Schiff: Kiel ab Tag 13, Rumpf und Segel nie bezahlbar. Der Endboss erscheint nur, wenn ein Monarch die Kristallhöhle erreicht.
- Während der Läufe gefunden und behoben (Commit, `check:diff` grün): Der Bericht stürzte die Oberfläche ab (`levers: null` bei neuen Zielen ohne Hinweise), Abstand bei Korridor ohne Breite, Diagnosewerte je Stufe (Boss erschienen, gefallene Monarchen, Hub-Stufe, Kämpfer), Absteige-Regel: bei mehreren Monarchen steigen nur die mit ungeradem Index ab (sonst fällt der Hub in Nacht 3).
- AC-05 offen. Nächster Schritt nach Entscheidung zu B-645 (A: `coop2` mit den Bausteinen von `wirtschaft`; B: Bot „Insel“; C: Werte).
- Neue Tickets: B-645.
