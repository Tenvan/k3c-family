# B-545 · Entscheidung D8: Baugerüst nur ohne Gefahr

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

GEB11 zeigte das Baugerüst vor der Burg bei `hubUpgrade.state = waitingWorker`, also auch nachts, wenn der Bau wegen Gefahr wartet und kein Bauer arbeitet.

## Ziel

🧑 bestätigt den umgesetzten Default.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 2.1 („ein Bauer baut“).

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen „bestätigt“ oder die Alternative vermerkt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Entscheidung 🧑 (KR2.1, 2026-10-23):** Alternative: Gerüst immer bei `waitingWorker`, auch nachts bei Gefahr (Stand GEB11). Umsetzung: B-572 (CLI, eine Zeile in `hubLookOf` und Test zurück). Der umgesetzte Default aus GEB12.3 gilt bis dahin.
