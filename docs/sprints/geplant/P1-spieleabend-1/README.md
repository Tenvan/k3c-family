# P1 · REG · Spieleabend 1

- **Status:** geplant
- **Projekt:** BAL
- **Domäne:** REG
- **Prio:** niedrig
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-008, B-151
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Die Familie hat das Spiel noch nie am TV gespielt; Balancing beruht auf Tests und Vermutungen. Ein Protokollformat und ein Fragebogen fehlen, `docs/playtests/` existiert noch nicht. Voraussetzung: Phase 1 (S1–S6) spielbar am TV, Spielmetrik-Report aus S2.

## Ziel

Die Familie spielt einen Abend, ein Protokoll mit Fragebogen und Spielmetrik liegt in `docs/playtests/`, Wünsche sind Tickets, Balancing-Änderungen stehen nur in JSON. Am Ende sichtbar: Protokoll und Folge-Tickets.

## Beteiligte und Zielgruppen

🧑 und Familie spielen am TV (auch Kinder); der Agent bereitet vor, protokolliert und wertet aus.

## Anforderungen

B-008 und B-151 › Anforderungen; zusätzlich Messung von Verbindung und Eingabe-Latenz am Abend (Plan Lücke 13, Regel aus B-144).

## Nicht-Ziele

Code-Änderungen am Abend, Balancing-Runden (B-155, B-156), Auswertungs-Werkzeug (B-160).

## Regeln und Einschränkungen

Balancing-Änderungen nur in `data/*.json`, alles andere als Tickets; Prozess nur in `docs/arbeitsweise.md`. Den Abend selbst und Termin legt nur 🧑 fest.

## Beispiele

Nacht 2 ist zu schwer → das Protokoll nennt die Stelle samt Spielmetrik, die Wertänderung steht in `data/`, Wünsche an Mechaniken werden Tickets.

## Ausnahme- und Fehlerfälle

Spiel stürzt ab oder hängt → das Protokoll hält es fest, Ticket vom Typ Problem; Server oder Netz fällt aus → Befund notieren, Abend abbrechen oder wiederholen entscheidet 🧑.

## Akzeptanzkriterien

- **AC-01** `docs/playtests/vorlage.md` mit allen Abschnitten und ein kindgerechter Fragebogen mit 8 Fragen (Daumen hoch/runter) plus einem Satz Freitext (Q24) liegen vor (B-151/AC-01, B-151/AC-02, B-151/AC-03).
- **AC-02** Ein Protokoll des Abends liegt in `docs/playtests/` (B-008/AC-01).
- **AC-03** Balancing-Änderungen stehen nur in JSON, alles andere als Tickets im Backlog (B-008/AC-02).
- **AC-04** Das Protokoll enthält den Spielmetrik-Report (Tod durch was, Nacht überlebt, Zeit bis zum ersten Bau) und die Beobachtung zu Verbindung und Latenz; die Latenz ist gegen das Ziel ≤ 100 ms (Q04) bewertet, gemessen mit dem Debug-Overlay (U4).

## Offene Fragen

Termin und Teilnehmer: 🧑, `docs/fragenkatalog.md Q24`.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| P1.1 | `P1.1-workshop-vorlage-fragebogen.md` | Workshop | Mensch | offen |
| P1.2 | `P1.2-spieleabend-am-tv.md` | Workshop | Mensch | offen |
| P1.3 | `P1.3-auswertung-abschluss.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
