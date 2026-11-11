# B-778 · Der Monarch wird im Spiel live als Gliederpuppe gezeichnet und ist mit Reittier, Rüstung und Waffe frei kombinierbar

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Jede Kombination des Monarchen wird heute als eigenes PNG-Sheet gerendert (`tools/paper`, Ausgabe `public/grafik/figuren/monarch/`). Gezählt am 2026-11-11: Der Monarch belegt **210 von 569 Sheets und 387 von 757 MiB RGBA (51 %)** der gesamten Grafik. 29 der 42 Grund-Animationen sind „Gangart + Reittier“ (`galopp-rappe`, `trab-einhorn` …), jede zusätzlich in vier Spielerfarben (`@spieler1`–`4`). Rüstungen und Waffen als Grafik gibt es noch nicht; mit Sheets würden 3 Rüstungen × 4 Waffen den Monarchen auf das 12-Fache bringen (rund 4,6 GiB, 2.500 Sheets). Die Specs (`public/grafik/**/*.paper.json`) beschreiben die Figuren bereits als Rig: Teile mit Drehpunkt (`pivot`) und Eltern-Teil (`eltern`), Vektor-Lagen mit Material und Ton; die Bewegung rechnet `tools/paper/rig/` (Keyframe-Winkel, Gangarten, Beinpfade, `reiten.ts`, `umhang.ts`), der Papier-Look nutzt einen Schlagschatten-Filter (`tools/paper/render/svg.ts`).

## Ziel

Monarch, Reittier, Rüstung und Waffe lassen sich frei kombinieren, ohne dass die Grafikmenge mit jeder Option multipliziert wächst; Bewegungen laufen so flüssig wie die Bildrate.

## Beteiligte und Zielgruppen

🧑 Ralf (entscheidet Look und Umfang, nimmt ab), Spieler (sehen ihren Monarchen mit gewählter Ausrüstung), Claude (Umsetzung).

## Anforderungen

- `tools/paper` exportiert je Teil ein kleines Bild samt Drehpunkt, Eltern-Teil und Zeichenreihenfolge (Teile-Atlas) statt fertiger Posen.
- Die Bewegung wird als Daten exportiert (Winkel und Sichtbarkeit je Teil und Animation), damit die Bewegungslogik nur einmal existiert (in `tools/paper`) und das Spiel sie nur abspielt.
- Das Spiel setzt Monarch, Reittier, Rüstung und Waffe zur Laufzeit zusammen; der Sitzpunkt kommt aus dem Rig des Reittiers.
- Spielerfarben entstehen zur Laufzeit (Einfärbung), nicht als eigene Dateien.
- Der Papier-Look (Schlagschatten je Lage) bleibt erkennbar gleich.
- Zwei Spieler gleichzeitig mit verschiedenen Kombinationen.
- Alle anderen Figuren (Truppen, Gegner, Gebäude) bleiben vorerst PNG-Sheets.

## Nicht-Ziele

Vektor-Zeichnen jeden Frame auf der Grafikkarte (zu teuer, Schatten-Filter). Umstellung aller Figuren. Regeln und Werte für Rüstungen und Waffen (REG, eigenes Ticket). Die Engine-Wahl (B-775).

## Regeln und Einschränkungen

Entscheidung 005: Assets nur aus Specs, Änderungen über MCP `assets_*`, Review in der Workbench. Engine: Go + Ebitengine (🧑 2026-11-11, B-775). Im Plan vor HUD und Menüs.

## Beispiele

Spieler 1 reitet einen Rappen in Plattenrüstung mit Schwert, Spieler 2 ein Einhorn in Lederrüstung mit Bogen → beide Kombinationen entstehen aus denselben Teile-Atlanten, ohne neues Rendern. Eine neue Rüstung = ein Satz Teile (wenige hundert KiB) statt hunderte Sheets.

## Ausnahme- und Fehlerfälle

Umhang: wird ohne Physik und ohne Verformung als **festes Teil** der Gliederpuppe gebaut (Entscheidung 🧑 2026-11-11), bewegt sich also nur mit seinem Eltern-Teil und über Keyframe-Winkel. Fehlt ein Teil einer Rüstung → Grundteil des Monarchen wird gezeichnet, Meldung im Log.

## Akzeptanzkriterien

- **AC-01** Prototyp: ein Monarch als Gliederpuppe auf zwei verschiedenen Reittieren mit einer umschaltbaren Test-Rüstung, neben dem heutigen Sheet-Monarchen zum Vergleich (🧑 beurteilt den Look).
- **AC-02** `tools/paper` exportiert Teile-Atlas und Bewegungsdaten für Monarch und Reittiere; Tests decken das Format ab.
- **AC-03** Grafikspeicher des Monarchen mit allen Reittieren liegt gemessen unter 40 MiB (heute 387 MiB).
- **AC-04** Zwei Spieler mit verschiedenen Kombinationen laufen gleichzeitig, Gangwechsel ohne Sprung.

## Offene Fragen

- Reicht der Schlagschatten als eingebackener Teil der Teile-Bilder, oder braucht es einen Shader? Klärt der Prototyp, 🧑 beurteilt.
- Review und Goldens: Was prüft die Workbench künftig, wenn das Spiel live zusammensetzt (gerenderte Referenzposen oder Teile)? Entscheidet 🧑.
- Welche Rüstungen und Waffen gibt es überhaupt (Regelwerk)? Entscheidet 🧑, eigenes REG-Ticket.

## Notizen

Aufwand (Einschätzung): etwa eine Woche für Export, Monarch und Reittiere. Nutzen neben der Kombinierbarkeit: flüssige Bewegung statt 8–14 Bilder/s, weiche Übergänge zwischen Animationen, Spec-Änderungen an Figuren ohne `gen:paper`. Herkunft: Chat 2026-11-11 im Zuge von NV1.
