# B-543 · Entscheidung D6: Geschlossene Truhe ohne Serverzustand

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** KR2
- **Projekt:** GEB
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB10 zeigt liegende Truhen geschlossen; beim Aufheben öffnet der Client sie kurz und blendet sie aus. Der Server kennt keinen Zustand „offen“.

## Ziel

🧑 bestätigt, dass kein Serverzustand nötig ist.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Kein SIM-Ticket ohne Entscheidung.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen „bestätigt“ oder die Alternative vermerkt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Entscheidung 🧑 (KR2.1, 2026-10-23):** bestätigt, „liegend = geschlossen, aufgehoben = kurz offen“ nur im Client; kein Serverzustand, kein SIM-Ticket.
