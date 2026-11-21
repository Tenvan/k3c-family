# FG1.2 · Waffenschmied, Rüstungsschmied, Bogner, Kombinationen und Elite in der Sim

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** fg1/2-meister-kombinationen
- **Abhängig von:** FG1.1
- **Tickets:** B-1200
- **Kriterien:** AC-02

## Ziel

Bogner, Waffenschmied und Rüstungsschmied fertigen Bogen, Schwert und Rüstung ins Regal; Bürger werden Bogenschütze oder Soldat, diese mit Rüstung zu Elite; der Hub-HP-Bonus der Rüstkammer entfällt.

## Kontext

Ticket B-1200. Waffenschmied ersetzt die Schmiede, Rüstungsschmied die Rüstkammer, Handwerksmeister (ECO1.3) macht Gürtel. Genau ein Beruf je Figur, der Bürger holt Ausrüstung selbst aus dem Regal. Produktion nur ganz oben (ECO1.6). Spielstand-Version erhöhen.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-data/**`
- `data/buildings.json`
- `data/troops.json`
- `data/hub.json`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Schmiede/Rüstkammer-Logik (`building/upgrades.rs`) lesen.
2. Waffenschmied und Rüstungsschmied mit Regal, Gegenstände Schwert und Rüstung.
3. Bürger + Schwert = Soldat, Soldat/Bogenschütze + Rüstung = Elite; Handwerker nimmt nichts.
4. Hub-HP-Bonus entfernen.
5. Test je Kombination.

## Fertig, wenn

- [ ] AC-02: Fünf Kombinationen mit Test je Kombination, Hub-HP-Bonus entfällt.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 geprüft: Mit nativen Regeln stellt die Schmiede (Waffenschmied) Schwärter her, die Rüstkammer (Rüstungsschmied) Rüstungen (Regal `Site::armors`, 3 Stück, 30 Eisen + 8 Gold, 30 s; Zahlen als Code-Konstanten in `unit/hammer.rs`, Balance im Abnahme-Ticket). Bürger holen Bogen, Schwert oder Werkzeuggürtel selbst (Bogenschütze, Soldat, Handwerker); Soldat oder Bogenschütze plus Rüstung ergibt Elite-Soldat bzw. Elite-Bogenschütze (`armor_convert`: nächste Figur zum Gebäude, ohne Kandidat bleibt die Rüstung im Regal); Handwerker und Bürger mit Beruf nehmen nichts; Elite-Angebot und Rüstungsstufen samt Hub-HP-Bonus entfallen. Ohne native Regeln bleibt alles wie in Go. Tests: `unit/combo_tests.rs` (je Kombination ein Test, Herstellung, 2 Spieler, Regal ohne Kandidat, kein Hub-HP-Bonus), alte Pfade mit `legacy_rules`. `cargo test -p k3c-sim -p k3c-room` grün bis auf den schon roten Insel-Takt-Test, clippy sauber.

Abweichungen: Spielstand-Version nicht erhöht (neues Feld `armors` ist optional und wird weggelassen, wenn 0). Gebäude behalten die Codes `smithy`/`armory`; neue Namen und Grafik folgen mit FG1.3. Die Rüstung wird beim Fertigwerden angewandt, ohne dass die Figur zum Regal läuft. Rot schon vor dieser Session: `alloc_je_bild`, `e2e_flow` (seit mindestens ECO1.3), `still_hud` (Bildabweichung).
