# HB1.2 · HUD: Gebäudename über dem Monarchen, feste Texte aus, Händler und Stände sichtbar

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/hb1
- **Abhängig von:** HB1.1
- **Tickets:** B-1186
- **Kriterien:** AC-02, AC-03

## Ziel

Vor einem Gebäude zeigt der Monarch darüber den Gebäudenamen, andere feste Gebäudetexte entfallen, Händler und Stände sind sichtbar.

## Kontext

Ticket B-1186. HUD und Welt in `rust/crates/k3c-game/src/hud/**` und `props/**`. Händler und Stände: erst klären, ob sie nicht gezeichnet (Props/Assets) oder nicht erzeugt werden (Sim/Level; dann Befund unter „Für den Hauptrechner“ statt Sim-Eingriff außerhalb der erlaubten Dateien). Texte über Text-Slots, keine Allokation je Bild.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/props/**`
- `rust/crates/k3c-game/src/view/**`

## Nicht-Ziele

Baustellen-Icons (HB1.1), neue Gebäude.

## Schritte

1. Name über dem Gebäude, solange ein Monarch davor steht (Abstand wie die Bau-Interaktion).
2. Andere feste Gebäudetexte ausblenden.
3. Ursache für fehlenden Händler samt Ständen finden und beheben.
4. Tests.

## Fertig, wenn

- [ ] AC-02: Gebäudename über dem Gebäude bei Monarch davor, andere feste Texte entfallen.
- [ ] AC-03: Händler und Stände sichtbar (Test) oder Ursache als Befund dokumentiert.
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

Gebäudename (klein, Gold) steht über dem Gebäude, solange ein Monarch davor steht (`monarch_before`, props/plan_sites.rs); das feste Treppen-Schild ist entfallen, Preis-/Bau-Texte bleiben. Ursache „Händler und Stände fehlen“: `props` zeichnete den Händler gar nicht, nur das HUD-Label. Neu: `props/plan_merchant.rs` zeichnet Kauf- und Verkauf-Stand an den Sim-Orten (`merchant_x`) mit Material-Icon und Lebensbalken. Tests: `plan_merchant_tests.rs`; `cargo test -p k3c-game --lib props` 100 grün, clippy sauber. `check:diff` rot nur an den 3 bekannten `planning.test.ts`-Fehlern. Für Ralf im Spiel prüfen: Händlerfigur selbst fehlt weiter (nur Stände), Optik der Stände.
