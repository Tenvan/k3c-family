# B-460 · Der Welt-Betrachter im Asset-Werkzeug zeigt Chunks maximiert und eine Live-Vorschau mit Hub-Stufen und schaltbaren Gebäuden

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ7
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`leveltest.html` (`src/tools/leveltest.ts`, `levelView.ts`, `levelApi.ts`) zeigt ein generiertes Level (Seed, Biom) als Streifen aus Chunks mit Objekten (B-092) und erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut (B-315). Die Chunk-Vorschau ist klein und nicht maximiert; eine Ansicht der Welt mit echter Grafik gibt es nur im Spiel.

## Ziel

Der Welt-Betrachter zieht ins Asset-Werkzeug (B-459): oben die Chunk-Vorschau, im Standard maximiert; darunter eine Live-Vorschau der Welt mit echter Grafik, in der Hub-Stufe 1–5 umschaltbar ist und sich einzelne Gebäude ein- und ausschalten lassen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Chunks planen oder festlegen (B-461); Freischaltung je Hub-Stufe bearbeiten (B-462); Spielen in der Vorschau (keine Eingabe, keine Simulation über die Zeit).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Der Welt-Betrachter im Asset-Werkzeug zeigt Seed, Biom, Chunks und Objekte wie `levelView.ts` und erzeugt den voll ausgebauten Spielstand (B-315).
- **AC-02** Die Chunk-Vorschau ist beim Öffnen maximiert und lässt sich verkleinern.
- **AC-03** Die Live-Vorschau zeichnet das Level mit der Spielgrafik (`worldRenderer`), die Hub-Stufe ist über alle Stufen aus `data/hub.json` › `levels` umschaltbar (heute 1–5, mehr mit B-458), Gebäude einzeln ein- und ausschaltbar; gleiche Eingaben → gleiches Bild.
- **AC-04** `leveltest.html` bleibt für Sofa und Controller (B-092, HW1) oder wird entfernt bzw. verweist auf das Asset-Werkzeug – Entscheidung 🧑; Entfernen ist eine PLAT-Session (`*.html`, `src/tools/devTiles.ts`).

## Offene Fragen

1. Woher kommt der `World` der Live-Vorschau? `worldRenderer` braucht einen vollen `World`; der Generator liefert nur ein Layout, der Voll-Ausbau liegt in SRV (`engine/room/fullsave.go`, `fullIsland` nicht exportiert). Annahme: SRV-Endpunkt bzw. exportierte Funktion liefert einen Snapshot für Seed, Biom und Hub-Stufe (eigene SRV-Session).
2. Gebäude „ein/aus“: nur Darstellung oder Spielstand mit/ohne Gebäude erzeugen? Annahme: nur Darstellung.
3. Zeigt die Live-Vorschau auch Tag/Nacht und Hintergrund-Schichten? Annahme: ja, Tag fest.
4. „Im Spiel starten“ aus `leveltest.html` geht über `openPage` der Shell; im Asset-Werkzeug (Desktop) gibt es keine Shell. Annahme: Link öffnet das Spiel im Browser.
5. Begriff: bisher „Level-Betrachter“ (B-092, `levelView.ts`), neu „Welt-Betrachter“ – ein Name ins Glossar.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
