# BI2.4 · Mine, Eisenstollen, Kristallhöhle: Verfall, Glut, Leuchten

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/bi2
- **Abhängig von:** BI2.2, BI2.3
- **Tickets:** B-904, B-905, B-906
- **Kriterien:** AC-05, AC-06, AC-07

## Ziel

Stützbalken und Verfall in der Mine, Glut im Eisenstollen, Schein der Kristall-Adern in der Kristallhöhle (B-904, B-905, B-906).

## Kontext

Baut auf `props/decor.go` (BI2.3) und der Kristallfarbe (BI2.2) auf.

## Erlaubte Dateien

- `native/k3c-game/props/decor_glow.go` (neu), `native/k3c-game/props/decor.go`, `native/k3c-game/props/decor_test.go`

## Nicht-Ziele

`world/light.go`; Spielregeln.

## Schritte

1. Tests je Ticket (AC-01, AC-02).
2. Umsetzung in `decor_glow.go`.
3. `check:diff`, Commit.

## Fertig, wenn

B-904, B-905, B-906 je AC-01, AC-02 per Unit-Test erfüllt, `check:diff` grün.

## Prüfen

`check_run task:check:diff`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
