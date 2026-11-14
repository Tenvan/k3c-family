# Rig-Format des nativen Spiels (Prototyp NV2, B-778; alle Figuren NV3.2, B-789)

Eine Figur ist ein Ordner unter `native/k3c-rig/assets/<gruppe>/<id>/` mit `rig.json` und einem PNG je Teil; `<gruppe>`
ist der Ordner der Spec unter `public/grafik` (dieselbe id kommt in zwei Gruppen vor: Gegner `wolf`, Reittier `wolf`).
Der Export (`task native:rig:export`, `native/k3c-rig/export/`) schreibt alle Specs mit Rig-Typ `figur` oder
`vierbeiner` (`export/catalog.ts`); `requisite`, `gebaeude` und `hintergrund` haben keinen Teile-Baum und bleiben Sheets.
Das Go-Paket `native/k3c-rig/rig` liest ihn und rechnet die Pose je Bild. Maße in **Einheiten der Figur** (y nach unten,
Füße bei y = 0), Winkel in Grad, positiv im Uhrzeigersinn (wie SVG).

**Was je Figur vorkommt:** Alle Figuren haben `teile`, `animationen` und `varianten` (leer ohne Farbvarianten). Nur der
Monarch (`figuren/monarch`, der Reiter) hat Plätze, Elemente, `standard`, `reiten` und Farbvarianten; nur Figuren mit
`umhang` in der Spec haben das Band `umhang`. Bei allen anderen Figuren bleibt die Waffe ein festes Teil, `plaetze` ist
leer. Reittiere (Gruppe `reittiere`) haben `sattel`, `maul`, `zuegel`, die Animation `stand` und die Varianten des
Reiters; Vierbeiner anderer Gruppen (Gegner) nur Teile, Bein-Bänder und ihre Animationen. Teile ohne Lagen (Bein-Knochen,
leere Schild-, Helm- oder Waffen-Teile mancher Bosse) haben kein Bild, drehen aber mit.

## rig.json

| Feld | Bedeutung |
|---|---|
| `id`, `rig`, `version` | wie in der Papier-Spec (`figur` oder `vierbeiner`) |
| `dichte` | Pixel der Teil-Bilder je Einheit der Figur (doppelte Dichte des Sheets, linear gefiltert) |
| `massstab` | Welt-Einheiten je Einheit der Figur (Pferd 1,3); Pivots und Versätze stehen in Einheiten der Figur |
| `rahmen` | Rahmen des Sheets in Welt-Einheiten (für den Bildvergleich; beim Vierbeiner der Reit-Rahmen) |
| `teile` | starre Teile in Reihenfolge der Spec: `id`, `eltern`, `pivot`, `ebene`, dazu `bild`, `versatz`, `groesse` |
| `plaetze` | benannte Plätze: `name`, `teil`, `punkt`, `drehung`, `ebene` |
| `baender` | gebogene Teile: `id`, `teil`, `ebene`, `mitte`, `ruhe`, `breite`, dazu `bild`, `versatz`, `groesse` |
| `umhang` | Physik des Umhangs wie in der Spec (`anker`, `punkt`, `laenge`, `breite`, `segmente`) |
| `bein` | Vierbeiner: `spitze` = Verlängerung des Beins über die Fessel |
| `proportionen` | wie in der Spec (Faktoren auf die Pose-Winkel) |
| `animationen` | wie in der Spec. figur: Animationen ohne Reittier (`idle`, `lauf` sind Posen des Rigs, Keyframes addieren sich; andere brauchen Keyframes, `sichtbar` blendet Teile aus, `fps` bestimmt die Frequenz); vierbeiner: Animationen ohne Ebene, je mit `gangart` (Gegner: `idle` = schritt, `lauf` = trab, `angriff` = galopp), Reittiere dazu `stand` |
| `varianten` | Variante → Bild-Id (Teil oder Band) → Datei statt des Standardbilds (Spielerfarben); `{}` ohne Varianten |
| `standard` | nur Monarch: Elemente, die ohne Auswahl an den Plätzen sitzen (`waffe: schwert`) |
| `reiten` | nur figur mit Reit-Animationen: `sitz` (Punkt, der auf dem Platz `sattel` liegt) und `ausblenden` (Teile, die beim Reiten entfallen) |
| `zuegel` | nur Reittiere: Farbe, Deckkraft, Breite, Durchhang und Ebene der Zügel |

**Teil:** dreht um `pivot`, Kinder erben die Drehung (`eltern`). `bild` ist das Teil in Ruhelage (alle Winkel 0) mit
Schatten und Papierstruktur; `versatz` ist die linke obere Ecke des Bilds in Einheiten, `groesse` seine Pixel. Teile ohne
Lagen (Bein-Knochen des Vierbeiners) haben kein Bild.

