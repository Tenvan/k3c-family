# BR2 · REG · Balancing-Runde Kampf und Bosse und Spieleabend 3

- **Status:** geplant
- **Domäne:** REG
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-156
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Nach K1 bis K5 sind Gegner-, Wellen- und Boss-Werte Startwerte; ein dritter Spieleabend steht aus (B-156).

## Ziel

Kampf-, Gegner- und Bosswerte liegen in den Zielkorridoren, sind begründet und durch einen Spieleabend geprüft.

Am Ende sichtbar: Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/`.

## Beteiligte und Zielgruppen

Familie spielt, 🧑 entscheidet über Werte; der Agent misst und wertet aus.

## Anforderungen

B-156 › Anforderungen. Nach Spieleabend 3 folgt ein Tag mit Release-Checkliste (Q20).

## Nicht-Ziele

Wirtschaft (BR1), neue Mechaniken, Release (RL1).

## Regeln und Einschränkungen

Werte nur in `data/`, Begründung in `docs/rules/`; Spieleabende und Freigaben nur durch 🧑. Golden-Daten nach dem Golden-Ablauf (B-137). Der Sprint bleibt in der Domäne REG; Code-Änderungen beschränken sich auf Werte in `data/`. Voraussetzung: BR1 (B-155).

## Beispiele

Endboss fällt mit 2 Spielern in unter einer Minute → Werte in `data/` anheben, Messung wiederholen, Begründung eintragen.

## Ausnahme- und Fehlerfälle

Ein Korridor lässt sich nicht erreichen → Abweichung als Ticket, 🧑 entscheidet neu.

## Akzeptanzkriterien

- **AC-01** Die Zielkorridore für Kampf und Bosse stehen vor der Runde als Zahlen in `docs/rules/` (B-156/AC-01).
- **AC-02** Jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen, jeder geänderte Wert ist in `docs/rules/` begründet (B-156/AC-02, B-156/AC-03).
- **AC-03** Spieleabend 3 hat stattgefunden, das Protokoll liegt in `docs/playtests/` (B-156/AC-04).
- **AC-04** Je Zielkorridor ist Pass oder Fail festgehalten, jede Abweichung hat ein Ticket, `task check` und `task check:go` grün (B-156/AC-05).
- **AC-05** Nach Spieleabend 3 ist ein Tag mit Release-Checkliste gesetzt, nach Bestätigung durch 🧑 (Q20, B-156/AC-06).

## Offene Fragen

Termin und Teilnehmer: `docs/fragenkatalog.md Q24` (🧑).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BR2.1 | `BR2.1-vorbereitung-messung.md` | Umsetzung | autonom | offen |
| BR2.2 | `BR2.2-spieleabend-3.md` | Workshop | Mensch | offen |
| BR2.3 | `BR2.3-auswertung-abschluss.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
