# Rig-Format des nativen Spiels (Prototyp NV2, B-778)

Eine Figur ist ein Ordner unter `native/k3c-rig/assets/<id>/` mit `rig.json` und einem PNG je Teil. Der Export
(`task native:rig:export`, `native/k3c-rig/export/`) schreibt ihn aus der Papier-Spec und der Stil-Datei; das Go-Paket
`native/k3c-rig/rig` liest ihn und rechnet die Pose je Bild. Das Format gilt für die Rig-Typen `figur` und `vierbeiner`
und ist für Gegner und Bosse gedacht (B-780). Maße in **Einheiten der Figur** (y nach unten, Füße bei y = 0), Winkel in
Grad, positiv im Uhrzeigersinn (wie SVG).

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
| `animationen` | figur: Animationen ohne Reittier (`idle`, `lauf` sind Posen des Rigs, die anderen Keyframes); vierbeiner: Gangarten |
| `varianten` | Variante → Bild-Id (Teil oder Band) → Datei statt des Standardbilds (Spielerfarben) |
| `standard` | Elemente, die ohne Auswahl an den Plätzen sitzen (`waffe: schwert`) |
| `reiten` | figur: `sitz` (Punkt, der auf dem Platz `sattel` liegt) und `ausblenden` (Teile, die beim Reiten entfallen) |
| `zuegel` | vierbeiner: Farbe, Deckkraft, Breite, Durchhang und Ebene der Zügel |

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

`assets/soll/*.json` hält je Frame die Winkel, Teil-Matrizen und Stützpunkte, gerechnet von `tools/paper`
(`export/expected.ts`). `go test ./rig/` vergleicht die Go-Rechnung damit (Toleranz 0,01, B-778/AC-02).

## Entscheidungen des Prototyps (B-778 › Offene Fragen)

- **Schatten:** in das Teil-Bild gebacken; er dreht sich mit dem Teil.
- **Schärfe:** Teile in doppelter Dichte, linear gefiltert.
- **Beine des Pferds und Umhang:** Dreiecksband mit einem Bild der Ruhelage.
- **Farbvarianten:** eigene PNGs nur für die Teile, die sich unterscheiden.
- **Papierstruktur:** hängt am Teil (im Sheet am Bild); das Pferd trägt die Struktur des Reiters (Seed `monarch`), weil
  das Reit-Sheet so gerendert ist.
