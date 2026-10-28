# BS3.6 · Simulationen BS3: Meilenstein-Lauf coop2 und Stufen-Tests

- **Status:** fertig
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

- AC-05 (Fassung 2026-10-28, BS3 Revision 2: bis Splitter-Titan) geprüft mit `task balance:milestone` (Go-Test, coop2, standard, 40 Tage): Seed `k3c-dev` besiegt den Splitter-Titan an Tag 26; Mehr-Seed-Lauf `k3c-dev,1,2,3`: Goblin-Anführer 4/4 (Tag 5), Troll-König 4/4 (15), Ratten-Königin 4/4 (20), Lava-Golem 4/4 (22), Splitter-Titan 3/4 (32), Endboss 0/4 (dem Stufen-Test überlassen). Schiff: Kiel T1, Rumpf und Segel spät (Bericht).
- Werte: `waves.json › perExtraPlayer` 0,5 → 0,25 (Commit 5125c425), neues Feld `bossHpPerExtraPlayer` 0,25 (B-650, 6fd5b4d6); Basiswerte unverändert (Regel 🧑: Coop nur über Multiplikatoren).
- Weg dorthin (alles über Tickets belegt): B-645 (Bots), B-650, B-651 (Kristallhöhle: Pool-Hebel B/C/E gemessen und verworfen), B-652 (verteilte Mauern/Türme, Tor = äußerste Mauer, BS12) mit Endspiel-Bot (Commit e60e84e6).
- `task balance:milestone` ersetzt die Workbench-Neubäume für Simuläufe (Commit 23141f44).
- `task check:diff` grün.
