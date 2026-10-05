# B-010 · Gebäude, Ressourcen und Hintergrund haben Grafiken

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** GR3
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 2, durch 🧑; umfasst B-010; nachgeschärft: Lizenz CC0/CC-BY, „zugeordnet“ statt „alle“, Portale/Truhen/Münzen, Stufen erst mit B-112

## Ausgangslage

Bisher sind Gebäude, Ressourcen und Hintergrund Platzhalter-Formen.

## Ziel

Gebäude, Ressourcen und Hintergrund haben Grafiken. Nutzen: Das Spiel soll am TV nach einem Spiel aussehen.

## Beteiligte und Zielgruppen

Spieler am TV (Edge auf der Xbox) und am Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Pixel-Art für alle Gebäude, Ressourcen und Parallax-Ebenen je Biom.
- Nur CC0- oder CC-BY-Grafiken, Credits in `public/`.
- Gezeichnet wird, was `docs/assets/zuordnung.md` als „zugeordnet“ führt; Lücken behalten die Platzhalter-Form.

## Nicht-Ziele

Figuren (vorhanden) und Grafiken für neue Mechaniken.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Seiten-Regeln aus `CLAUDE.md`; Taste B nicht belegen; 2 Spieler gleichzeitig (Split-Screen).

## Beispiele

Mauer im Hub → wird als Sprite statt als Platzhalter-Form gezeichnet.

## Ausnahme- und Fehlerfälle

Grafik fehlt für ein Gebäude → die Platzhalter-Form bleibt sichtbar, keine leere Stelle.

## Akzeptanzkriterien

- **AC-01** Alle Gebäude, Ressourcen und Parallax-Ebenen je Biom, die in `docs/assets/zuordnung.md` „zugeordnet“ sind, werden als Pixel-Art gezeichnet (CC0 oder CC-BY); Lücken behalten die Platzhalter-Form.
- **AC-02** Die Credits stehen in `public/`.

## Offene Fragen

keine

## Notizen

Stand 2026-10-01: Zwölf CC0-Packs (Warped Caves: CC BY 3.0) liegen unter `public/grafik/` und sind auf `grafiken.html` ansehbar (B-087, erledigt). Eingebaut ist noch nichts; welche Grafik wofür dient, steht seit GR1 in `docs/assets/zuordnung.md` (Lücken schließt GR2).
Lücken ohne Treffer: Mine-Hintergrund, Rekrutierungslager, Werkstatt, Farm, Kaserne, Treppen. Recherche: `docs/funde/b010-grafik-funde.html`.
