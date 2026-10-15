# GEB1 · DEV · Grundregeln für Gebäude-Grafik und Werkstatt

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-435
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gebäude bestehen aus gemischten Grafikpacks. Ein Werkstatt-Entwurf (Handelsstand mit Gestell, 192×128 px, DB32) ist von 🧑 am 2026-10-15 als Stilvorlage angenommen (B-435). Die Quelle liegt noch außerhalb des Repos.

## Ziel

Grundregeln für Gebäude-Grafik stehen fest und sind an der Werkstatt belegt. Am Ende sichtbar: Die Werkstatt (Stufe 1) erscheint in der Vorschau des Asset-Managers im Spielmaßstab, mit Regeldatei im Repo, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-435 › Anforderungen (dort auch die Dichte-Regel). Sprint-eigen: Die Regeln nennen Kachelmaße je Gebäudeklasse, Palette (DB32), Lichtrichtung, Materialsprache (Holz/Stoff), Baustein-Liste und Ausbaustufen-Mechanik. Die Regeln stehen in einer Datei unter `docs/standards/`. Ein Gebäude ist eine Daten-Datei (JSON), ein Build-Skript im Repo erzeugt daraus das PNG deterministisch (gleiche Daten → gleiche Pixeldaten).

**Dichte-Regel (Chat 2026-10-15, Ralf; belegt am Werkstatt-Vergleich 16/32/64 px):**
- Die Quelle speichert alle Formen in **Kachel-Einheiten**, nicht in Pixeln. Die Pixeldichte ist ein Render-Parameter des Build-Skripts.
- **Master-Dichte: 32 px je Kachel** (= `UNIT_PX`). In dieser Dichte wird die Quelle gezeichnet, geprüft und abgenommen. Vorbehalt: Die Entscheidung zur Pixelgröße relativ zu den Figuren (Offene Frage 3) kann den Master ändern.
- **Erlaubte Dichten** sind das ½-, 1-, 2- und 4-fache der Master-Dichte (16, 32, 64, 128 px je Kachel), alle anderen Werte werden abgelehnt; kein Anti-Aliasing, feste DB32-Farben.
- **Feinere Dichten** (64, 128 px) entstehen automatisch und ohne Verlust: gleiches Bild, feinere Pixel, kein neues Detail.
- **Gröbere Dichte** (16 px) ist nur mit bewusst vereinfachter Detail-Ebene zulässig: Die Quelle markiert Details als `fein` (z. B. einzelne Werkzeuge, Faltenlicht, Plankenfugen), die bei der gröberen Dichte entfallen oder zusammengefasst werden. Die Mindestbreite jeder Linie ist 1 px der Zieldichte.
- Jede neue Dichte wird von 🧑 in der Vorschau abgenommen, bevor sie im Spiel verwendet wird.

**Skalierungs-Prüfpunkt (B-251, GR7.4):** Gebäude müssen am TV ganzzahlig skaliert flimmerfrei bleiben. Die Regeldatei nennt den vom Spiel verwendeten Skalierungsfaktor nach Entscheidung von 🧑 (B-251/GR7.4 betrifft nur Figuren; offen sind Geltung für Gebäude, Pixelgröße relativ zu den Figuren, TV-Auflösung und Kamera-Zoom, siehe Offene Fragen).

## Nicht-Ziele

Weitere Gebäude (je eigener Sprint im Projekt GEB); weitere Baumvarianten und Level-Paletten (feilt 🧑 später aus); Animationen; vollautomatische Prompt-Erzeugung im Asset-Manager (nur Vorschau und Regel-Rahmen); Figuren (GR8).

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Eine Regeldatei `docs/standards/gebaeude-grafik.md` nennt Raster, Maße, Palette, Licht, Bausteine, Stufenmechanik, die Dichte-Regel und den Skalierungsfaktor nach Entscheidung von 🧑 (B-435/Anforderungen; Faktor: sprint-eigen).
- **AC-02** Die Werkstatt liegt als Daten-JSON (Kachel-Einheiten, Details mit Markierung `fein`), Build-Skript und PNG im Repo (`public/grafik/gebaeude/`, `scripts/`), in der Master-Dichte 32 px je Kachel, ohne Glättung.
- **AC-03** Die Werkstatt erscheint in der Asset-Vorschau im Spielmaßstab (1 Kachel = 32 px).
- **AC-04** 🧑 hat die Werkstatt am Bildschirm abgenommen, in der Master-Dichte und in jeder weiteren Dichte, die im Spiel verwendet werden soll.
- **AC-05** Drei schmale Bäume (Tanne, Pappel, Birke, je 1 Kachel breit) liegen als Daten und PNG im Repo, erscheinen in der Vorschau als überlappende Reihe und folgen denselben Regeln (B-435/Beispiele).
- **AC-06** Das Build-Skript rendert dieselbe Quelle in den erlaubten Dichten (Parameter px je Kachel: 16, 32, 64, 128): Test: Die 64-px-Ausgabe ist nach Nearest-Downsampling (je 2×2 Pixel) pixelgleich zur 32-px-Ausgabe; ein anderer Dichte-Wert wird mit Fehlermeldung abgelehnt; die Vorschau zeigt Master- und gröbere Dichte nebeneinander (16 px ohne die als `fein` markierten Details).

## Offene Fragen

- Entschieden 2026-10-15 (Chat, Ralf): Skript-Generator im Repo. Die Beschreibung wird in Daten (JSON: Maße, Bausteine, Palette, Stufe) übersetzt, ein Build-Skript erzeugt daraus das PNG.
- Entschieden 2026-10-15 (Chat, Ralf): Ablage unter `public/grafik/gebaeude/` (PNG, Daten-JSON daneben), Build-Skript in `scripts/`.
- Entschieden 2026-10-15 (Chat, Ralf): Dichte-Regel (Quelle in Kachel-Einheiten, Master 32 px, feiner automatisch, gröber nur mit Detail-Vereinfachung).
- **Blockierend für AC-01 (Skalierungsfaktor), nicht für GEB1.2/GEB1.3** (🧑, Skalierung am TV; B-251/GR7.4 betrifft nur Figuren):
  1. Stört das Flimmern der Figuren am TV, und falls ja ×2 oder ×3?
  2. Gilt die ganzzahlige Skalierung auch für Gebäude und Objekte?
  3. Pixelgröße der Gebäude relativ zu den Figuren: Soll die Master-Dichte 32 px je Kachel bleiben (feiner als die Figuren) oder sollen die Figuren auf 32 px je Kachel angehoben bzw. die Gebäude auf Figurdichte gebracht werden? Die Antwort kann die Master-Dichte ändern.
  4. TV-Auflösung (1080p, 4K) und Kamera-Zoom bei der Beobachtung?
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB1.1 | – (Entwurf) | Workshop | Mensch | offen |
| GEB1.2 | – (Entwurf) | Umsetzung | autonom | offen |
| GEB1.3 | – (Entwurf) | Umsetzung | autonom | offen |
| GEB1.4 | – (Entwurf) | Review | autonom | offen |

- GEB1.1 Grundregeln aus Werkstatt und Bäumen festlegen, Skalierungsfragen klären (AC-01).
- GEB1.2 Werkstatt: Daten, Build-Skript mit Mehrfach-Dichte, PNG, Vorschau im Asset-Manager (AC-02, AC-03, AC-06).
- GEB1.3 Drei Bäume als Daten und PNG, Reihe in der Vorschau (AC-05).
- GEB1.4 Review und Abnahme durch 🧑 am Bildschirm (AC-04, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
