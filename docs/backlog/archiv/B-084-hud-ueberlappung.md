# B-084 · HUD-Texte überlappen im 2×2-Raster

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-01 🧑 Chat („mach B-083 und B-084“, Revision 2; Revision 3 folgt dem 3-Spieler-Layout aus B-016 Revision 3)

## Ausgangslage

Beim Browser-Lauf zu SP08 (2026-10-01): Im 2×2-Raster liegen die Texte `Holz`, Tageszeit und Kampfanzeige oben rechts im Feld von Spieler 2,
der Steuerungshinweis unten rechts im Feld von Spieler 4 bzw. im Info-Feld bei 3 Spielern und deckt dort Welt oder Text zu.
Die Texte sind für Vollbild und zwei Streifen gesetzt (`src/scenes/HudScene.ts`).

## Ziel

Gemeinsame Anzeigen und Hinweise liegen auch bei 3–4 lokalen Spielern lesbar und verdecken kein Spielerfeld.

## Beteiligte und Zielgruppen

Spieler am TV; 🧑 entscheidet über das Aussehen.

## Anforderungen

- Gemeinsame Anzeigen (Vorrat, Tageszeit, Kampf) stehen bei 3 und 4 Spielern mittig am Kreuzpunkt; der Steuerungshinweis bleibt am unteren Rand.

## Nicht-Ziele

Neues HUD-Design.

## Regeln und Einschränkungen

Domäne CLI; `src/scenes` zeichnet nur.

## Beispiele

4 Spieler → die gemeinsamen Anzeigen liegen mittig über dem Kreuzpunkt der Felder, nicht in einer Spielerecke.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Darstellung.

## Akzeptanzkriterien

- **AC-01** Die Position der gemeinsamen Anzeigen ist je Layout festgelegt (Vollbild/Streifen oben rechts, 3 und 4 Spieler mittig; Test der Positionsfunktion `sharedAnchor`).
- **AC-02** Bei 3 und 4 lokalen Spielern sind die Anzeigen lesbar und stehen in keiner Spielerecke (Beobachtung durch 🧑 am TV oder im Browser).

## Offene Fragen

keine

## Notizen

Gefunden beim Browser-Lauf zu SP08. Revision 2: AC-01 neu gefasst (ein Text über vier lückenlosen Feldern überdeckt immer etwas; Kreuzpunkt statt „überdeckt nichts“), AC-02 ist die Beobachtung. Umgesetzt: `sharedAnchor` in `src/scenes/layout.ts`, `HudScene.placeShared()`; AC-01 durch Test belegt, AC-02 steht aus, deshalb bleibt das Ticket offen. Im Browser-Pane beobachtet (Agent, Freigabe 🧑, 4 Spieler: Vorrat und Tageszeit mittig am Kreuzpunkt, lesbar); die Bestätigung durch 🧑 am TV oder Rechner fehlt noch.
