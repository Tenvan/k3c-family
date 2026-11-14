# tools/paper/rig/

## Responsibility
Bewegte Rigs des Papierschnitt-Renderers nach dem Prototyp `docs/decisions/005/prototyp-monarch-rig.html`: Figur (`figur`, Posen idle/lauf mit Umhang), Vierbeiner (`vierbeiner`, Gangarten mit IK und Beinpfad) und Reiten (Figur auf Reittier). Liefert Frames und Geometrie an `render/bild.ts` und die Trockenprüfung für `validate`.

## Design
- Eintrag: `register.ts` trägt per Seiteneffekt in `render/bild.ts` › `RIG_BILDER` ein: `figur` → `figurBild`, `vierbeiner` → `vierbeinerBild` bzw. bei Animationen mit `ebene` `reittierEbeneBild` (Reittier-Ebenen, B-752); so hängt `render/` nicht von `rig/` ab.
- Zeitregel: `animation.ts` (Phase = n / frames, Pose-Zeit = Phase / freq; Physik mit 1/120 s und 240 Schritten Einschwingen vor Frame 0; `frameSchritte`, `umhangAnker`, `umhangLauf`, `simuliere`), `takt.ts` (`ausgabeFps` = Frequenz × Frames für Rig-Posen und Gangarten, sonst `STANDARD_FPS`).
- Figur: `figur.ts` (`FIGUR_POSEN` idle/lauf, `figurFreq`, `figurPose`; Keyframes der Spec addieren sich), `figurBild.ts` (Umhang hinter allen Teilen, Teile mit Pose-Versatz, Papier-Filter), `umhang.ts` (Verlet-Kette am Anker, Schwerkraft und Fahrtwind, `DT` = 1/120 s, `umhangSvg`).
- Vierbeiner: `vierbeiner.ts` (Ride-Matrix: Versatz, Neigung um (0, −80), `massstab`; `rideMatrix`, `vierbeinerMatrizen`), `gangarten.ts` (`GANGARTEN`, `BEINE`, `gangPose`: Standphase, Schwungbahn, Rückengelenk, Kopfnicken, Schweif), `ik.ts` (Zwei-Knochen-IK, `HL1`/`HL2`), `kurve.ts` (Catmull-Rom `crPath`, `spline`), `beinpfad.ts` (Ober- und Unterbein als ein Papierstück, `beinPunkte`, `beinKanten`), `proportionen.ts` (B-715: Fußform, Beinlängen aus Pivots, erlaubte Gangarten, `figurFaktor`), `vierbeinerBild.ts` (`vierbeinerBild`, `vierbeinerGroesse`, `pruefeVierbeiner`, `gangartVon`).
- Reiten: `reiten.ts` (Reittier laden und hashen, `reitRahmen` als Hülle aus Reittier und Reiter, Ebenen-Reihenfolge `EBENEN`/`reitSegmente`, Zügel, `reitBild` für Figur-Animationen mit `reittier`, aufgerufen aus `figurBild`; `reittierEbeneBild`), `reitBoxen.ts` (B-709: Reittier-Geometrie mit Präfix `reittier:` für `frame`-Boxen).
- Prüfung: `pruefen.ts` (`pruefeRenderRegeln`: Animationen der Figur, Reittier, Gangart und Beinteile ohne Rastern).

## Flow
1. `render/ausgabe.ts` importiert `register.ts` → `bildVon(spec)` → `RIG_BILDER[spec.rig]` → `figurBild` (`figurPose` → `weltMatrizen` → `simuliere` Umhang → SVG) bzw. `vierbeinerBild` (`gangPose` → `ik` → `beinpfad` → SVG); Figur-Animation mit `reittier` → `reitBild` (`reitZustand` → Reittier + Reiter → Segmente).
2. `bin/paper.ts validate` → `pruefeRenderRegeln(spec)` → Fehler als `Abgelehnt`.
3. `cli/frame.ts` → `reittierGeometrie` für Boxen beim Reiten.

## Integration
- Abhängigkeiten: `render/` (Matrix, Baum, Pose, SVG, Raster, Stufen, Stil, Bild, Fehler), `schema/` (Typen, Hash).
- Aufrufer: `render/ausgabe.ts`, `render/bild.ts`, `bin/paper.ts`, `cli/frame.ts`, `cli/genCache.ts`, `cli/genParallel.ts` (`takt`).
- Tests: je Modul `*.test.ts` (u. a. `animation`, `gangarten`, `reiten`, `register`, `stand`, `ebenen`).
