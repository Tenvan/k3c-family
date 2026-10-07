# BR1 · REG · Balancing-Runde Wirtschaft und Spieleabend 2

- **Status:** geplant
- **Domäne:** REG
- **Prio:** niedrig
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-155, B-015
- **Start-Commit:** edb2a8f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Nach W1 bis W6 sind die Wirtschaftswerte Startwerte; Zielkorridore liegen aus F1 vor, ein zweiter Spieleabend steht aus (B-155, B-015).

## Ziel

Die Wirtschaftswerte liegen in den Zielkorridoren, sind begründet und durch einen Spieleabend geprüft.

Am Ende sichtbar: Pass/Fail je Kennzahl, Begründungen in `docs/rules/`, Protokoll in `docs/playtests/`.

## Beteiligte und Zielgruppen

Familie spielt, 🧑 entscheidet über Werte; der Agent misst und wertet aus.

## Anforderungen

B-155 › Anforderungen, B-015 › Anforderungen. Vor AC-01 werden die Zielkorridore in `docs/rules/zielkorridore.md` auf den 6-min-Tag (Q65) neu gefasst (heute z. B. „erste Mauer ≤ 10 min“). Der Tageszyklus 6/2/4/2 min (Q65, Startwert) wird am Spieleabend 2 bestätigt oder mit Beschluss geändert. Nach Spieleabend 2 folgt ein Tag mit Release-Checkliste (Q20).

## Nicht-Ziele

Kampf und Bosse (BR2), neue Mechaniken, Ausbau des Testers (BAL1 bis BAL3).

## Regeln und Einschränkungen

Werte nur in `data/`, Begründung in `docs/rules/`; Spieleabende und Freigaben nur durch 🧑. Golden-Daten nach dem Golden-Ablauf (B-137). Der Sprint bleibt in der Domäne REG; Code-Änderungen beschränken sich auf Werte in `data/`.

## Beispiele

Zeit bis zur Mauer über dem Korridor → Kosten in `data/` senken, Messung wiederholen, Begründung eintragen.

## Ausnahme- und Fehlerfälle

Ein Korridor lässt sich nicht erreichen → Abweichung als Ticket, 🧑 entscheidet neu.

## Akzeptanzkriterien

- **AC-01** Die Zielkorridore der Wirtschaft stehen vor der Runde als Zahlen in `docs/rules/`, neu gefasst auf den 6-min-Tag (Q65) (B-155/AC-01).
- **AC-02** Jede Wertänderung in `data/` nennt im Commit die betroffenen Kennzahlen, jeder geänderte Wert ist in `docs/rules/` begründet (B-155/AC-02, B-155/AC-03, B-015/AC-01).
- **AC-03** Spieleabend 2 hat stattgefunden, das Protokoll liegt in `docs/playtests/`, die Werte sind damit geprüft (B-155/AC-04, B-015/AC-02).
- **AC-04** Je Zielkorridor ist Pass oder Fail festgehalten, jede Abweichung hat ein Ticket, `task check` und `task check:go` grün (B-155/AC-05).
- **AC-05** Der Tageszyklus 6/2/4/2 min (Q65, Startwert) ist am Spieleabend 2 bestätigt oder mit Beschluss von 🧑 geändert (B-155/AC-06).
- **AC-06** Nach Spieleabend 2 ist ein Tag mit Release-Checkliste gesetzt, nach Bestätigung durch 🧑 (Q20, B-155/AC-07).

## Offene Fragen

Termin und Teilnehmer: `docs/fragenkatalog.md Q24` (🧑).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BR1.1 | `BR1.1-vorbereitung-messung.md` | Umsetzung | autonom | fertig |
| BR1.2 | `BR1.2-spieleabend-2.md` | Workshop | Mensch | offen |
| BR1.3 | `BR1.3-auswertung-abschluss.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
