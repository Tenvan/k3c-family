# Gebäude-Grafik: Grundregeln

Gilt für alle Gebäude und Strukturen, die mit `tools/buildings` entstehen (Projekt GEB, B-435). Belegt an der
Werkstatt (`public/grafik/gebaeude/werkstatt.json`, Sprint GEB1). Die Regeln wachsen mit jedem Gebäude; Änderungen
nur mit Zustimmung von 🧑.

## 1. Raster und Maße

- **Kachel** = `UNIT_PX` = 32 px in der Master-Dichte. Alle Maße in Kacheln, Vorderansicht, Boden = Unterkante des Bilds.
- Koordinaten und Maße sind Vielfache von **1/32 Kachel** (1 px Master), auch Bruch-Maße wie 2,25. Andere Werte lehnt das Werkzeug ab.

| Klasse | Breite × Höhe (Kacheln) | Beleg |
|---|---|---|
| Handelsstand / kleines Gebäude | 6 × 4 | Werkstatt (`workshop`), Taverne (`tavern`), Heilplatz (`healer`), Schmiede (`smithy`), Rüstkammer (`armory`) (GEB2); Lager (`storage`), Kaserne (`barracks`) in Stein (GEB5) |
| Baum (schmal) | 1 × 2,25 | Tanne, Pappel, Birke (GEB1.3); je Art Form 2 und 3 (`tanne-2` … `birke-3`, Looks Wald und Höhle, GEB6.1) |
| Busch | 1 × 1 | `busch.json` (GEB6.1) |
| Truhe (offen, mit Gold) | 1 × 1 | `truhe.json` (GEB6.4) |
| Portal, Ausgang (statisch, ohne Stufen) | 2 × 3 | `portal.json`, `ausgang.json` (GEB6.4); Portal mit leuchtender Kristallfläche, Ausgang mit dunkler Öffnung und Holzrahmen |
| Rekrutierungslager (Zelt und Feuerstelle, statisch) | 3 × 2 | `rekrutierungslager.json` (GEB6.4) |
| Ader (Stufen 1–4 = Stein, Kupfer, Eisen, Kristall) | 2 × 2 | `ader.json` (GEB6.3); Felsbuckel mit Erzgang, Stein-Ader in `pancho`/`white`, die nur im Gang vorkommt |
| Fels (Looks Wald, Höhle, Mine), Erzbrocken (Look Mine) | 1 × 1 | `felsen.json`, `erzbrocken.json` (GEB6.2); Licht auf Stein ist `topaz` (heller als `dim-gray`), Risse in Höhle und Mine `black` |
| Mauer (Stufen 1–5) | 1 × 5 | `mauer.json` (GEB3) |
| Turm (Stufen 1–5) | 3 × 8 | `turm.json` (GEB3) |
| Hintergrundmauer (Stufen 1–5, kachelbar) | 1 × 4 | `hintergrundmauer.json` (GEB3, B-442) |
| Hintergrund-Schicht (je Biom fern, mitte, nah; kachelbar, Klasse `hintergrund`) | 24 × 15 (768 × 480 px) | `hg-<biom>-<schicht>.json` (GEB7, B-435), siehe § 10 |
| Burg (Hub-Stufen 1–5) | 8 × 9 | `burg.json` (GEB4) |
| Tor (Stufen 1–5, Stufe der Mauerlinie) | 2 × 4 | `tor.json` (GEB4) |
| Farm (Scheune, Holz) | 4 × 3 | `farm.json` (`farm`, GEB5) |
| Setzling (leerer Plantage-Platz; gewachsen: Pappel) | 1 × 1 | `setzling.json` (`plantation`, GEB5) |
| Treppe hoch (ein Bild, Stein) | 4 × 4 | `treppe-hoch.json` (`stairsUp`, GEB5) |
| Treppe runter (Steinbogen mit Abgang) | 4 × 3 | `treppe-runter.json` (`stairsDown`, GEB5) |
| weitere Gebäude | legt der jeweilige GEB-Sprint fest, hier nachtragen | B-435 › Notizen |

## 2. Palette und Licht