**Ebene:** Zeichenreihenfolge, kleiner = weiter hinten. Teile tragen Index · 10 aus der Spec, Plätze und Bänder liegen
dazwischen. Der Umhang hat −10 (hinter allen Teilen).

**Platz:** hängt an einem Teil; ein Element oder eine andere Figur wird mit `Welt-Matrix(teil) · translate(punkt) ·
rotate(drehung)` gezeichnet. Ohne Element bleibt der Platz leer. Plätze des Monarchen: `waffe`, `ruestung`,
`kopfbedeckung`, `hand` (Zügel); des Pferds: `sattel` (Reiter), `maul` (Zügel).

**Band:** ein Bild der Ruhelage (gerade nach unten), das entlang von Stützpunkten gebogen wird. `mitte` ist das x der
Mittellinie, `ruhe` das y je Stützpunkt, `breite` der Abstand der linken und rechten Kante von der Mittellinie je
Stützpunkt (Papierkante plus Rand für den Schatten). `teil` nennt den Rahmen, in dem die Stützpunkte liegen (Bein: das
Rumpf-Teil; Umhang: Welt). Die Laufzeit legt ein Dreiecksband durch die Stützpunkte (Catmull-Rom, wie die Umrisse in
`tools/paper`).

## Elemente

`assets/elemente/elemente.json` nennt optionale Elemente: `id`, `platz`, `dichte`, `bild`, `groesse` und `versatz`
(linke obere Ecke **ab dem Punkt des Platzes**). Das Schwert des Monarchen ist ein Element am Platz `waffe`, kein festes
Teil. `axt` und `brustpanzer` sind Platzhalter nur für den Prototyp (`export/elements.ts`, keine Spec in `public/grafik`).

## Reiten

Der Reiter sitzt mit `reiten.sitz` auf dem Platz `sattel` des Reittiers:
`Welt-Matrix(sattel) · scale(1 / massstab) · translate(0, Hub) · rotate(Neigung) · translate(−sitz)`. Hub, Neigung und
die Winkel des Reiters liefert die Gangart (`rig/gait.go`). Gezeichnet wird in der Ebenen-Reihenfolge des Reittiers: An
der Ebene des Platzes `sattel` kommt der Reiter (erst sein Umhang), danach der Hals und die Zügel.

## Sollwerte

`assets/soll/<gruppe>/<id>/<animation>.json` hält je Animation jeder Figur und je Frame die Winkel, Teil-Matrizen und
Stützpunkte, gerechnet von `tools/paper` (`export/expected.ts`): figur mit Versatz, Wind, ausgeblendeten Teilen und
Umhang; vierbeiner mit Ride (Hub, Neigung), Wind und Beinen. Reittiere haben dazu `reiten-<gangart>.json` je
Reit-Animation des Monarchen (Matrizen beider Figuren, Umhang, Zügel). `go test ./rig/` geht über alle Ordner mit
`rig.json` und vergleicht die Go-Rechnung damit (Toleranz 0,01, B-778/AC-02, B-789/AC-03). `TestLegStripNeverFolds`
prüft die Bein-Bänder aller Vierbeiner in jeder Gangart ohne Ausnahme (`rig/band_test.go` › `knownFolds` ist seit
B-791 leer): `tangentLimits` begrenzt die Tangente der Kurve am Band-Ende (Elefant: langes Unterbein, kurze
Fußspitze), `unfold` hält die innere Kante an engen Beugungen an (Gegner-Wölfe im Galopp), statt sie rückwärts laufen
zu lassen: Die Ecke kommt per Lot auf die Linie davor, das Dreieck wird zur Linie und die Kante bewegt sich stetig
(`rig/band.go`, geprüft von `TestUnfoldIsContinuous` und `TestLegStripContinuous`).

## Entscheidungen des Prototyps (B-778 › Offene Fragen)

- **Schatten:** in das Teil-Bild gebacken; er dreht sich mit dem Teil.
- **Schärfe:** Teile in doppelter Dichte, linear gefiltert.
- **Beine des Pferds und Umhang:** Dreiecksband mit einem Bild der Ruhelage.
- **Farbvarianten:** eigene PNGs nur für die Teile, die sich unterscheiden.
- **Papierstruktur:** hängt am Teil (im Sheet am Bild); das Pferd trägt die Struktur des Reiters (Seed `monarch`), weil
  das Reit-Sheet so gerendert ist.
