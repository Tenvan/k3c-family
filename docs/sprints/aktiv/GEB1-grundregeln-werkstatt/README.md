# GEB1 · DEV · Grundregeln für Gebäude-Grafik und Werkstatt

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-435
- **Start-Commit:** aa6838cd
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-16, Chat (Ralf), Revision 1

## Ausgangslage

Gebäude bestehen aus gemischten Grafikpacks. Ein Werkstatt-Entwurf (Handelsstand mit Gestell, 192×128 px, DB32) ist von 🧑 am 2026-10-15 als Stilvorlage angenommen (B-435). Die Quelle liegt noch außerhalb des Repos.

## Ziel

Grundregeln für Gebäude-Grafik stehen fest und sind an der Werkstatt belegt. Am Ende sichtbar: Die Werkstatt (Stufe 1) erscheint in der Vorschau des Asset-Managers im Spielmaßstab, mit Regeldatei im Repo, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

🧑 entscheidet Stil und Dichte und nimmt am Bildschirm ab; Agenten bauen Generator, Daten und Vorschau; künftige GEB-Sprints nutzen Regeldatei und Generator für weitere Gebäude.

## Anforderungen

B-435 › Anforderungen (dort auch die Dichte-Regel). Sprint-eigen: Die Regeln nennen Kachelmaße je Gebäudeklasse, Palette (DB32), Lichtrichtung, Materialsprache (Holz/Stoff), Baustein-Liste und Ausbaustufen-Mechanik. Die Regeln stehen in einer Datei unter `docs/standards/`. Ein Gebäude ist eine Daten-Datei (JSON), das Build-Werkzeug `tools/gebaeude/` (Go) erzeugt daraus das PNG deterministisch (gleiche Daten → gleiche Pixeldaten).

**Dichte-Regel (Chat 2026-10-15, Ralf; belegt am Werkstatt-Vergleich 16/32/64 px):**
- Die Quelle speichert alle Formen in **Kachel-Einheiten**, nicht in Pixeln. Die Pixeldichte ist ein Render-Parameter des Build-Werkzeugs.
- **Master-Dichte: 32 px je Kachel** (= `UNIT_PX`). In dieser Dichte wird die Quelle gezeichnet, geprüft und abgenommen (bestätigt im Workshop GEB1.1).
- **Erlaubte Dichten** sind das ½-, 1-, 2- und 4-fache der Master-Dichte (16, 32, 64, 128 px je Kachel), alle anderen Werte werden abgelehnt; kein Anti-Aliasing, feste DB32-Farben.
- **Feinere Dichten** (64, 128 px) entstehen automatisch und ohne Verlust: gleiches Bild, feinere Pixel, kein neues Detail.
- **Gröbere Dichte** (16 px) ist nur mit bewusst vereinfachter Detail-Ebene zulässig: Die Quelle markiert Details als `fein` (z. B. einzelne Werkzeuge, Faltenlicht, Plankenfugen), die bei der gröberen Dichte entfallen oder zusammengefasst werden. Die Mindestbreite jeder Linie ist 1 px der Zieldichte.
- Jede neue Dichte wird von 🧑 in der Vorschau abgenommen, bevor sie im Spiel verwendet wird.

**Skalierung (Workshop GEB1.1):** Ganzzahlige Skalierung gilt für Figuren, Gebäude und Objekte gemeinsam. Den Faktor liefert GR7.4 (B-251); die Regeldatei führt ihn bis dahin als offen.

## Nicht-Ziele

Weitere Gebäude (je eigener Sprint im Projekt GEB); weitere Baumvarianten und Level-Paletten (feilt 🧑 später aus); Animationen; vollautomatische Prompt-Erzeugung im Asset-Manager (nur Vorschau und Regel-Rahmen); Figuren (GR8).

## Regeln und Einschränkungen

- Domäne DEV: `tools/gebaeude/`, `public/grafik/gebaeude/`, `src/tools/assetvorschau*.ts`, `docs/standards/`, `Taskfile.yml` (nur Ziel `gebaeude`). Spiel-Code (`src/scenes/`, `engine/`) und `data/assets.json` bleiben unberührt; die Zuordnung der Werkstatt ins Spiel ist ein eigener Sprint.
- Determinismus: kein Zufall im Generator; gleiche JSON → byte-gleiches PNG (wie `tools/atlas`, B-163).
- Keine neue Abhängigkeit (Go stdlib `image`, `image/png`, `encoding/json`).
- Budget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

- `task gebaeude` mit `werkstatt.json` → `werkstatt.png` 192×128 px (6×4 Kacheln à 32 px); zweiter Lauf ändert keine Datei.
- Dieselbe Quelle mit Dichte 16 → 96×64 px, ohne `fein`-Details (z. B. keine einzelnen Werkzeuge am Gestell).
- Asset-Vorschau mit Slot `workshop` → Werkstatt 32 px und 16 px ×2 nebeneinander.

## Ausnahme- und Fehlerfälle

- Dichte nicht in {16, 32, 64, 128} → Abbruch mit Meldung, die die erlaubten Werte nennt; kein PNG.
- Farbe in der JSON nicht aus DB32 → Abbruch mit Meldung (Datei, Baustein, Farbe).
- Form außerhalb der Gebäudemaße → Abbruch mit Meldung statt stillem Abschneiden.
- Linie unter 1 px der Zieldichte bei 16 px, die nicht `fein` ist → Abbruch mit Meldung.

