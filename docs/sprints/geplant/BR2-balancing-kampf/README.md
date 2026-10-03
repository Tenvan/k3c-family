# BR2 · REG · Balancing-Runde Kampf und Bosse und Spieleabend 3

- **Status:** geplant
- **Domäne:** REG
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-156
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach K1 bis K5 sind Gegner-, Wellen- und Boss-Werte Startwerte; ein dritter Spieleabend steht aus (B-156).

## Ziel

Kampf-, Gegner- und Bosswerte liegen in den Zielkorridoren, sind begründet und durch einen Spieleabend geprüft.

Am Ende sichtbar: Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/`.

## Beteiligte und Zielgruppen

Familie spielt, 🧑 entscheidet über Werte; der Agent misst und wertet aus.

## Anforderungen

B-156 › Anforderungen.

## Nicht-Ziele

Wirtschaft (BR1), neue Mechaniken, Release (RL1).

## Regeln und Einschränkungen

Werte nur in `data/`, Begründung in `docs/rules/`; Spieleabende und Freigaben nur durch 🧑. Golden-Daten nach dem Golden-Ablauf (B-137). Der Sprint bleibt in der Domäne REG; Code-Änderungen beschränken sich auf Werte in `data/`.

## Beispiele

Endboss fällt mit 2 Spielern in unter einer Minute → Werte in `data/` anheben, Messung wiederholen, Begründung eintragen.

## Ausnahme- und Fehlerfälle

Ein Korridor lässt sich nicht erreichen → Abweichung als Ticket, 🧑 entscheidet neu.

## Akzeptanzkriterien

- **AC-01** Die Zielkorridore für Kampf und Bosse stehen vor der Runde als Zahlen in `docs/rules/` (B-156/AC-01).
- **AC-02** Jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen, jeder geänderte Wert ist in `docs/rules/` begründet (B-156/AC-02, B-156/AC-03).
- **AC-03** Spieleabend 3 hat stattgefunden, das Protokoll liegt in `docs/playtests/` (B-156/AC-04).
- **AC-04** Je Zielkorridor ist Pass oder Fail festgehalten, jede Abweichung hat ein Ticket, `task check` grün (B-156/AC-05).

## Offene Fragen

Termin und Teilnehmer: `docs/fragenkatalog.md Q24`; Zahlen der Korridore: `Q02` (🧑).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BR2.1 Vorbereitung: Zielkorridore prüfen, Kennzahlen je Grad messen, Wertänderungen vorschlagen (AC-01, AC-02).
- BR2.2 🧑 Spieleabend 3 mit Protokoll in `docs/playtests/` (AC-03).
- BR2.3 Auswertung, Werte nachziehen und begründen, Pass/Fail festhalten; schließt den Sprint ab (Doku-Sprint, kein Review) (AC-02, AC-04).

## Abnahme

–
