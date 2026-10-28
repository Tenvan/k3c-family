# BS3.6 · Simulationen BS3: Meilenstein-Lauf coop2 und Stufen-Tests

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** bs3/6-simulationen
- **Abhängig von:** BS3.3
- **Tickets:** B-635, B-638, B-645
- **Kriterien:** AC-05

## Ziel

Alle Simulationen von BS3 laufen hier, nicht in Umsetzungs-Sessions: Stufen-Tests (`task balance:fullgame`) und der Meilenstein-Lauf `coop2`, Welt standard, Seed `k3c-dev` (AC-05). Ergebnis: Berichte mit Bericht-IDs, rote Befunde als Tickets.

## Kontext

Regel von 🧑 (2026-10-28): Umsetzungs-Sessions fahren keine Simulationen, nur Code. Pro Sprint gibt es eine eigene autonome Sim-Session vor der Abnahme. Übernimmt die Lauf-Schritte aus BS3.4 (Schritte 1–4) und den Diagnosestand aus B-645 › Notizen. Läufe nur über `sim_test` (B-348), lange Läufe blockierend (`wait`).

## Erlaubte Dateien

- `data/*.json` (Werte, keine `seeds`/`seedMode`), nur nach den Regeln aus BS2
- `testdata/golden/**` (nur mit Begründung im Commit)
- `docs/backlog/B-635-*.md`, `B-638-*.md`, `B-645-*.md` (Notizen über Planungs-Tools)
- Befunde als Ticket, keine Code- oder Mechanikänderung

## Nicht-Ziele

Code- und Mechanikänderungen; Wert-Justierung nur in `data/` nach BS2-Regeln, höchstens ~5 Runden, sonst Befund-Ticket und 🧑 fragen.

## Schritte

1. `workbench_status`: Workbench aktuell (nach BS3.3 neu gestartet, Bericht kennt die Schiff-Zeile)? Sonst `Status: blockiert` und 🧑 um Neustart bitten.
2. Stand von B-645 › Notizen lesen (Diagnose, Bot-Korrekturen, `dawnGoldPerDepth` 6, Camp-Platzierung).
3. Stufen-Tests nur über `sim_test`/`task balance:fullgame` nach dem Stand der Notizen, je Stufe den aktuellen: offen ist die Kristallhöhle (0 Kämpfer, Endboss fällt ohne Hilfen nicht); danach ohne Hilfen, dann `coop2`.
4. Meilenstein-Lauf: `sim_test {action: start, bots: coop2, world: standard, days: 40, wait: true}` (Seed-Modus `dev`), Ergebnis mit `report_read` lesen.
5. Rot → Werte in `data/` in kleinen Schritten justieren (Regeln aus BS2, Tuning-Hinweise des Berichts), erneut laufen; höchstens ~5 Runden, sonst Befund-Ticket und 🧑 fragen.
6. Grüner Lauf: Bericht-ID, Endboss-Tag und Schiff-Zeile in B-635/B-638 › Notizen; BS3.4 AC-05 darauf verweisen.
7. `check_run task:check:diff`, Commit.

## Fertig, wenn

- [ ] AC-05: Bericht `coop2` standard `k3c-dev` mit Endboss Insel 1 besiegt, Bericht-ID notiert (Zeile „Schiff (Bedarf): … bezahlbar ab Tag …“ vorhanden).
- [ ] Bleibt der Lauf rot: Befund-Ticket und 🧑 gefragt, Stand in B-645 › Notizen.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

**Zwischenstand 2026-10-28 (Claude), AC-05 offen, Session nicht fertig.**
- Neu: `task balance:milestone` (Go-Test `TestMeilensteinCoop2`, `K3C_VARY`, `K3C_SEEDS`), derselbe Lauf wie `sim_test coop2 standard 40` ohne Workbench-Neubau (Commit 23141f44).
- Regel von 🧑: Coop nur über Multiplikator-Felder justieren, Basiswerte bleiben Solo-Werte.
- Ausgang (data/ unverändert, `perExtraPlayer` 0,5): stecken nach Stufe 2 (Tag 6), Troll-König fällt nie, Schiff Kiel T1, Rumpf T16,5, Segel T28.
- Multiplikator-Runde `waves.json › perExtraPlayer` (wirkt auf Wellengröße und Boss-HP je Zusatzspieler), Seed k3c-dev, nur im Speicher:

| perExtraPlayer | Ergebnis (coop2, 40 Tage) |
|---|---|
| 0 | schlechter: Burg fällt, Hub 1, 0 Kämpfer, nur Troll-König |
| 0,1 | Stufe 3 (Tag 15), Ratten-Königin fällt nicht |
| 0,15 | 4 Minibosse bis Tag 22, Stufe 5, Splitter-Titan fällt nicht |
| 0,2 | Stufe 4 (Tag 21), Lava-Golem fällt nicht |
| 0,25 | wie 0,15: Troll-König T14, Ratten-Königin T18, Lava-Golem T22, Stufe 5, Splitter-Titan fällt nicht |
| 0,3 | Stufe 3 (Tag 15), Ratten-Königin fällt nicht |
| 0,5 (Ist) | Stufe 2, Troll-König fällt nicht |

- Befund: Die Kurve ist nicht monoton (ein Seed, Bot-Zufall durch Geometrie); 0,15 bis 0,25 tragen bis Stufe 5, der Endboss wird mit keinem Wert erreicht. Der Splitter-Titan (Stufe 5, Welle 3) räumt das Heer ab, `perExtraPlayer` allein löst das nicht.
- Nicht übernommen: `data/` ist unverändert (Golden und Zielkorridore hängen an dem Wert, Entscheidung 🧑).
- Nächster Schritt: Entscheidung 🧑, `perExtraPlayer` auf 0,25 (Golden neu, Begründung im Commit); danach Splitter-Titan (Bot oder getrennter Boss-Faktor je Spieler, eigenes Feld statt Wellenfaktor).
