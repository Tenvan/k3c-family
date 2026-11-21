# WD1.3 · Anzeige: Waldstücke grün schraffiert in Overlay, Workbench und F4

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wd1/3-anzeige-waldstuecke
- **Abhängig von:** WD1.2
- **Tickets:** B-1204
- **Kriterien:** AC-03

## Ziel

Waldstücke erscheinen als grün schraffierte Fläche, die markierbaren äußeren Bäume hervorgehoben: Debug-Overlay im Spiel, Level-Ansicht der Workbench, Umschalter im Cheat-Fenster (F4).

## Kontext

B-1204 AC-05. Geometrie aus WD1.2. Debug-Overlays: `rust/crates/k3c-game/src/devtools/drawdebug.rs`, `debuggeo.rs`; Cheat-Fenster `k3c-devwin`/`k3c-devproto`; Workbench `tools/k3c-workbench/frontend/src/world/` (Level-Ansicht, Backend liefert die Geometrie).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/devtools/**`
- `rust/crates/k3c-game/src/props/**`
- `rust/crates/k3c-devwin/**`
- `rust/crates/k3c-devproto/**`
- `tools/k3c-workbench/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Overlay-Geometrie (Fläche, hervorgehobene Bäume) in `debuggeo.rs` und Zeichnen in `drawdebug.rs`.
2. Umschalter im Cheat-Fenster (devproto-Befehl).
3. Level-Ansicht der Workbench (Backend-Feld und Frontend-Schraffur).
4. Tests der Geometrie.

## Fertig, wenn

- [ ] AC-03: Anzeige in den drei Ansichten (Geometrie-Tests; Sicht am PC durch 🧑 im Abnahme-Ticket).
- [ ] `task check:diff` grün (außer vorher roten Tests).

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
