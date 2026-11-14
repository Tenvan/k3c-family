# MG11 · PAP · Papierschnitt nachbessern: Gebäude, Hintergründe ohne Bäume, Natur und Icons

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-721, B-722, B-723, B-699
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

PS1.3 und PS1.4: Gebäude-Stufen nur farblich, Hintergründe flach und mit verwechselbaren Bäumen, Natur zu klein, Icons unlesbar.

## Ziel

Welt-Grafik besteht die Sichtprüfung.

## Beteiligte und Zielgruppen

Claude setzt um (Assets über MCP); 🧑 prüft die Grafik im Review der Workbench.

## Anforderungen

B-721, B-722, B-723.

## Nicht-Ziele

Spiel-Einbau der Größen (US3).

## Regeln und Einschränkungen

Nach RD2. B-699 ist das Dach-Ticket des Projekts PAP und schließt mit dem Projekt, nicht mit diesem Sprint.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `public/grafik/gebaeude/**`, `public/grafik/hintergruende/**`, `public/grafik/objekte/**`, `public/grafik/icons/**`, `tools/paper/golden/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** B-721/AC-01 bis AC-03
- **AC-02** B-722/AC-01 bis AC-03
- **AC-03** B-723/AC-01 bis AC-03
- **AC-04** Review ohne offenen schweren Befund; Sichtprüfung durch 🧑 im Review der Workbench.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MG11.1 | `MG11.1-gebaeude.md` | Umsetzung | autonom | offen |
| MG11.2 | `MG11.2-hintergruende-wald-hoehle-mine.md` | Umsetzung | autonom | offen |
| MG11.3 | `MG11.3-hintergruende-stollen-kristall.md` | Umsetzung | autonom | offen |
| MG11.4 | `MG11.4-natur-icons-muenze.md` | Umsetzung | autonom | offen |
| MG11.5 | `MG11.5-review.md` | Umsetzung | autonom | offen |

Reife Entwurf:

- MG11.1 Gebäude (B-721)
- MG11.2 Hintergründe Wald, Höhle, Mine (B-722)
- MG11.3 Hintergründe Eisenstollen, Kristall (B-722)
- MG11.4 Natur, Icons, Münze (B-723)
- MG11.5 Review

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
