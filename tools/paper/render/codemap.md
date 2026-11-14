# tools/paper/render/

## Responsibility
Kern des Papierschnitt-Renderers (`docs/standards/paper-spec.md` › 2, 3, 6–8): baut aus Spec und Stil je Ausgabe (Animation, Variante, Gebäude-Stufe) SVG-Frames, rastert sie mit resvg zu Frame-Streifen, berechnet Frame-Hashes und schreibt Manifest, PNGs und Gruppen-Index atomar.

## Design
- Geometrie: `matrix.ts` (2D-Affin-Matrizen im SVG-Format; `mul`, `verschiebe`, `skaliere`, `drehung`, `anwenden`, `invers`; Winkel in Grad, positiv im Uhrzeigersinn), `baum.ts` (`weltMatrizen`: Teil dreht um seinen Pivot, Kinder erben).
- Bewegung: `pose.ts` (Phase von Frame n = n / frames; Keyframes linear und zyklisch, `keyframeWinkel`, `keyframeAusgeblendet`, `animationVon`, `posen`).
- Stil und Stufen: `stil.ts` (`STIL_PFAD` = `public/grafik/stil.paper.json`, `ladeStil`, `farbe`, `tausche`/`tauscheTeil` für Varianten und Material), `stufen.ts` (`stufenSpec`: Stufe n = Zusätze und Tausch der Stufen 1..n-1; `ausgaben`/`ausgabeSpec` = Ausgabedateien je Animation, Variante, Stufe).
- Zeichnen: `svg.ts` (Lagen und Teile als SVG, Filter für Schatten und Papierstruktur aus dem Stil, Transformationen als `rotate(…)`-Kette wegen der 2-Nachkommastellen-Regel; `defs`, `lageSvg`, `teileSvg`, `seed`, `dichteFaktor`), `bild.ts` (`einfachesBild` für Rigs ohne eigene Bewegung, `RIG_BILDER` als Eintragsstelle für `rig/`, `bildVon` wählt).
- Raster: `raster.ts` (`frameGroesse`, `streifenSvg` = ein verschachteltes `<svg>` je Frame mit viewBox = Rahmen, `rastere` mit resvg, `frameHashes` = SHA-256 der RGBA-Pixel je Ausschnitt), `png.ts` (`liesPng`, `groesse` aus IHDR, `abweichend` für Pixelvergleich, ohne weitere Abhängigkeit).
- Ausgabe: `ausgabe.ts` (`rendereAsset` im Speicher → Manifest + PNGs, `schreibe` atomar über Temp-Ordner und Umbenennen mit Rückfall auf den alten Stand, Aufräumen verwaister `.tmp`/`.alt`-Ordner, `renderSpec` für eine Spec, `renderOrdner`/`specsIn` für eine Gruppe samt `index.paper.json`); importiert `rig/register.ts` mit Seiteneffekt.
- Fehler: `fehler.ts` (`CliFehler`, `Aufruf` → Exit 2, `Abgelehnt` mit `{feld, regel, text}` → Exit 1).

## Flow
`renderSpec(pfad)` → Spec lesen und prüfen (`schema/check.ts`) → `ausgaben(spec)` → je Ausgabe `bildVon` (Rig aus `RIG_BILDER` oder `einfachesBild` → `weltMatrizen` → `teileSvg`) → `streifenSvg` → `rastere` → `frameHashes` → Manifest mit `renderKey` und Abspielrate (`rig/takt.ts`) → `schreibe`.

## Integration
- Aufrufer: `bin/paper.ts` (`render`), `cli/gen.ts`, `cli/genParallel.ts`, `cli/frame.ts`, `golden/fixtures.ts`; `rig/` nutzt Matrix, Baum, Pose, SVG, Raster und Bild.
- Abhängigkeiten: `@resvg/resvg-js`, `schema/` (Typen, Prüfung, Hash), `node:crypto`, `node:fs`.
- Tests: `baum`, `pose`, `raster`, `render`, `stil`, `stufen`, `svg` (`*.test.ts`).
