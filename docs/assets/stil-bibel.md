# Stil-Bibel der Grafiken

Gilt für jede Grafik im Spiel, aus Packs, eigenem Zeichenraster oder Bild-KI (B-331). Umgesetzt von `task gen:pipeline`
(`tools/pipeline`). Stand: Entwurf (GR8.2, autonom, Claude); Outline- und Lichtregel gibt 🧑 frei (B-331/AC-01).

## Ziel-Palette: DB32

DawnBringer 32 (Lospec), dieselben Werte wie `tools/buildings/db32.go`. Jedes deckende Pixel eines Ergebnisses hat genau
eine dieser Farben; der Go-Test prüft das für jedes Ergebnis.

| | | | | | | | |
|---|---|---|---|---|---|---|---|
| `#000000` | `#222034` | `#45283c` | `#663931` | `#8f563b` | `#df7126` | `#d9a066` | `#eec39a` |
| `#fbf236` | `#99e550` | `#6abe30` | `#37946e` | `#4b692f` | `#524b24` | `#323c39` | `#3f3f74` |
| `#306082` | `#5b6ee1` | `#639bff` | `#5fcde4` | `#cbdbfc` | `#ffffff` | `#9badb7` | `#847e87` |
| `#696a6a` | `#595652` | `#76428a` | `#ac3232` | `#d95763` | `#d77bba` | `#8f974a` | `#8a6f30` |

Abbildung: nächste Farbe im OKLab-Farbraum, ohne Dithering; bei Gleichstand die frühere Farbe der Tabelle.

## Pixeldichte: 16 px

Ein Quell-Pixel ist ein Spiel-Pixel, gezeichnet mit ×2 (Stilbeschluss Q13, Entscheidung 🧑 2026-10-06).
Eine Kachel (1 Unit) ist 16 Quell-Pixel breit.

- **Hochskalierung erkennen:** größter gemeinsamer Teiler aller waagerechten und senkrechten Lauflängen gleicher Farbe;
  das Bild wird um diesen Faktor verkleinert. Gilt nur, wenn die kürzere Seite danach noch mindestens 8 px hat (sonst ist
  eine flache Kachel, z. B. ein einfarbiger Schatten, kein Hinweis auf Hochskalierung).
- **Raster der Quelle:** Ein Pack mit 32-px-Raster (`"dichte": 32` in der Liste) wird zusätzlich halbiert, 48 px gedrittelt,
  64 px (z. B. PixelLab) geviertelt. Je Block gilt die häufigste Farbe (Gleichstand: deckend vor transparent).
- Bild-KI-Assets laufen ohne Sonderweg durch: Eintrag in die Liste mit ihrem Raster.

## Alpha: hart

Alpha ≥ 128 wird deckend, darunter transparent. Weiches Alpha (Schatten, Glühen) nur über die Ausnahme `alpha-weich`.

## Outline

1 px dunkle Kontur aus DB32 (bevorzugt `#222034`, `#000000` oder `#45283c`), wo die Quelle eine Kontur hat. Die Pipeline
zeichnet keine Kontur nach und entfernt keine: Sie bildet nur die vorhandene Kontur auf DB32 ab. Neue Zeichnungen und
Bild-KI-Assets bekommen die Kontur bereits in der Quelle.

## Licht

Licht fällt von **oben links**: helle Kanten oben und links, Schatten unten und rechts. Die Pipeline prüft das nicht; es ist
die Vorgabe für neue Zeichnungen, Bild-KI-Prompts und die Auswahl von Packs.

## Liste und Ausnahmen

`docs/assets/pipeline-liste.json`: Pfad unter `public/grafik/` → Optionen. Das Ergebnis liegt unter demselben Pfad in
`public/grafik-db32/`; die Quelle samt `LICENSE.txt` und `CREDITS.md` bleibt unverändert und gültig.

| Option | Werte | Wirkung |
|---|---|---|
| `dichte` | 16 (Standard), 32, 48, 64 … | Raster der Quelle in px |
| `ausnahme` | `alpha-weich` | Alpha bleibt weich, Farben trotzdem DB32 |
| `ausnahme` | `quelldichte` | nur die Hochskalierung zurücknehmen, nicht auf 16 px verkleinern (Asset verliert sonst Details) |
| `grund` | Text | Begründung der Ausnahme |

Derzeit gilt keine Ausnahme. Bekannte Fälle zur Prüfung durch 🧑:

- `plants-and-flowers-pixel-art/kandidaten/greentree.png`: 10-fach hochskaliert, schrumpft auf 16×16 (Topfpflanze statt
  Baum). Lücke oder neu zeichnen (B-331, offene Frage).
- `castle-set/kandidaten/castle-tower.png` und `building-block-assets-stone-logs-bricks/kandidaten/woodwall.png` haben
  weiches Alpha an den Kanten, das jetzt hart wird.
- Gothicvania-Violett (Dächer, Schatten) fällt teils auf Grau-Braun; Feinabstimmung je Asset wäre eine neue Option.

## Ablauf für ein neues Asset

1. Datei mit Lizenz nach `public/grafik/<pack>/` legen, Eintrag in `CREDITS.md`.
2. Pfad mit Raster in `docs/assets/pipeline-liste.json` eintragen.
3. `task gen:pipeline`, Ergebnis auf `pipeline.html` (Entwicklerseite) prüfen.
