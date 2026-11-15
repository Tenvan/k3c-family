# NA1.2 · Ende-zu-Ende-Lauf TestFlow wieder grün

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/na1
- **Abhängig von:** NA1.1
- **Tickets:** B-912
- **Kriterien:** AC-03

## Ziel

`TestFlow` im Paket `native/k3c-game/e2e` ist grün, die Ursache des Fehlers steht im Ergebnis (B-912).

## Kontext

`native/k3c-game/e2e/flow_test.go`: `flowScript` (Zeile 33–39) ist die feste Eingabe-Folge (Format `Spieler@Takt=Eingabe`, `game.ParseScript`); Spieler 1 = Tastatur, Spieler 2 = Pad 1 (B-893). `day()` (Zeile 84–95) wartet mit `r.until("Spieler 2 findet einen Skill-Punkt", 600, …)` auf das Ereignis `skillPoint` von Spieler 2 (`hasEvent`). Stand 2026-11-15: Stationen bis `muenzen` (Takt 299) grün, danach „bis Takt 600 nicht erreicht“. Spieler 2 läuft ab Takt 21 nach rechts und hält bei Takt 380. Vermutet: Die Welt Standard legt den Skill-Punkt seit geänderten Welt- oder Biom-Daten (BI1) anders. Der Test läuft nur ohne `-short` (nicht in der CI). Hilfen: `run_test.go` (`until`, `mark`, `frame`), Codemap `native/k3c-game/e2e/codemap.md`.

## Erlaubte Dateien

- `native/k3c-game/e2e/**`

## Nicht-Ziele

Spielregeln, Welt- oder Biom-Daten ändern; Code außerhalb von `e2e/`; `TestFlow` in die CI aufnehmen.

## Schritte

1. `go test ./e2e/ -run TestFlow -count=1` im Modul `native/k3c-game`, Fehler bestätigen.
2. Im Lauf ausgeben, wo Skill-Punkte der Stufe liegen und wo Spieler 2 bei Takt 380 und 600 steht; prüfen, ob das Ereignis `skillPoint` überhaupt kommt.
3. Liegt es am Test: Laufzeit von Spieler 2 aus der Position des nächsten Skill-Punkts im Frame ableiten oder Skript und Grenzen anpassen; keine Station streichen.
4. Liegt es am Spiel: Ticket mit Fundort anlegen (`plan_create`, Projekt NAT, Sprint NA1), Session als blockiert melden, Test nicht verbiegen.
5. `go test ./e2e/ -run TestFlow -count=2`, dann `task check:diff`.

## Fertig, wenn

- [ ] AC-03: `go test ./e2e/ -run TestFlow -count=2` grün; Ursache mit Datei und Zeile im Ergebnis; `grep -n "skillPoint\|skillmenue" native/k3c-game/e2e/flow_test.go` zeigt die Stationen weiter (B-912/AC-01, AC-02).
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
