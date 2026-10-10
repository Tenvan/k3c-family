# B-387 · Ö schließt bei offenem Cheat-Dialog zuerst den Dialog, das Overlay bleibt offen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-10, 🧑 im Chat (Befunde aus U5.4 freigegeben)

## Ausgangslage

Beobachtung 🧑 am 2026-10-10 beim Test von U5.4 (PC, Chrome): Mit offenem Cheat-Dialog schließt Ö das Overlay mit, statt nur den Dialog zu schließen. U5 verlangt in „Ausnahme- und Fehlerfälle“: Dialog offen und Ö → Dialog zuerst zu, Overlay bleibt. Die Ursache im Code (Tastenbehandlung des Debug-Overlays) ist noch nicht geprüft.

## Ziel

Ö schließt bei offenem Cheat-Dialog nur den Dialog; das Overlay bleibt offen, wie in U5 festgelegt.

## Beteiligte und Zielgruppen

🧑 testet am PC; Agent baut in `src/scenes/`.

## Anforderungen

- Dialog offen und Ö → nur der Dialog schließt.
- Dialog geschlossen und Ö → Overlay und Aktionsliste schließen wie bisher (U5.1).

## Nicht-Ziele

Neue Dev-Aktionen; Xbox-Zugang zum Overlay (B-195).

## Regeln und Einschränkungen

B nicht belegen, View + Menu reserviert; `src/scenes` rechnet nichts (`noSim.test.ts`); Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Overlay offen, Cheat-Dialog offen, Ö → Dialog zu, Overlay bleibt offen. Ö erneut → Overlay zu.

## Ausnahme- und Fehlerfälle

Dialog nicht offen → Ö schließt Overlay und Aktionsliste wie bisher.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Bei offenem Dialog schließt Ö nur den Dialog; bei geschlossenem Dialog schließt Ö Overlay und Liste.
- **AC-02** 🧑 bestätigt am PC in Chrome, dass Ö nur den Dialog schließt (Beobachtung).

## Offene Fragen

keine

## Notizen

Herkunft: U5.4, Frage 10 mit „nein“ beantwortet. Verwandt: U5 (U5.1, U5.2), B-192, B-231.
