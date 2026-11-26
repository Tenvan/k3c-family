# ECO1.4 · Handwerker mit Gürtel macht alle Bauarbeiten

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/4-handwerker-bauarbeiten
- **Abhängig von:** ECO1.3
- **Tickets:** B-1194
- **Kriterien:** AC-04

## Ziel

Ein Bürger mit Werkzeuggürtel ist Handwerker und macht Bau, Ausbau, Reparatur, Rodung und Fels; ohne Gürtel macht er davon nichts.

## Kontext

Ticket B-1194, Gürtel aus ECO1.3. Berufe Bergmann und Baumeister gehen im Handwerker auf (Default). Fällt der Handwerker, bleibt der Gürtel liegen; ein Bürger ohne Gürtel holt ihn nur ohne Gegner in 8 Units (B-929). Deterministisch, 2+ Spieler, Spielstand-Version erhöhen.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/unit/**`
- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-sim/src/state/**`
- `data/troops.json`
- `docs/rules/buerger.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Job-Zuteilung (`unit/jobs.rs`, Hammer-Arbeit) lesen.
2. Gürtel als Voraussetzung für Bau, Reparatur, Rodung, Fels.
3. Liegenbleiben und Aufheben des Gürtels.
4. Tests je Arbeit mit und ohne Gürtel.

## Fertig, wenn

- [ ] AC-04: Ein Bürger mit Gürtel macht Bau, Reparatur, Rodung und Fels; ohne Gürtel nichts davon.
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

AC-04 umgesetzt und geprüft: Mit nativen Regeln (`WorldValues::native_rules`) arbeitet ein Bauer nur mit Werkzeuggürtel (`Troop::hammer`) am Gelände: `unit/jobs.rs › may_gather` (Rodung und Fels/Adern); Bau und Reparatur verlangten den Gürtel schon (`build_job`, `repair_job`). Ausnahme: Bergmann (Beruf `miner`) an Fels und Adern. Ohne native Regeln sammelt jeder Bauer (Go-Parität, Golden-Läufe laufen mit `native_rules=false`). Fällt der Handwerker, bleibt der Gürtel liegen, ein Bürger ohne Gürtel holt ihn (bestehende `hammer_tests.rs`, B-929). Tests: `unit/belt_tests.rs` (4: nur Gürtel rodet und haut, Bergmann, ohne native Regeln, Bau und Reparatur nur mit Gürtel); `world/vein_tests.rs`: Test-Bäuern wurde ein Gürtel gegeben. Abweichung vom Default: Die Berufe Bergmann und Baumeister bleiben vorerst bestehen (Entfernen bräuchte Daten in `buildings.json`, mit Go geteilt) → Hinweis für FG1 (ein Beruf je Figur). Holzfällen und Roden sind im Code derselbe Sammelauftrag am Baum; daher braucht jede Baumarbeit den Gürtel. Hinweis für 🧑: Das heißt, Bürger ohne Gürtel bringen kein Holz mehr (Balance im Abnahme-Ticket prüfen). Spielstand-Format unverändert. Rot, nicht von dieser Session (auch auf main): `eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`, `campaign_ausbau`, `campaign_abstieg`, `alloc_je_bild`, `e2e_flow`, `bilder_stimmen_mit_go_ueberein`, `alle_welt_dateien_aus_data_sind_ladbar` (lokale rom.json).