## Akzeptanzkriterien

- **AC-01** Eine Regeldatei `docs/standards/gebaeude-grafik.md` nennt Raster, Maße, Palette, Licht, Bausteine, Stufenmechanik, die Dichte-Regel und die Skalierungsregel (ganzzahlig, gilt für Figuren, Gebäude und Objekte; Faktor = Ergebnis GR7.4/B-251, bis dahin dort als offen geführt) (B-435/Anforderungen; Skalierung: Workshop GEB1.1).
- **AC-02** Die Werkstatt liegt als Daten-JSON (Kachel-Einheiten, Details mit Markierung `fein`) und PNG in `public/grafik/gebaeude/`, das Build-Werkzeug in `tools/gebaeude/` (`task gebaeude`), in der Master-Dichte 32 px je Kachel, ohne Glättung; zweimal bauen ergibt byte-gleiche PNGs.
- **AC-03** Die Werkstatt erscheint in der Asset-Vorschau im Spielmaßstab (1 Kachel = 32 px).
- **AC-04** 🧑 hat die Werkstatt am Bildschirm abgenommen, in der Master-Dichte und in jeder weiteren Dichte, die im Spiel verwendet werden soll, und entschieden, welche Dichte das Spiel zeigt (32 px ×1 oder 16 px ×2, Workshop GEB1.1 Frage 3).
- **AC-05** Drei schmale Bäume (Tanne, Pappel, Birke, je 1 Kachel breit) liegen als Daten und PNG im Repo, erscheinen in der Vorschau als überlappende Reihe und folgen denselben Regeln (B-435/Beispiele).
- **AC-06** Das Build-Werkzeug rendert dieselbe Quelle in den erlaubten Dichten (Parameter px je Kachel: 16, 32, 64, 128): `go test`: Die 64-px-Ausgabe ist nach Nearest-Downsampling (je 2×2 Pixel) pixelgleich zur 32-px-Ausgabe; ein anderer Dichte-Wert wird mit Fehlermeldung abgelehnt. Die Vorschau zeigt Master- und gröbere Dichte nebeneinander (16 px ohne die als `fein` markierten Details).

## Offene Fragen

- Entschieden 2026-10-15 (Chat, Ralf): Skript-Generator im Repo. Die Beschreibung wird in Daten (JSON: Maße, Bausteine, Palette, Stufe) übersetzt, ein Build-Werkzeug erzeugt daraus das PNG.
- Entschieden 2026-10-15 (Chat, Ralf): Ablage unter `public/grafik/gebaeude/` (PNG, Daten-JSON daneben).
- Entschieden 2026-10-15 (Chat, Ralf): Dichte-Regel (Quelle in Kachel-Einheiten, Master 32 px, feiner automatisch, gröber nur mit Detail-Vereinfachung).
- Entschieden 2026-10-16 (Workshop GEB1.1, Ralf): Das Build-Werkzeug ist Go unter `tools/gebaeude/` nach dem Vorbild `tools/atlas` (stdlib `image/png`, feste EXE `bin/gebaeude`, `task gebaeude`); ersetzt die frühere Festlegung `scripts/`.
- Entschieden 2026-10-16 (Workshop GEB1.1, Ralf), Skalierung am TV:
  1. Flimmern der Figuren: **noch nicht geprüft**. Der Faktor kommt aus GR7.4 (B-251, Agent: Mensch).
  2. Ganzzahlige Skalierung gilt **für alles** (Figuren, Gebäude, Objekte; ein Faktor, ein Pixelraster).
  3. Master bleibt **32 px je Kachel**. Ob das Spiel Gebäude in 32 px ×1 (feiner als die Figuren) oder 16 px ×2 (Figurdichte, wie die heutigen Gebäude-Packs) zeigt, entscheidet 🧑 in GEB1.4 am Vergleich in der Vorschau.
  4. Beobachtung am TV in **4K, Kamera-Zoom 1**. Ungeprüft: Edge auf der Xbox rendert die Seite vermutet mit 1080p-Viewport und `devicePixelRatio` 2; dann ist ein CSS-Pixel 2×2 Geräte-Pixel.
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB1.1 | `GEB1.1-workshop-grundregeln.md` | Workshop | Mensch | fertig |
| GEB1.2 | `GEB1.2-werkstatt-generator.md` | Umsetzung | autonom | fertig |
| GEB1.3 | `GEB1.3-baeume.md` | Umsetzung | autonom | fertig |
| GEB1.4 | `GEB1.4-review-abnahme.md` | Review | autonom | offen |

- GEB1.1 Grundregeln aus Werkstatt und Bäumen festlegen, Skalierungsfragen klären (AC-01).
- GEB1.2 Werkstatt: Daten, Build-Skript mit Mehrfach-Dichte, PNG, Vorschau im Asset-Manager (AC-02, AC-03, AC-06).
- GEB1.3 Drei Bäume als Daten und PNG, Reihe in der Vorschau (AC-05).
- GEB1.4 Review und Abnahme durch 🧑 am Bildschirm (AC-04, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
