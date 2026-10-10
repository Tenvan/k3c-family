# B-394 · Overlay-Gesten (RB, LB + RB) sind an der Xbox abgenommen (B-195/AC-02)

- **Domäne:** PLAT
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** ABN
- **Erstellt:** 2026-10-10
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

PL1.5 (2026-10-10) konnte B-195/AC-02 nicht abnehmen: Es gab keinen Xbox-Test. RB 3 s halten (Diagnose) und LB + RB 3 s halten (Cheat-Dialog) sind am PC nachgestellt, an der Xbox mit Edge aber nicht geprüft.

## Ziel

🧑 hat die Overlay-Gesten an der Xbox mit dem Controller geprüft und das Ergebnis steht fest.

## Beteiligte und Zielgruppen

🧑 prüft am Gerät; ein Agent trägt das Ergebnis ein und legt Mängel als Ticket an.

## Anforderungen

- Ablauf wie PL1.5 › Schritt 3: `gamepad-test.html` (RB, LB je 3 s, Bericht senden), dann `game.html`: RB 3 s = Info-Zeilen, LB + RB 3 s = Cheat-Dialog, LB + RB kurz = zu. Je Schritt notieren, was reagiert.

## Nicht-Ziele

Code ändern; Mängel hier beheben (→ Ticket).

## Regeln und Einschränkungen

Hardware entkoppelt (`docs/arbeitsweise.md`): wird erledigt, wenn die Xbox da ist; B ist nicht belegt, View + Menu bleibt reserviert.

## Beispiele

RB 3 s gehalten → Info-Zeilen links unten erscheinen; noch einmal → weg.

## Ausnahme- und Fehlerfälle

Edge reagiert selbst auf RB/LB (Tab, Zurück, Fokus) → Bericht aus `gamepad-test.html` an den Agenten, Befund als Ticket.

## Akzeptanzkriterien

- **AC-01** 🧑 öffnet und schließt Diagnose und Cheat-Dialog auf der Xbox mit dem Controller (B-195/AC-02); Ergebnis je Schritt steht im Ticket.

## Offene Fragen

keine

## Notizen

Aus PL1.5 (AC-03 verschoben, Beschluss 🧑 2026-10-10 im Chat). Gehört zu B-195.
