# B-545 · Entscheidung D8: Baugerüst nur ohne Gefahr

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
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

**Frage:** Steht das Gerüst nur, wenn wirklich gebaut werden kann?

- **Default (umgesetzt in GEB12.3):** Gerüst nur bei `waitingWorker` ohne `danger`; bei Gefahr Zustand ohne Gerüst.
- **Alternative:** Gerüst immer bei `waitingWorker` (Stand GEB11), auch nachts.
- **Wenn 🧑 die Alternative wählt:** eine Zeile in `src/scenes/render/hubGrowth.ts` (`hubLookOf`) und der Test zurück; kein Server betroffen.