- Nur **DB32** (DawnBringer 32, Namen nach Lospec, Tabelle in `tools/buildings/db32.go`); dazu Transparenz. Kein Anti-Aliasing, keine Zwischenfarben.
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
| Kupfer (Mauer-, Turmstufe 3) | `tahiti-gold` | `twine` | `rope`, Fuge `oiled-cedar`, Patina `verdigris` |
| Eisen hell (Mauer-, Turmstufe 4) | `heather` | `light-steel` | `dim-gray`, Fuge `valhalla` |
| Kristall (Mauer-, Turmstufe 5) | `royal-blue` | `cornflower`, Glanz `white` | `deep-koamaru`, leuchtende Fuge `viking` |
| Stoff blau (Decke) | `cornflower` | `viking` | `royal-blue` (Falte) |
| Hintergrund Wald | `opal` | `verdigris` (Krone), `dell` (fein) | `valhalla` |
| Hintergrund Holz | `oiled-cedar` | `rope` (Kante, schmal) | `loulou`, `valhalla` |
| Hintergrund Fels | `valhalla` | `opal`, Kante `smokey-ash` (schmal) | `black`; kühl `deep-koamaru`, warm `loulou` |
| Hintergrund Eisen | `smokey-ash` | `dim-gray` (Kante, fein) | `valhalla` |
| Hintergrund Glut | `brown` | `tahiti-gold` (Kern, fein, sparsam) | `oiled-cedar`, `loulou` |
| Hintergrund Kristall | `deep-koamaru` | `royal-blue` (fein), Glanz `viking` (fein, sparsam) | `valhalla` |
| Fels (Felsen, Erzbrocken) = Stein | `dim-gray` | `smokey-ash` | Fuge `valhalla` (fein) |
| Zelttuch (Lager) = Stoff hell | `pancho` | `white` (Falte) | `twine` |
| Erde (Wurzeln, Hügelrand) = Pfosten, Balken | `oiled-cedar` | `rope` | – |
| Gold (Truhe) = Glut | `tahiti-gold` | `golden-fizz` | `brown` |

Die vier letzten Zeilen (GEB6.1) sind Verweise auf vorhandene Materialien, neu ist nur der Name; 🧑 hat sie als Antwort auf die Offene Frage 4 aus GEB6 bestätigt (Option 1, 2026-10-16, Chat). Auf Stein dient `topaz` als Licht, weil `smokey-ash` dunkler als `dim-gray` ist und als Schatten wirkt.

Zeichen am Gebäude sind Motive, keine Schutzzeichen: Das Heil-Zeichen ist ein **grünes** Kreuz (`christi`, Licht `atlantis`).

## 4. Bausteine

Ein Gebäude ist eine Liste von Rechtecken in Zeichenreihenfolge (hinten zuerst). Arten der Werkstatt: `wand`, `fuge`,
`schatten`, `schild`, `zeichen`, `pfosten`, `balken`, `tuch`, `volant`, `falte`, `tresen`, `kiste`, `gestell`,
`werkzeug`; Taverne: `kamin`, `dach`, `schindel`, `fenster`, `glas`, `sprosse`, `tuer`, `beschlag`, `stufe`, `ausleger`,
`kette`, `krug`, `fass`, `reifen`; Heilplatz: `kraut`, `schnur`, `liege`, `kissen`, `decke`, `schale`, `wasser`, `korb`, `flechtung`; Mauer und Turm: `zinne`, `wand`, `bruestung`, `konsole`, `schaft`, `scharte`, `tuerrahmen`, `tuer`, `sockel`,
`patina`, `band`, `niete`, `glanz`, `kristall`, `aura`, `runenband`, `rune`, `kante` (Hintergrundmauer); Burg: `turm`, `obergeschoss`, `mast`, `banner`; Tor: `sturz`, `fluegel`, `riegel` (Flügel in `stinger` bleiben Holz, nicht im Farbtausch); Schmiede: `haken`, `esse`, `oeffnung`, `glut`, `funke`, `klotz`, `amboss`, `hammer`, `trog`;
Rüstkammer: `sockel`, `scharte`, `tor`, `band`, `niete`, `staender`, `helm`, `ruestung`, `wappen`, `speer`, `regal`; Lager: `sack`, `holz`, `erz`; Kaserne: `schwert`; Farm: `luke`, `strebe`, `zaun`, `heu`; Treppe hoch: `stufe`, `gelaender`, `laterne`; Treppe runter: `bogen`, `schlussstein`, `oeffnung`; Bäume: `stamm`, `krone`, `licht`, `strich`, `blatt`; Hintergrund Wald: `boden`, `huegel`, `schatten`, `farn`, `fuge`, `laub`, `gras`, `busch`, `buschlicht`; Hintergrund Höhle: `wand`, `wandkopf`, `pfeiler`, `sturz`, `stufe`, `schimmer`, `decke`, `deckenkante`, `zapfen`, `zapfenschatten`, `tropfen`, `brocken`, `brockenschatten`, `bodenkante`, `bogen`, `bogenschatten`, `stalagmit`; Hintergrund Mine: `fels`, `spalte`, `stuetze`, `kante`, `quer`, `kopfband`, `querkante`, `lampe`, `schutt`, `schotter`, `schwelle`, `schiene`, `schienenkante`; Hintergrund Eisenstollen: `bogen`, `licht`, `traeger`, `strebe`, `niete`, `pfeiler`, `platte`, `spalte`, `kern`; Hintergrund Kristall: `sockel`, `turm`, `rippe`, `kantenlicht`, `bodenlicht`, `gruppe`, `mittel`, `splitter`, `glanz`, `kristall`, `flanke`, `zapfen`, `zflanke`, `zkante`, `zlicht`, `zrand`, `zglanz`. Die Art dient der Lesbarkeit und späteren Zusätzen; gezeichnet wird nur Rechteck und Farbe.

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
  Datei, bricht `task gen:gebaeude` ab. `index.json` übernimmt sie.
