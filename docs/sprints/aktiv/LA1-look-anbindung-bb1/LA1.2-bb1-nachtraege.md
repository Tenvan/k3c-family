# LA1.2 · BB1-Nachträge im Rust-Spiel: Waldwand, Schiff-Baustelle, Pflanzung, Leitmotive

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** LA1.1
- **Tickets:** B-1146, B-1147, B-1148, B-1149, B-1150, B-1151
- **Kriterien:** AC-02

## Ziel

Die BB1-Nachträge sind im Rust-Spiel angebunden: Waldwand und acht neue Bäume, Leitmotive mit Luftperspektive, Schiff-Baustelle mit eigenem Gerüst und Bauplatz-Breiten, unterscheidbare Plantage-Stufen, Schiffsrumpf 4 bis 5 M lang, Baum-Texturbudget um 15 MiB gesenkt.

## Kontext

Lane zweit, nach LA1.1 (Größenstufen). Tickets B-1146 bis B-1151 (Ausgangslage dort), Standard `docs/standards/bildaufbau.md`, Papierschnitt über MCP `assets_*` (Entscheidung 005), Grafik rendern mit `task gen:paper` nur für die eigene Prüfung (Lane zweit darf Gen laufen lassen). Die Größen folgen der Tabelle aus LA1.1.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/art/**`, `world/**`, `props/**`
- `public/grafik/**` (Specs, `index.paper.json`, `golden.json`), `data/paper.json`, `data/look.json`
- Assets nur über MCP `assets_*`

## Nicht-Ziele

Stimmung (LA1.4, LA1.5), Kompass (LA1.3), neue Regeln.

## Schritte

1. Tickets B-1146 bis B-1151 lesen, Reihenfolge nach Wirkung (Waldwand, Schiff, Plantage zuerst).
2. Je Ticket umsetzen mit Test oder Bildvergleich; Assets nur über `assets_*`.
3. Texturbudget messen (B-1151), Ergebnis eintragen.
4. `cargo test -p k3c-game`, `task rust:lint`, einmal `task check:diff`.

## Fertig, wenn

- [ ] AC-02: B-1146 bis B-1151 erledigt oder mit Grund als Ticket verschoben.
- [ ] Texturbudget der Bäume um 15 MiB gesenkt (gemessen).
- [ ] Tests und `task rust:lint` grün.

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
