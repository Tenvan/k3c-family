# Gebäude-Grafik: Grundregeln

Gilt für alle Gebäude und Strukturen, die mit `tools/gebaeude` entstehen (Projekt GEB, B-435). Belegt an der
Werkstatt (`public/grafik/gebaeude/werkstatt.json`, Sprint GEB1). Die Regeln wachsen mit jedem Gebäude; Änderungen
nur mit Zustimmung von 🧑.

## 1. Raster und Maße

- **Kachel** = `UNIT_PX` = 32 px in der Master-Dichte. Alle Maße in Kacheln, Vorderansicht, Boden = Unterkante des Bilds.
- Koordinaten und Maße sind Vielfache von **1/32 Kachel** (1 px Master), auch Bruch-Maße wie 2,25. Andere Werte lehnt das Werkzeug ab.

| Klasse | Breite × Höhe (Kacheln) | Beleg |
|---|---|---|
| Handelsstand / kleines Gebäude | 6 × 4 | Werkstatt (`workshop`), Taverne (`tavern`), Heilplatz (`healer`), Schmiede (`smithy`), Rüstkammer (`armory`) (GEB2) |
| Baum (schmal) | 1 × 2,25 | Tanne, Pappel, Birke (GEB1.3) |
| weitere Gebäude | legt der jeweilige GEB-Sprint fest, hier nachtragen | B-435 › Notizen |

## 2. Palette und Licht

- Nur **DB32** (DawnBringer 32, Namen nach Lospec, Tabelle in `tools/gebaeude/db32.go`); dazu Transparenz. Kein Anti-Aliasing, keine Zwischenfarben.
- **Licht von links oben:** Kanten links und oben eine Stufe heller (z. B. `oiled-cedar` → `rope` → `twine` → `pancho`), Schatten unter Dächern und Brettern eine Stufe dunkler.

## 3. Materialsprache

| Material | Grund | Licht | Schatten / Fuge |
|---|---|---|---|
| Holz (Wand, Tresen, Gestell) | `rope` | `twine`, `pancho` | `oiled-cedar` |
| Pfosten, Balken | `oiled-cedar` | `rope` | – |
| Stoff rot | `brown` | `mandy` (Falte) | `loulou` |
| Stoff hell | `pancho` | `white` (Falte) | `twine` |
| Metall (Werkzeug) | `heather` | – | `dim-gray` |
| Nadeln (Tanne) | `elf-green` | `christi` | `opal` |
| Laub (Pappel, Birke) | `christi` | `atlantis` | `dell` |
| Birkenrinde | `white` | – | `heather`, Striche `black` (fein) |
| Putz (Fachwerk) | `pancho` | – | `twine` |
| Dachziegel | `brown` | `mandy` (linke Kante) | `loulou` (Traufe, Schindel-Fuge fein) |
| Fensterlicht | `tahiti-gold` | `golden-fizz` (oben links) | Sprossen `oiled-cedar` (fein) |
| Stein (Kamin, Esse, Sockel, Stufe) | `dim-gray` | `smokey-ash` | Fuge `valhalla` (fein) |
| Glut | `tahiti-gold` | `golden-fizz` (Kern, Funken fein) | `brown`, Öffnung `black` |
| Schiefer (Dach) | `deep-koamaru` | `venice-blue` (linke Kante) | `valhalla` |
| Eisen (Rüstung, Band) | `heather` | `light-steel` | `dim-gray` |
| Wappen | `brown` | `mandy` | Rand `tahiti-gold`, Zeichen `golden-fizz` (fein) |
| Kräuter | `christi` | `atlantis` | `elf-green` |
| Wasser (Schale, Trog) | `cornflower` | – | – |
| Stoff blau (Decke) | `cornflower` | `viking` | `royal-blue` (Falte) |

Zeichen am Gebäude sind Motive, keine Schutzzeichen: Das Heil-Zeichen ist ein **grünes** Kreuz (`christi`, Licht `atlantis`).

## 4. Bausteine

Ein Gebäude ist eine Liste von Rechtecken in Zeichenreihenfolge (hinten zuerst). Arten der Werkstatt: `wand`, `fuge`,
`schatten`, `schild`, `zeichen`, `pfosten`, `balken`, `tuch`, `volant`, `falte`, `tresen`, `kiste`, `gestell`,
`werkzeug`; Taverne: `kamin`, `dach`, `schindel`, `fenster`, `glas`, `sprosse`, `tuer`, `beschlag`, `stufe`, `ausleger`,
`kette`, `krug`, `fass`, `reifen`; Heilplatz: `kraut`, `schnur`, `liege`, `kissen`, `decke`, `schale`, `wasser`, `korb`, `flechtung`; Schmiede: `haken`, `esse`, `oeffnung`, `glut`, `funke`, `klotz`, `amboss`, `hammer`, `trog`;
Rüstkammer: `sockel`, `scharte`, `tor`, `band`, `niete`, `staender`, `helm`, `ruestung`, `wappen`, `speer`, `regal`; Bäume: `stamm`, `krone`, `licht`, `strich`, `blatt`. Die Art dient der Lesbarkeit und späteren Zusätzen; gezeichnet wird nur Rechteck und Farbe.

