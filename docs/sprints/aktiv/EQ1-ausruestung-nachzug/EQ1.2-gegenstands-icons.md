# EQ1.2 · Gegenstands-Icons am Regal und im Holen-Hinweis

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** ?
- **Branch:** eq1/2-gegenstands-icons
- **Abhängig von:** –
- **Tickets:** B-1207
- **Kriterien:** AC-02

## Ziel

Das Regal am Gebäude zeigt die Stückzahl als Icons, der Holen-Hinweis zeigt das Icon des Gegenstands.

## Kontext

Ticket B-1207. Icons `icons/icon-guertel|bogen|schwert|ruestung` aus FG1.3 sind in `data/paper.json` zugeordnet. Icon-Cache: `hud/drawstock.rs` (`refresh_icon_cache`, `draw_icon`), Regal-Plan in `props/plan_sites.rs`, Hinweis in `hud/hints.rs`. Frame-Budget beachten.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/props/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Regal-Plan der vier Hersteller lesen.
2. Je Stück ein Icon am Regal.
3. Icon im Holen-Hinweis (de/en).
4. Tests, `alloc_je_bild` nicht schlechter.

## Fertig, wenn

- [ ] AC-02: Regal und Hinweis zeigen die Icons (B-1207 AC-01 bis AC-03).
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
