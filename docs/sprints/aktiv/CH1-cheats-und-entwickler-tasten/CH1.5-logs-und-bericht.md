# CH1.5 · Zusammenfassungs-Logs und Sitzungsbericht

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/ch1
- **Abhängig von:** CH1.2
- **Tickets:** B-889
- **Kriterien:** AC-06

## Ziel

Das native Spiel loggt Zustandswechsel, Kampf je Nacht, Wirtschaft je Tag, Takte über Budget und Cheats knapp und schreibt am Ende einen Sitzungsbericht nach `reports/`.

## Kontext

- Ereignisse der Sim kommen im Frame (`view.Frame.Events`).
- Budget-Wert in `data/` (Default 4 ms).
- Logs nie je Takt, nur bei Wechsel oder als Zusammenfassung.

## Erlaubte Dateien

- `native/k3c-game/devtools/**`
- `native/k3c-game/game/**`
- `native/k3c-game/local/**`
- `native/k3c-game/view/**`
- `data/*.json`

## Nicht-Ziele

- Balancing-Auswertung.

## Schritte

1. Zähler und Wechsel-Erkennung rein und getestet (Listener).
2. Bericht als JSON, Test mit Temp-Ordner.

## Fertig, wenn

- [ ] AC-06: Go-Tests grün (kein Log ohne Wechsel, Bericht geschrieben).
- [ ] `check_run task:check:diff` grün.

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
