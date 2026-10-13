# HW1.10 · Overlay-Gesten RB und LB + RB an der Xbox (aus PL1.5)

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** hw1/10-overlay-gesten-xbox
- **Abhängig von:** –
- **Tickets:** B-394
- **Kriterien:** AC-09

## Ziel

🧑 hat auf der Xbox mit Edge und Controller Diagnose (RB 3 s) und Cheat-Dialog (LB + RB 3 s, zu mit LB + RB kurz) geöffnet und geschlossen; das Ergebnis je Schritt steht in B-394.

## Kontext

PL1.5 (2026-10-10) konnte B-195/AC-02 mangels Xbox nicht abnehmen; am PC nachgestellt. Ablauf aus B-394 › Anforderungen: zuerst `gamepad-test.html` (RB, LB je 3 s halten, Bericht senden → `reports/*.json`), dann `game.html`. B ist nicht belegt, View + Menu bleibt reserviert.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `docs/backlog/B-394-xbox-overlay-gesten-abnahme.md` (über `plan_section`/`plan_set`)
- neue Tickets für Mängel (über `plan_create`)

## Nicht-Ziele

Code ändern; Mängel hier beheben (→ Ticket).

## Schritte

1. 🧑 öffnet `gamepad-test.html` auf der Xbox, hält RB und LB je 3 s, sendet den Bericht.
2. Agent liest den Bericht (`reports_list`, `report_read`) und notiert, ob Edge selbst auf RB/LB reagiert.
3. 🧑 öffnet `game.html`: RB 3 s → Info-Zeilen an/aus; LB + RB 3 s → Cheat-Dialog; LB + RB kurz → zu.
4. Agent trägt das Ergebnis je Schritt in B-394 ein und legt Mängel als Ticket an.

## Fertig, wenn

- [ ] AC-09: Diagnose und Cheat-Dialog sind auf der Xbox mit dem Controller geöffnet und geschlossen, Ergebnis je Schritt steht in B-394 (B-394/AC-01, B-195/AC-02).

## Prüfen

Beobachtung am Gerät durch 🧑 (Xbox, Edge, Controller); Bericht aus `gamepad-test.html` unter `reports/`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
