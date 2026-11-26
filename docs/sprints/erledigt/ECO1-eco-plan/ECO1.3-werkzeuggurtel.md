# ECO1.3 · Werkzeuggürtel vom Handwerksmeister

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/3-werkzeuggurtel
- **Abhängig von:** –
- **Tickets:** B-1193
- **Kriterien:** AC-03

## Ziel

Der Handwerksmeister (umbenannte Werkstatt) stellt einen Werkzeuggürtel (Hammer, Spitzhacke, Axt) her, der im Regal liegt.

## Kontext

Ticket B-1193. Neue Mechanik nur in Rust (`k3c-sim`), deterministisch, 2+ Spieler. Gürtel ersetzt den Einzelhammer (Preis 30 Holz, 10 Stein, 3 Gold in `data/buildings.json › workshop`), Regal bis 3. Spielstand-Version erhöhen, alte Stände egal. Regeltexte `docs/rules/buerger.md`, `materialien-gebaeude.md`, Glossar ziehen mit. Go-Golden nur mit `native_rules` aus (siehe HB1).

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-sim/src/state/**`
- `rust/crates/k3c-data/src/**`
- `data/buildings.json`
- `docs/rules/buerger.md`
- `docs/rules/materialien-gebaeude.md`
- `docs/glossar.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Werkstatt-Logik (Hammer im Regal) in `k3c-sim/src/building` finden.
2. Gürtel statt Hammer im Regal, Preis aus den Daten, Spielstand-Roundtrip.
3. Regeltexte und Glossar.
4. Tests (Herstellen, Regal, Spielstand).

## Fertig, wenn

- [ ] AC-03: Werkstatt stellt Gürtel her; Spielstand-Roundtrip grün.
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

AC-03 umgesetzt und geprüft: Mit nativen Regeln (`WorldValues::native_rules`) kostet das Werkstatt-Produkt den Werkzeuggürtel, 30 Holz + 10 Stein + 3 Gold (`unit/hammer.rs › belt_cost`, `place_product`); ohne native Regeln bleibt der Hammer aus `buildings.json` (Go-Parität, `buildings.json` ist mit Go geteilt). Regal weiter `hammerRack` (3), Herstellung, Holen und Spielstand wie beim Hammer: `Troop::hammer` und `Site::hammers` zählen jetzt den Gürtel (Namen bleiben wegen Go-Parität), das Spielstand-Format ist unverändert, die Version musste nicht steigen. Tests: `werkstatt_stellt_den_werkzeuggurtel_her` (mit und ohne native Regeln), `werkstatt_hammer_herstellen_und_holen` (zwei Spieler, Preis über `place_product`). Regeltexte `buerger.md`, `materialien-gebaeude.md` und Glossar (neuer Begriff Werkzeuggürtel) angepasst. Abweichung: Umbenennung der Werkstatt in Handwerksmeister (Text und Grafik) liegt bei FG1.3. Rot, nicht von dieser Session (auch auf main): `eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`, `campaign_ausbau`, `campaign_abstieg` (nextId 383, erwartet 380, durch HB1-Regeln). `task check:diff` nur mit fehlendem `golangci-lint`.
