# ECO1.3 · Werkzeuggürtel vom Handwerksmeister

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
