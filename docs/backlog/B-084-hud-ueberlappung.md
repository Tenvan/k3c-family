# B-084 · HUD-Texte überlappen im 2×2-Raster

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beim Browser-Lauf zu SP08 (2026-10-01): Im 2×2-Raster liegen die Texte `Holz`, Tageszeit und Kampfanzeige oben rechts im Feld von Spieler 2,
der Steuerungshinweis unten rechts im Feld von Spieler 4 bzw. im Info-Feld bei 3 Spielern und deckt dort Welt oder Text zu.
Die Texte sind für Vollbild und zwei Streifen gesetzt (`src/scenes/HudScene.ts`).

## Ziel

Gemeinsame Anzeigen und Hinweise liegen auch bei 3–4 lokalen Spielern lesbar und verdecken kein Spielerfeld.

## Beteiligte und Zielgruppen

Spieler am TV; 🧑 entscheidet über das Aussehen.

## Anforderungen

- Gemeinsame Anzeigen (Vorrat, Tageszeit, Kampf) und der Steuerungshinweis wandern im Raster in die Mitte oder ins Info-Feld.

## Nicht-Ziele

Neues HUD-Design.

## Regeln und Einschränkungen

Domäne CLI; `src/scenes` zeichnet nur.

## Beispiele

4 Spieler → die gemeinsamen Anzeigen liegen mittig über dem Kreuzpunkt der Felder, nicht in einer Spielerecke.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Darstellung.

## Akzeptanzkriterien

- **AC-01** Bei 3 und 4 lokalen Spielern überdeckt kein gemeinsamer Text ein Spielerfeld (Beobachtung durch 🧑 am TV).

## Offene Fragen

keine

## Notizen

Gefunden beim Browser-Lauf zu SP08.
