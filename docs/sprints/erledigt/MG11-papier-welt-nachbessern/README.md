# MG11 · PAP · Papierschnitt nachbessern: Gebäude, Hintergründe ohne Bäume, Natur und Icons

- **Status:** erledigt
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-721, B-722, B-723, B-699
- **Start-Commit:** 9cbe9be79
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

- **Bereich:** `public/grafik/gebaeude-paper/**`, `public/grafik/hintergruende/**`, `public/grafik/objekte/**`, `public/grafik/icons/**`, `tools/paper/golden/**`, `data/paper.json`
- **Abhängig von:** –
- **Braucht 🧑:** Sichtprüfung (AC-04)
- **Sperre:** golden

Korrektur im Review MG11.5: Bereich nannte `public/grafik/gebaeude/**`, geändert wurde `gebaeude-paper/`; `data/paper.json` (Frame-Maße der Bäume) kam in MG11.4 dazu.

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
| MG11.1 | `MG11.1-gebaeude.md` | Umsetzung | autonom | fertig |
| MG11.2 | `MG11.2-hintergruende-wald-hoehle-mine.md` | Umsetzung | autonom | fertig |
| MG11.3 | `MG11.3-hintergruende-stollen-kristall.md` | Umsetzung | autonom | fertig |
| MG11.4 | `MG11.4-natur-icons-muenze.md` | Umsetzung | autonom | fertig |
| MG11.5 | `MG11.5-review.md` | Umsetzung | autonom | fertig |

Reife Entwurf:

- MG11.1 Gebäude (B-721)
- MG11.2 Hintergründe Wald, Höhle, Mine (B-722)
- MG11.3 Hintergründe Eisenstollen, Kristall (B-722)
- MG11.4 Natur, Icons, Münze (B-723)
- MG11.5 Review

## Abnahme

2026-11-14, Claude autonom (Review MG11.5). AC-01–AC-03: Session-Ergebnisse MG11.1–MG11.4; Tests `tools/paper/golden/{gebaeude,hintergruende,natur}.test.ts`, `task check` und `task check:go` grün. AC-04: Review ohne schweren Befund; Sichtprüfung durch 🧑 offen (Gruppen `gebaeude-paper`, `hintergruende`, `objekte`, `icons`).
Behoben im Review: `materialien.test.ts` auf 75 Hintergründe, Wald ohne `smaragd`/`stein`-Ersatz, Farne an der Kachelnaht (neuer Test: Formen in 0..768), Münz-Test prüft alle Phasen je Keyframe, Baum-Regex mit Umlaut, Bereich korrigiert.
Neue Tickets: B-881 (Burg-Ausbau-Variante im Client), B-883 (fünf Hintergrund-Varianten im Client). Offen als Risiko (vermutet): 360-px-Bäume werden im Spiel auf ≈0,4 verkleinert, mehr Texturspeicher und mögliches Flimmern auf der Xbox; mit US3 klären.
