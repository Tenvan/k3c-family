# BR1 · REG · Balancing-Runde Wirtschaft und Spieleabend 2

- **Status:** geplant
- **Domäne:** REG
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-155, B-015
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach W1 bis W6 sind die Wirtschaftswerte Startwerte; Zielkorridore liegen aus F1 vor, ein zweiter Spieleabend steht aus (B-155, B-015).

## Ziel

Die Wirtschaftswerte liegen in den Zielkorridoren, sind begründet und durch einen Spieleabend geprüft.

Am Ende sichtbar: Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/`.

## Beteiligte und Zielgruppen

Familie spielt, 🧑 entscheidet über Werte; der Agent misst und wertet aus.

## Anforderungen

B-155 › Anforderungen, B-015 › Anforderungen.

## Nicht-Ziele

Kampf und Bosse (BR2), neue Mechaniken, Ausbau des Testers (BAL1 bis BAL3).

## Regeln und Einschränkungen

Werte nur in `data/`, Begründung in `docs/rules/`; Spieleabende und Freigaben nur durch 🧑. Golden-Daten nach dem Golden-Ablauf (B-137). Der Sprint bleibt in der Domäne REG; Code-Änderungen beschränken sich auf Werte in `data/`.

## Beispiele

Zeit bis zur Mauer über dem Korridor → Kosten in `data/` senken, Messung wiederholen, Begründung eintragen.

## Ausnahme- und Fehlerfälle

Ein Korridor lässt sich nicht erreichen → Abweichung als Ticket, 🧑 entscheidet neu.

## Akzeptanzkriterien

- **AC-01** Die Zielkorridore der Wirtschaft stehen vor der Runde als Zahlen in `docs/rules/` (B-155/AC-01).
- **AC-02** Jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen, jeder geänderte Wert ist in `docs/rules/` begründet (B-155/AC-02, B-155/AC-03, B-015/AC-01).
- **AC-03** Spieleabend 2 hat stattgefunden, das Protokoll liegt in `docs/playtests/`, die Werte sind damit geprüft (B-155/AC-04, B-015/AC-02).
- **AC-04** Je Zielkorridor ist Pass oder Fail festgehalten, jede Abweichung hat ein Ticket, `task check` grün (B-155/AC-05).

## Offene Fragen

Termin und Teilnehmer: `docs/fragenkatalog.md Q24`; Zahlen der Korridore: `Q02`; Reihenfolge Wirtschaft vor Bürger: `Q22` (🧑).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BR1.1 Vorbereitung: Zielkorridore prüfen, Kennzahlen messen, Wertänderungen vorschlagen (AC-01, AC-02).
- BR1.2 🧑 Spieleabend 2 mit Protokoll in `docs/playtests/` (AC-03).
- BR1.3 Auswertung, Werte nachziehen und begründen, Pass/Fail festhalten; schließt den Sprint ab (Doku-Sprint, kein Review) (AC-02, AC-04).

## Abnahme

–
