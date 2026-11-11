# NG9.4 · Abnahme am PC zu zweit mit zwei Gamepads

- **Status:** offen
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** –
- **Umgebung:** live
- **Branch:** ng9/4-abnahme-pc
- **Abhängig von:** NG9.2
- **Tickets:** B-802
- **Kriterien:** AC-04

## Ziel

🧑 spielt das native Spiel am PC zu zweit mit zwei Gamepads und hält Befund und Entscheidung fest.

## Kontext

Spec: B-802/AC-04, B-775/AC-03 (Teil PC). Start: `task native:game`. Checkliste: Hauptmenü, Lobby mit zwei Pads, Welt mit Fluss und festem `fern`, Rig-Figuren, Bauen, Nacht mit Gegnern, HUD und Hinweise, Ton, Pause, Speichern, Weiter. Ein Agent bereitet nur vor (Build, Checkliste), prüft nicht selbst.

## Erlaubte Dateien

- Planung NG9, B-802 (über MCP)

## Nicht-Ziele

Code ändern; Befunde werden Tickets.

## Schritte

1. **Ralf:** Spiel starten, Checkliste durchspielen (etwa 15 Minuten).
2. **Ralf:** Befund und Entscheidung in B-802 › Notizen.
3. **Claude:** Befunde als Tickets anlegen.

## Fertig, wenn

- [ ] AC-04: Befund und Entscheidung 🧑 in B-802 › Notizen.

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
