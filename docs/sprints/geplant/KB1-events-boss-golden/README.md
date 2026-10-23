# KB1 · SIM · Kampf-Nachträge: Mond-Events, Endboss-Bau, Golden-Läufe, Feuergeist

- **Status:** geplant
- **Projekt:** KMP
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-379, B-380, B-343, B-327, B-328
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Offene Punkte aus K1 und K3: Die Vollmond-Belohnung zahlt nur in Stufe 0 (B-379, `engine/sim/events_moon.go`), der Blutmond verstärkt Boss-Flächen nicht (B-380, `boss.go`, `boss_abilities.go`), der Endboss-Bau liegt an der inneren Kante des Ausgangs-Chunks (B-343), Eisenstollen und Kristallhöhle haben keinen Golden-Lauf (B-327), der Feuergeist hat keine Flammen-Fläche (B-328).

## Ziel

Mond-Events und Gegner verhalten sich in allen Stufen und Biomen wie im Regelwerk beschrieben und sind durch Tests abgesichert. Am Ende sichtbar: `task check:go` grün, neue Golden-Dateien für Tiefe 3 und 4 mit Begründung im Commit.

## Beteiligte und Zielgruppen

Spielende (faire Events), Agent in SIM, 🧑 entscheidet B-343, B-327 und B-328.

## Anforderungen

B-379, B-380, B-343, B-327, B-328 › Anforderungen (Specs noch Entwurf; drei davon sind Fragen).

## Nicht-Ziele

Balancing der Werte (BAL ruht); Anzeige im Client (K5); neue Gegner oder Events.

## Regeln und Einschränkungen

Nur Domäne SIM; Werte aus `data/*.json`; `engine/rng`, kein `math/rand`. Golden-Dateien nur mit `task go:golden` und Begründung. Regeln in `docs/rules/gegner.md` und `docs/rules/` gelten; Abweichungen werden dort vom REG-Beschluss, nicht von der Sim bestimmt.

## Beispiele

Alle spielen in Stufe 2 und die Burg hält die Vollmond-Nacht → die 50 Münzen liegen am Hub der bespielten Stufe. Blutmond und Boss-Flächenangriff → Schaden mit Mond-Faktor.

## Ausnahme- und Fehlerfälle

Spieler in mehreren Stufen gleichzeitig → Belohnung je bespielter Stufe höchstens einmal je Nacht. Bau passt in kein freies Chunk → Fehler beim Erzeugen, nicht stilles Überlappen.

## Akzeptanzkriterien

- **AC-01** Die Vollmond-Belohnung gilt für die bespielten Stufen, der Blutmond verstärkt Boss-Flächen und Splitter (B-379, B-380).
- **AC-02** Die Lage des Endboss-Baus ist entschieden und im Generator mit Test umgesetzt (B-343).
- **AC-03** Golden-Läufe für Eisenstollen und Kristallhöhle liegen unter `testdata/golden/` (B-327).
- **AC-04** Der Feuergeist hinterlässt eine Flammen-Fläche, oder `docs/rules/gegner.md` ist angepasst (B-328).

## Offene Fragen

Vorentschieden (autonom, Claude, 2026-10-23), Bestätigung durch 🧑 über KR6:

- **B-379:** Vollmond-Belohnung je Stufe mit Spielern: jede bespielte Stufe, deren Burg in der Nacht hielt, bekommt die Münzen an ihrem Hub (höchstens einmal je Stufe und Nacht). Ereignisse `eventStarted`/`eventEnded` meldet weiter nur Stufe 0.
- **B-380:** Ja, der Blutmond verstärkt auch Boss-Flächenangriff und Splitter (`moonDamage`), die Bodenfläche (Hazard) bleibt ohne Faktor.
- **B-343:** Bau bleibt an der inneren Kante des Ausgangs-Chunks (heutige Umsetzung), kein Mindestabstand zu Portalen; Nachweis über die bestehenden Tests.
- **B-327:** Ja, je ein Golden-Lauf für Eisenstollen und Kristallhöhle mit vollem Aggressionspool, 2 Spieler.
- **B-328:** Keine Flammen-Fläche bauen (neue Mechanik mit Balancing-Wirkung); `docs/rules/gegner.md` vermerkt, dass der Feuergeist bis zur Entscheidung nur Fernkampf mit Abstand hat.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KB1.1 | `KB1.1-mond-events.md` | Umsetzung | autonom | offen |
| KB1.2 | `KB1.2-endboss-bau-lage.md` | Umsetzung | autonom | offen |
| KB1.3 | `KB1.3-golden-eisen-kristall.md` | Umsetzung | autonom | offen |
| KB1.4 | `KB1.4-feuergeist-flaeche.md` | Umsetzung | autonom | offen |
| KB1.5 | `KB1.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
