# WZ23 · CLI, DEV · Studio-Vorschauen: Reittiere aus dem Spielcode, Monarch-Seite, Boss-Anzeigen, Gebäude-Planer abgenommen

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** CLI, DEV
- **Reife:** bereit
- **Tickets:** B-476, B-471, B-502, B-469, B-501
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-22, autonom, Claude (Auftrag Ralf: autonom abarbeiten), Revision 1; Defaults: Reittiere auf Anforderung laden, Monarch-Sheets bleiben wie im Spiel, Event-Banner nur für Boss-Ereignisse

## Ausgangslage

Das Asset-Studio baut Laden und Animationen der 13 Reittiere selbst nach, weil `src/scenes/render/sprites.ts` nur das Standard-Reittier kennt (B-476, Abweichung von B-463/AC-02). `renderReference` (`src/tools/spriteReference.ts`) lässt alte Animationsschleifen weiterlaufen (B-471). Die Monarch-Seite nutzt zwei Sheets, hat keinen Zoom und steht einspaltig (B-502). Die Gegner-Seite zeigt Warnkreis, Boss-Leiste, Phasen und Event-Banner nur als Lücke, bis K5.1 den Zeichencode im Spiel liefert (B-469). Der Gebäude-Planer hat Live-Vorschau unten und Markierung schon bekommen, die visuelle Abnahme durch 🧑 steht aus (B-501).

## Ziel

Die Vorschauen im Studio zeigen mit dem Code des Spiels, was das Spiel zeigt, und lassen sich bequem vergleichen. Am Ende sichtbar: Reittiere, Monarchen mit Zoom und Boss-Anzeigen im Studio ohne nachgebauten Spielcode, Gebäude-Planer von 🧑 abgenommen.

## Beteiligte und Zielgruppen

🧑 im Asset-Studio; Agent in CLI (Zeichencode des Spiels) und DEV (Studio); 🧑 gibt die Spec frei und nimmt am PC ab.

## Anforderungen

B-476 › Ziel; B-471 › Anforderungen; B-502 › Anforderungen; B-469 › Anforderungen; B-501 › Ausgangslage (Abnahme des Umgesetzten). Specs noch Entwurf.

## Nicht-Ziele

Neue Animationen oder Sheets (GRA, B-505); Boss-Anzeigen im Spiel selbst (K5.1, B-132); Event-Banner für Nacht-Events, solange 🧑 nicht entschieden hat.

## Regeln und Einschränkungen

Reihenfolge CLI → DEV. **Abhängigkeit:** WZ23.3 (B-469) erst nach K5.1 (Sprint K5, Branch `sprint/k5`). Das Studio nutzt den Zeichencode des Spiels über `@game`, baut ihn nicht nach. Das Spiel lädt nicht mehr als heute (Atlas-Budget Xbox, GR4). Oberfläche nach Atomic Design.

## Beispiele

Studio zeigt Reittier `wolf` → geladen über `sprites.ts`, `loadFigureSheets` im Studio entfällt. Monarch-Seite: Alt + Mausrad zoomt, Skills links, Monarchen rechts. Endboss Phase 2 gewählt → Warnkreis mit `ability.radius`.

## Ausnahme- und Fehlerfälle

Reittier ohne Sheet → Platzhalter mit Warnung 🤒. Boss-Phase ohne `ability` → Fähigkeit der vorigen Phase. Schmales Fenster → Monarch-Seite untereinander.

## Akzeptanzkriterien

- **AC-01** `sprites.ts` lädt beliebige Reittiere auf Anforderung und legt ihre Animationen an, `createRider` nimmt das Reittier als Parameter; das Studio nutzt das und baut nichts nach (B-476).
- **AC-02** `renderReference` liefert eine Abmeldung, das Studio meldet beim Abbau ab (B-471/AC-01, B-471/AC-02).
- **AC-03** Die Monarch-Seite hat Zoom und zweispaltiges Layout; die Sheets folgen der Entscheidung von 🧑 (B-502).
- **AC-04** Die Gegner-Seite zeigt Warnkreis, Boss-Leiste, Phasen und Event-Banner mit dem Zeichencode des Spiels (B-469/AC-01 ff.).
- **AC-05** 🧑 hat Gebäude-Planer, Monarch- und Gegner-Seite am PC abgenommen oder Nachbesserungen als Ticket benannt (B-501).

## Offene Fragen

🧑: Reittiere im Spiel auf Anforderung laden (Vorschlag, B-476)? Monarch-Sheet nur im Studio vereinheitlichen oder auch im Spiel (B-502 › Offene Fragen 1)? Event-Banner nur für Boss-Ereignisse (Vorschlag, B-469 › Offene Fragen 2)?

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ23.1 | `WZ23.1-reittiere-auf-anforderung.md` | Umsetzung | autonom | fertig |
| WZ23.2 | `WZ23.2-studio-nutzt-spielcode.md` | Umsetzung | autonom | fertig |
| WZ23.3 | `WZ23.3-gegner-seite-boss.md` | Umsetzung | autonom | blockiert |
| WZ23.4 | `WZ23.4-review.md` | Review | autonom | fertig |
| WZ23.5 | `WZ23.5-abnahme-pc.md` | Workshop | Mensch | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ23.1 (CLI) `sprites.ts` lädt Reittiere auf Anforderung, `createRider` mit Parameter (AC-01).
- WZ23.2 (DEV) Studio nutzt das Laden des Spiels, `renderReference` mit Abmeldung, Monarch-Seite mit Zoom und zwei Spalten (AC-01, AC-02, AC-03).
- WZ23.3 (DEV) Gegner-Seite mit Boss-Anzeigen aus K5.1 (AC-04; erst nach K5.1).
- WZ23.4 (DEV) Review (Code-Sprint): alle Kriterien prüfen.
- WZ23.5 (DEV, Mensch) Abnahme am PC: Gebäude-Planer, Monarch- und Gegner-Seite (AC-05).

## Abnahme

2026-10-22 (Teil): AC-01 bis AC-03 nachgewiesen (WZ23.1, .2, .4), `check:diff` grün, Reviewer-Befund behoben. Sprint bleibt aktiv: AC-04 wartet auf K5.1 (WZ23.3 blockiert), AC-05 ist Ralfs Abnahme am PC (WZ23.5; B-469 und B-501 offen). Entscheidungen für den Nachgang: Reittiere auf Anforderung, Monarch-Sheets wie im Spiel.
