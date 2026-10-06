# SK1 · SIM · Skill-Baum mit Tank und Zauberer, Respec-Regeln abfragbar

- **Status:** geplant
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-007, B-270
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

S1 liefert Schlag und erste Skills; der Baum mit Tank und Zauberer ist nicht vollständig spielbar (B-007), Respec und Lernen sind nur mit Seiteneffekt prüfbar (B-270).

## Ziel

Tank und Zauberer lassen sich über den Baum lernen und umskillen, der Server fragt die Regeln seiteneffektfrei ab. Am Ende sichtbar: Skill-Baum spielbar, Respec und Lernen ohne Seiteneffekt prüfbar.

## Beteiligte und Zielgruppen

Spielende; Agent baut in `engine/sim/`.

## Anforderungen

B-007 › Anforderungen; B-270 › Anforderungen.

## Nicht-Ziele

Anzeige im Client (S3), Heiler und Dieb über B-007 hinaus.

## Regeln und Einschränkungen

SIM; Werte aus `data/`, keine Zahlen im Code.

## Beispiele

Respec am Tag an der Burg → Punkte frei, Abfrage ändert nichts.

## Ausnahme- und Fehlerfälle

Lernen ohne Punkte → abgelehnt 🚫, Zustand unverändert.

## Akzeptanzkriterien

- **AC-01** Skill-Baum mit Tank und Zauberer ist spielbar (B-007/AC-01, B-007/AC-02, B-007/AC-03).
- **AC-02** Die Sim prüft Respec und Lernen ohne Seiteneffekt (B-270/AC-01).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SK1.1 Lern- und Respec-Regeln seiteneffektfrei (AC-02).
- SK1.2 Skill-Baum Tank und Zauberer (AC-01).
- SK1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
