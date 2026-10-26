# BS3.4 · Meilenstein-Lauf coop2 standard läuft durch

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