- `rechteck` = `[x, y, b, h]` in Kacheln, Ursprung oben links. Dateiname = `id` + `.json`.
- `klasse` (optional) gruppiert in der Vorschau; `baum` = Teil der Baum-Reihe; `kachel` = in der Breite nahtlos kachelbar
  (Vorschau zeigt drei Kacheln nebeneinander). Kachelbar heißt: keine Seitenkanten, Muster periodisch über die Breite,
  erste und letzte Pixelspalte gleich (Go-Test `TestHintergrundmauerKachelbar`).
- `klasse: hintergrund` = Hintergrund-Schicht (§ 10): wie `kachel` in der Breite nahtlos, nur Dichten 16 und 32, Oberkante transparent,
  unterste Pixelzeile deckend. Die Vorschau zeigt sie im Streifen „Hintergründe“ (§ 9), nicht als Gebäude-Karte.
- `wiederholung: { "schritt": <Kacheln>, "anzahl": <n ≥ 2> }` am Baustein (GEB7): `anzahl` Kopien entlang x im Abstand `schritt`
  (die erste ist das Original, Kopien folgen direkt in der Zeichenreihenfolge). Schritt ein positives Vielfaches von 1/32, jede Kopie
  im Bild; sonst bricht `task gen:gebaeude` mit Gebäude, Baustein und Feld ab. Gilt auch für `zusaetze`.
- `fein: true` markiert Details, die bei gröberer Dichte entfallen (Werkzeuge, Faltenlicht, Plankenfugen).

### Bäume

Schmal, 1 Kachel breit, 2,25 Kacheln hoch, Stamm mittig (x 14–18 px Master). Sie stehen als kleiner Wald in Reihen mit
**½ Kachel Abstand**, überlappend, von links nach rechts gezeichnet (Vorschau: `baumReihe`). Je Art gibt es **drei
Formen** (GEB6.1): Form 1 sind `tanne`, `pappel`, `birke` (unverändert, nur Wald-Look, Dateinamen stabil), Form 2 und 3
sind eigene JSONs mit unterscheidbarem Umriss (`tanne-2` breit mit tiefen Ästen, `tanne-3` schlank mit schiefer Spitze,
`pappel-2` schmal mit Lücken, `pappel-3` mit Zwiegabel, `birke-2` mit hängenden Zweigen, `birke-3` niedriger und breiter).
Der **Busch** (`busch`) ist 1 × 1, nur im Wald.

## 6. Ausbaustufen

Stufen und Level-Looks entstehen aus derselben Quelle: über **Zusätze** (weitere Bausteine, z. B. Schild, zweites
Gestell) und **Palettentausch** (Farbnamen je Stufe ersetzen), nicht durch Neuzeichnen.

**Umsetzung (GEB3, Entscheidung 🧑 2026-10-16):** Ein Gebäude mit Ausbaustufen ist **eine** JSON. Bausteine und
Vorlage beschreiben die **Grundform = Stufe 1**. Das Feld `stufen` listet jede weitere Stufe und beschreibt genau, was
sich gegenüber der Grundform ändert:

```json
"stufen": [
  { "stufe": 2, "aenderung": "Steinquader statt Pfählen, Zinnen oben",
    "farben": { "oiled-cedar": "dim-gray", "rope": "smokey-ash" },
    "zusaetze": [ { "art": "zinne", "rechteck": [0, 0, 0.25, 0.25], "farbe": "smokey-ash" } ] }
]
```

