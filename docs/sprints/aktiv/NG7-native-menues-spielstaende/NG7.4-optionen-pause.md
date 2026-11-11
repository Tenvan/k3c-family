# NG7.4 · Optionen, Pause-Menü und Spielende

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** menue
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng7/4-optionen-pause
- **Abhängig von:** NG7.3
- **Tickets:** B-800
- **Kriterien:** AC-04

## Ziel

Optionen, Pause-Menü und Spielende funktionieren und führen zurück ins Hauptmenü.

## Kontext

Spec: B-800. Optionen nach der Entscheidung 🧑 (Default: Lautstärke Musik und Effekte, Vollbild, Sprache). Vorbild: `src/scenes/optionsLogic.ts`, `OptionsScene.ts`, `pauseHub.ts` samt Tests; Spielende aus `engine/sim/island_defeat.go`, `island_victory.go` über den Frame. Werte gehen als Optionen über `view` an `audio` und `game`. Vertrag und Importregel wie NG7.1.

## Erlaubte Dateien

- `native/k3c-game/menu/**`
- Planung NG7, B-800 (über MCP)

## Nicht-Ziele

Online- und Dev-Optionen; Tastenbelegung ändern.

## Schritte

1. Fälle aus den TS-Tests als Go-Tests (rot).
2. Optionen.
3. Pause-Menü und Spielende.

## Fertig, wenn

- [ ] AC-04: Tests grün.
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
