# B-100 · Eine Insel hat n Stufen, die alle laufen und pro Spieler begehbar sind

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** SP12
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1

## Ausgangslage

Heute ist eine Stufe eine Welt (`engine/sim/campaign.go`, `travel.go`): Alle Spieler wechseln gemeinsam, nur die aktuelle Stufe tickt, die anderen stehen still. Entscheidung 003 und `docs/rules/stufen.md` verlangen: ein Level (Insel) mit n Stufen, alle laufen weiter, jeder Spieler wechselt einzeln.

## Ziel

Eine Insel mit ihren Stufen läuft in der Simulation als Einheit; Spieler sind an eine Stufe gebunden und wechseln einzeln über Tiefen-Eingang und Treppe. Nutzen: Spieler können an verschiedenen Orten arbeiten.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen, auch in verschiedenen Stufen); Umsetzung durch Entwickler oder Agent in Go; 🧑 beschließt Reihenfolge der Sprints.

## Anforderungen

- Mehrere Stufen einer Insel ticken in jedem Tick mit gemeinsamer Zeit und eigenen Wellen je Stufe, auch ohne Spieler.
- Jeder Spieler hat eine Stufe; der Wechsel erfolgt einzeln (2 s am Eingang oder an einer gebauten Treppe).
- Wellenstärke zählt die Spieler der Insel (siehe B-101).
- Das Baumaterial ist ein Vorrat je Insel (alle Stufen teilen ihn, B-108); Gold bleibt je Spieler.
- Spielstand speichert Insel und Stufe je Spieler (`SAVE_VERSION` erhöhen, Altstände laden).

## Nicht-Ziele

Protokoll und Client (B-104, B-106), Inselwechsel (B-103), Bosse, Raum-Optionen (B-101, B-102).

## Regeln und Einschränkungen

Entscheidung 003, `docs/rules/stufen.md`. Reihenfolge laut Arbeitsweise: REG → SIM → CLI; Protokolländerung als eigene Session. Last: B-099 und SP11 (Pi 3) messen. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Spieler 1 steht im Wald, Spieler 2 in der Höhle; beide Stufen ticken, in der Höhle läuft der Aggressionspool, im Wald kommt die Nacht-Welle.

## Ausnahme- und Fehlerfälle

Hub ohne Verteidiger fällt → Niederlage-Modus dieser Stufe (B-102). Spieler fällt in einer Stufe → Respawn an deren Burg.

## Akzeptanzkriterien

- **AC-01** Test: zwei Stufen ticken gleichzeitig, ein Spieler in jeder, Wellen und Zyklus laufen unabhängig weiter.
- **AC-02** Test: Einzelwechsel über Eingang und Treppe, Reihenfolge der Spieler bleibt, andere Spieler bleiben stehen.
- **AC-03** Golden-/Determinismus-Test: gleicher Seed und gleiche Eingaben ergeben gleiche Zustände.
- **AC-04** Spielstand mit Insel und Stufe je Spieler wird gespeichert und geladen; ein alter Stand wird geladen.
- **AC-05** Rechenzeit je Tick mit 3 aktiven Stufen und 4 Spielern ist gemessen (Wert im Ticket).

## Offene Fragen

Tick-Budget mit 3 aktiven Stufen auf dem Pi 3 (SP11).

## Notizen

Aus R1.3. Groß: SP12 setzt den SIM-Kern um (Insel, Einzelwechsel, Spielstand); Raum und Protokoll folgen in B-133 und B-104.

SP12 (2026-10-02) hat den SIM-Kern umgesetzt (Insel, Einzelwechsel, Vorrat je Insel, Spielstand Version 2, Benchmark). Offen für den Rest: Raum auf Insel (B-133), Protokoll (B-104), Kamera je Stufe (B-106).