- `stufe` 2, 3, … lückenlos und in Reihenfolge; `aenderung` (Pflicht) ist der Text zur Stufe und gehört zur Vorlage.
- `farben` tauscht Farben der Grundform-Bausteine (alt → neu, beide DB32); `zusaetze` werden danach gezeichnet und
  gelten wie Bausteine (Raster, Maße, `fein`). Zusätze einer Stufe gelten nur für diese Stufe, nicht für spätere.
- Ausgabe `<id>-<stufe>-<dichte>.png` für alle Stufen einschließlich 1; `index.json` führt `stufen` mit Nummer,
  Änderung (Stufe 1: „Grundform“) und Dateien, `dateien` des Eintrags zeigt die Grundform.
- Gebäude ohne `stufen` bleiben bei `<id>-<dichte>.png`.

**Biom-Look bei Natur-Objekten (GEB6, Annahme, 🧑 hat mit der Freigabe zugestimmt):** Bei Objekten der Welt (Bäume,
Busch, Fels, Erzbrocken) bedeutet die **Stufe den Look**, nicht den Ausbau: Stufe 1 = Wald (Grundform), 2 = Höhle,
3 = Mine; `aenderung` nennt den Look („Höhle: kühleres, dunkleres Laub“). Ein Look ist ein Farbtausch derselben
Quelle, keine zweite Zeichnung. Ein Objekt hat nur die Looks der Biome, in denen es steht: Bäume Wald und Höhle (Mine
zeigt den Look Höhle), Busch nur Wald, Fels Wald, Höhle und Mine; Biome ohne eigenen Look (Eisenstollen,
Kristallhöhle) zeigen Mine, begrenzt auf den höchsten Look des Objekts. Der Höhle-Look der Bäume: Laub eine Stufe
dunkler und kühler (`christi` → `elf-green`, `atlantis` → `christi`, `dell` → `opal`), Nadeln `elf-green` → `opal`,
`christi` → `elf-green`, `opal` → `deep-koamaru`, Stämme eine Stufe dunkler. Bei **Adern** (`ader`) bedeutet die Stufe
das **Material** (1 Stein, 2 Kupfer, 3 Eisen, 4 Kristall), nicht das Biom.

**Hub-Gebäude (GEB9, Entscheidung autonom Claude 2026-10-22, Annahme bis 🧑 widerspricht):** Die acht Hub-Gebäude (Werkstatt,
Farm, Kaserne, Lager, Taverne, Schmiede, Heilplatz, Rüstkammer) haben die Stufen 1–5 und folgen der **Hub-Stufe**, nicht eigenen
Gebäudestufen. Das Material je Stufe ist das der Hub-Stufe (`docs/rules/materialien-gebaeude.md` § 2). Stufen vor der
Freischaltung des Gebäudes bleiben erhalten (Vorschau, Erweiterbarkeit, der Gebäude-Planer verschiebt die Freischaltung). Das
Camp (`rekrutierungslager`) ist ein Welt-Objekt und bleibt Grundform.

*Farbrollen.* Der Tausch folgt der Rolle der Farbe, nicht dem Namen: jedes Material hat Körper, Licht, Schatten und Tief.

| Stufe | Material | Körper | Licht | Schatten | Tief |
|---|---|---|---|---|---|
| 1 | Holz | `rope` | `twine` | `oiled-cedar` | `loulou` |
| 2 | Stein | `dim-gray` | `smokey-ash` | `valhalla` | `black` |
| 3 | Kupfer | `tahiti-gold` | `twine` | `rope` | `oiled-cedar` |
| 4 | Eisen | `heather` | `light-steel` | `dim-gray` | `valhalla` |
| 5 | Kristall | `royal-blue` | `cornflower` | `deep-koamaru` | `viking` |

Eine Farbe der Grundform wird auf die Farbe derselben Rolle im Material der Stufe getauscht (Holz-Grundform: Spalte der Stufe 1,
Stein-Grundform: Spalte der Stufe 2; Dachfarben `brown` = Körper, `mandy` = Licht, `loulou` = Tief). Das entspricht dem Tausch
von Mauer, Turm und Burg. Feinabstimmung nach Sicht ist erlaubt, wenn ein Detail verschwindet (z. B. ein Eisenband, das auf Stufe 4 dieselbe
Farbe wie der Körper bekäme); die Abweichung steht im Ergebnis der Session.

