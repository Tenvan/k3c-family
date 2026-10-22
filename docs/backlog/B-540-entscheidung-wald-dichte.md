# B-540 · Entscheidung D3: Wald-Dichte und Höhlenbüsche einplanen, Werte setzt Ralf

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KR2
- **Projekt:** GEB
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Mehr Bäume je Abschnitt ist ein Wert in `data/biomes/forest.json` › `resourcesPerChunk` (Holz-Balancing, Golden betroffen); der Höhlenbusch aus GEB10 erscheint im Spiel nicht, weil die Höhle keine Büsche erzeugt. Umsetzungs-Ticket: B-531.

## Ziel

🧑 legt die Baumzahl fest und sagt, ob Höhle (und Mine) Büsche bekommen.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Werte in `data/`; Balancing-Änderung mit Golden und `sim_test`.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen die Baumzahl je Abschnitt und die Büsche je Biom genannt oder „so lassen“.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Wie viel dichter soll der Wald werden, und bekommen Höhle und Mine Deko-Büsche?

- **Default:** B-531 ist eingeplant (Rang im Projekt GEB nach GEB12), aber ohne Werteänderung, bis 🧑 Zahlen nennt. Vorschlag zum Abnicken: Wald `tree [6, 9]` statt `[4, 7]`, Höhle `bush [1, 3]`, Mine ohne Büsche.
- **Alternative:** so lassen (Wald wirkt über breitere Kronen dichter), B-531 verwerfen.
- **Wenn 🧑 Werte nennt:** B-531 wird mit diesen Werten umgesetzt (REG, `task go:golden` mit Begründung, `sim_test` für Holz).