## 5. Datenformat

```json
{
  "id": "werkstatt",
  "vorlage": {
    "motiv": "Offener Handelsstand der Werkstatt mit gestreiftem Tuchdach und Werkzeuggestell rechts",
    "teile": ["Holzwand mit senkrechten Planken …", "Tuchdach aus 8 Bahnen rot und hell …", "Gestell rechts mit Hämmern und Bögen"],
    "materialien": ["Holz", "Stoff rot", "Stoff hell", "Metall"],
    "stimmung": "Markttag, warm und handwerklich",
    "fein": "Plankenfugen, Faltenlicht, Werkzeug"
  },
  "kacheln": { "b": 6, "h": 4 },
  "bausteine": [
    { "art": "pfosten", "rechteck": [0.0625, 0.625, 0.25, 3.375], "farbe": "oiled-cedar" },
    { "art": "fuge", "rechteck": [0.75, 1.4375, 0.03125, 2.1875], "farbe": "oiled-cedar", "fein": true }
  ]
}
```

- **`vorlage` (Pflicht, GEB8):** die Gebäude-Vorlage (Glossar). Felder `motiv` (ein Satz, Pflicht), `teile` (Liste,
  Pflicht; von links nach rechts, hinten nach vorn, Lage grob in Kacheln), `materialien` (Namen aus § 3), `stimmung`,
  `fein` (was bei 16 px entfällt). Sie beschreibt, was das PNG zeigt, und ist die Grundlage, um das Gebäude später
  annähernd gleich neu zu erzeugen. Fehlt sie, ist `motiv` oder `teile` leer oder steht ein unbekanntes Feld in der
  Datei, bricht `task gebaeude` ab. `index.json` übernimmt sie.
- `rechteck` = `[x, y, b, h]` in Kacheln, Ursprung oben links. Dateiname = `id` + `.json`.
- `klasse` (optional) gruppiert in der Vorschau; `baum` = Teil der Baum-Reihe.
- `fein: true` markiert Details, die bei gröberer Dichte entfallen (Werkzeuge, Faltenlicht, Plankenfugen).

### Bäume

Schmal, 1 Kachel breit, 2,25 Kacheln hoch, Stamm mittig (x 14–18 px Master). Sie stehen als kleiner Wald in Reihen mit
**½ Kachel Abstand**, überlappend, von links nach rechts gezeichnet (Vorschau: `baumReihe`). Varianten je Art und
Level-Paletten folgen später (B-435 › Beispiele).

## 6. Ausbaustufen

Stufen und Level-Looks entstehen aus derselben Quelle: über **Zusätze** (weitere Bausteine, z. B. Schild, zweites
Gestell) und **Palettentausch** (Farbnamen je Stufe ersetzen), nicht durch Neuzeichnen. Die Umsetzung (eine Datei je
Stufe oder Zusatz-Liste) legt der erste GEB-Sprint mit mehreren Stufen fest und trägt sie hier ein.

## 7. Dichte-Regel

- Die Quelle speichert Formen in Kachel-Einheiten; die Pixeldichte ist ein Render-Parameter.
- **Master: 32 px je Kachel.** Erlaubt sind nur **16, 32, 64, 128** px je Kachel; andere Werte → Fehler.
- **64 und 128** sind verlustfreie Vergrößerungen (Test: 64 nach 2×2-Downsampling = 32).
- **16** lässt `fein`-Bausteine weg und rundet ab. Ein Baustein ohne `fein`, der bei 16 dünner als 1 px wäre (also 1 px im Master), ist ein Fehler.
- Jede Dichte nimmt 🧑 in der Vorschau ab, bevor das Spiel sie nutzt. **Spiel-Dichte: 32 px je Kachel ×1** (Entscheidung 🧑 2026-10-16, GEB1.4; abgenommen in 32 und 16 px). Gebäude sind damit feiner als Figuren und die heutigen 16-px-Packs.

## 8. Skalierung

- Ganzzahlige Skalierung gilt für **Figuren, Gebäude und Objekte gemeinsam** (ein Faktor, ein Pixelraster; Workshop GEB1.1).
- **Faktor: offen**, kommt aus GR7.4 (B-251, Beobachtung am TV). Beobachtet wird in 4K mit Kamera-Zoom 1.
- Ungeprüft: Edge auf der Xbox rendert vermutet mit 1080p-Viewport und `devicePixelRatio` 2.

## 9. Ablauf

1. Zuerst die Vorlage schreiben (§ 5), dann die Bausteine danach zeichnen; ändert sich das Bild, die Vorlage mitziehen.
   Daten-JSON in `public/grafik/gebaeude/` anlegen oder ändern.
2. `task gebaeude` rendert `<id>-16.png`, `-32.png`, `-64.png`, `-128.png` und `index.json`; gleiche Daten → byte-gleiche PNGs.
3. Vorschau: `assetvorschau.html?szene=gebaeude`, Streifen „Neue Gebäude (GEB)“ (Baum-Reihe, dann je Gebäude 32 px ×1 neben 16 px ×2).
4. PNGs und `index.json` mit committen.