*Teile statt ganzer Farbe.* Damit nur Mauerwerk und Dach das Material wechseln, kann ein Schlüssel in `farben` auf eine Bausteinart
eingeschränkt werden: `"wand/rope": "dim-gray"` tauscht `rope` nur bei Bausteinen der Art `wand`; ein Schlüssel ohne `/` gilt
für alle Bausteine. Der engere Schlüssel gilt vor dem weiteren; die alte Farbe muss in der Grundform bei dieser Art vorkommen.

*Was tauscht, was bleibt, Zusatz.* Handwerkliche und bewegliche Teile (Gestell, Tresen, Zaun, Tür, Fass, Krug, Werkzeug, Glut,
Heu, Tuch, Rüstung) bleiben auf allen Stufen unverändert. Je Stufe höchstens **ein** Zusatz; er trägt die Farben des Materials.

Die Wandzeichnung `fuge` und `schatten` folgt immer dem Mauerwerk, zu dem sie gehört (gleiche Rolle, gleiche Wand), und steht deshalb
bei jedem Gebäude mit in der Liste, wenn dort eine tauschende Art (`wand`, `dach`) vorkommt.

| Gebäude | Arten, die das Material wechseln | Zusatz |
|---|---|---|
| Werkstatt | `wand`, `pfosten`, `balken` | ab Stufe 3 Wimpel am rechten Pfosten |
| Farm | `wand`, `pfosten`, `balken`, `dach`, `schindel` | ab Stufe 3 Windfahne rechts am First (Tor mit Rahmen und Streben bleibt) |
| Kaserne (Stein) | `wand`, `sockel`, `zinne`, `licht`, `schatten`, `fuge`, `scharte` | Stufe 2 Eisenband am Sockel (links und rechts vom Tor), ab Stufe 3 Wimpel an den Zinnen |
| Lager | `wand` (dunkle Innenwand = Rolle Schatten), `pfosten`, `dach`, `schindel`; ab Stufe 3 auch `sockel` | ab Stufe 3 Vorratsschild an der Wand (das Lager hat kein Tor) |
| Taverne | `wand` (Putz = Rolle Licht), `balken`, `pfosten`, `dach`, `schindel`; ab Stufe 3 auch `kamin`, `stufe` | ab Stufe 3 Laterne links neben der Tür |
| Schmiede | `wand`, `pfosten`, `dach`, `haken`; ab Stufe 3 auch `kamin`, `esse` | ab Stufe 3 Rauchfang (Haube) über der Esse |
| Heilplatz | `pfosten`, `gestell` (das Zelttuch bleibt) | ab Stufe 3 Laterne rechts am Zelt |
| Rüstkammer | `wand`, `dach` (Rollen Körper, Licht für `deep-koamaru`, `venice-blue`, Tief für `valhalla`), `schindel`; ab Stufe 3 auch `sockel` | ab Stufe 3 Wimpel auf dem Dach (Tor mit Beschlägen bleibt) |

*Stein-Grundform.* Die Kaserne ist in Stufe 1 schon Stein: Stufe 2 behält das Material und unterscheidet sich nur durch
den Zusatz (Stufe 2 der Tabelle); ab Stufe 3 wechselt der Stein auf Kupfer, Eisen, Kristall. Bei Lager, Taverne, Schmiede und Rüstkammer
(Holz mit Steinteilen) wechselt auf Stufe 2 nur das Holz-Mauerwerk, der Stein von Sockel, Kamin und Esse bleibt, ab Stufe 3 wechselt auch er.

*Übergang bis GEB9.5.* Mit `stufen` schreibt `tools/buildings` sonst nur `<id>-<stufe>-<dichte>.png`. Solange das Spiel noch die Grundform
zeichnet, schreibt der Generator für die Hub-Gebäude zusätzlich `<id>-<dichte>.png` (= Stufe 1, byte-gleich). GEB9.5 stellt das Spiel
um, löscht diese Dateien und entfernt den Übergang.

## 7. Dichte-Regel

