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

**Blockiert/in Arbeit, Richtungswechsel von 🧑 (2026-10-26): erst Single-Player, erst Durchspielbarkeit sichern, Werte für coop2 später.**

- Entscheidung B-645 (A): coop2 nutzt je Rolle die Bausteine von wirtschaft; sim_test offline mit players und vary (Werte nur für den Lauf).
- Neu (Vorschlag 🧑): Stufen-Tests für ein komplettes Spiel mit einem Bot im Balance-Paket, Build-Tag fullgame, nicht in check:diff: task balance:fullgame STUFE=n (Datei fullgame_test.go, Taskfile balance). Stufe 1 Miniboss Wald ✅ (Tag 5), Stufe 2 tiefste Stufe erreicht ✅ (Tag 3), Stufe 3 alle vier Minibosse (aktuell 2 von 4: Troll-König ✅, Ratten-Königin ✗), Stufe 4 Endboss (offen). Sie laufen mit Dev-Hilfen (Dev-Baukosten, voller Vorrat, Grad dev); K3C_FULLGAME=ohne spielt ohne.
- Bot-Regel gelernt: Der Bot darf eine Stufe erst verlassen, wenn ihr Miniboss besiegt ist (sonst fällt der Hub). Mit dem Tor baut er Hub-Stufe 5 und 10 Kämpfer in Wald, Höhle und Mine.
- Nächste Schritte (je Stufe nur den aktuellen Test fahren): Stufe 3 (Ratten-Königin), dann Lava-Golem und Endboss; danach ohne Hilfen, dann coop2 mit Werten.
- AC-05 weiter offen.