- Die Quelle speichert Formen in Kachel-Einheiten; die Pixeldichte ist ein Render-Parameter.
- **Master: 32 px je Kachel.** Erlaubt sind nur **16, 32, 64, 128** px je Kachel; andere Werte → Fehler.
- **64 und 128** sind verlustfreie Vergrößerungen (Test: 64 nach 2×2-Downsampling = 32). Die Klasse `hintergrund` rendert nur **16 und 32** (64/128 wären mehrere MB je Schicht ohne Spielwert).
- **16** lässt `fein`-Bausteine weg und rundet ab. Ein Baustein ohne `fein`, der bei 16 dünner als 1 px wäre (also 1 px im Master), ist ein Fehler.
- Jede Dichte nimmt 🧑 in der Vorschau ab, bevor das Spiel sie nutzt. **Spiel-Dichte: 32 px je Kachel ×1** (Entscheidung 🧑 2026-10-16, GEB1.4; abgenommen in 32 und 16 px). Gebäude sind damit feiner als Figuren und die heutigen 16-px-Packs.

## 8. Skalierung

- Ganzzahlige Skalierung gilt für **Figuren, Gebäude und Objekte gemeinsam** (ein Faktor, ein Pixelraster; Workshop GEB1.1).
- **Faktor: offen**, kommt aus GR7.4 (B-251, Beobachtung am TV). Beobachtet wird in 4K mit Kamera-Zoom 1.
- Ungeprüft: Edge auf der Xbox rendert vermutet mit 1080p-Viewport und `devicePixelRatio` 2.

## 9. Ablauf

1. Zuerst die Vorlage schreiben (§ 5), dann die Bausteine danach zeichnen; ändert sich das Bild, die Vorlage mitziehen.
   Daten-JSON in `public/grafik/gebaeude/` anlegen oder ändern.
2. `task gen:gebaeude` rendert `<id>-16.png`, `-32.png`, `-64.png`, `-128.png` und `index.json`; gleiche Daten → byte-gleiche PNGs.
3. Vorschau: Asset-Studio, Seite Grafik, Reiter Szenen, Szene Gebäude, Streifen „Neue Gebäude (GEB)“ (Baum-Reihe, dann je Gebäude 32 px ×1 neben 16 px ×2).
4. Hintergrund-Schichten: Streifen „Hintergründe“ (Szene Umgebung desselben Reiters) (je Biom die drei Schichten fern → nah auf `palette.sky`, zwei Kacheln nebeneinander, 32 px ×1 und 16 px ×2).
5. PNGs und `index.json` mit committen.

## 10. Hintergrund-Schichten

Je Biom drei Schichten (`hg-<biom>-fern|mitte|nah`, Biom: `wald`, `hoehle`, `mine`, `eisenstollen`, `kristall`; Klasse `hintergrund`, GEB7).

- **Maß:** 24 × 15 Kacheln (768 × 480 px bei 32 px, 384 × 240 px bei 16 px); Spiel: Maßstab 1, Unterkante auf der Bodenlinie. Der Himmel ist `palette.sky` des Bioms (`data/biomes/*.json`, nicht DB32) und scheint oben durch: **oberste Pixelzeile transparent**.
- **Bodenlinie:** unterste Pixelzeile **durchgehend deckend** (kein Spalt über dem Boden, die Fluss-Spiegelung zeigt sonst Streifen).
- **Naht:** erste = letzte Pixelspalte. Nichts berührt die Randspalten außer durchgehenden Bändern (Boden); Bausteine gehen nie über die Bildkante, Wiederholungen enden mit Abstand zum Rand.
- **Helligkeit (Luma 0,299 R + 0,587 G + 0,114 B, Mittel über die deckenden Pixel bei 32 px):** fern < mitte < nah, je Schicht mindestens **8** Punkte heller als die dahinter; „nah“ im Mittel höchstens **75**, also dunkler als das dunkelste Gebäude (Farm 79). Leuchtfarben (Glut, Kristalllicht) nur in kleinen Flächen. Prüfung: `TestHintergrundStufen` (`tools/buildings/hintergrund_test.go`).
- **Material:** die Zeilen „Hintergrund …“ in § 3 (Wald: Krone, Hügel, Farn; Holz: Stamm, Balken, Gerüst; Fels: Wand, Bogen, Zapfen; Eisen: Träger, Strebe; Glut: Lavaschein, Spalte; Kristall: Gruppe, Zapfen); fernere Schichten entsättigter und dunkler. Motive stehen in der Vorlage jeder JSON.
- **Scroll-Faktoren im Spiel** (hinten < vorn, Code in `worldSprites.ts`): Wald 0,15 / 0,35 / 0,5, Höhle 0,1 / 0,2 / 0,4, Mine, Eisenstollen und Kristallhöhle 0,1 / 0,25 / 0,45. Der Faktor am TV bleibt offen (§ 8).
- **Variation** entsteht nur in den Daten (Wiederholungen mit festen Schritten und Versätzen), nie durch Zufall.
