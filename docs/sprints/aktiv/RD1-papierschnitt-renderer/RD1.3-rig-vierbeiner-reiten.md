# RD1.3 · Rig vierbeiner: IK, Gangarten, Rückengelenk, Beinpfad; Reiten als Kombination

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** renderer
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** rd1/3-rig-vierbeiner-reiten
- **Abhängig von:** RD1.2
- **Tickets:** B-699
- **Kriterien:** AC-02, AC-03

## Ziel

Der Renderer zeichnet `vierbeiner`-Specs in den Gangarten `schritt`, `trab`, `galopp` mit Zwei-Knochen-IK, Rückengelenk und
Beinpfad wie im Prototyp, und Reit-Animationen einer `figur` als Kombination aus Reittier, Umhang und Reiter. Pferd (3
Gangarten) und Monarch `reiten` haben feste Golden-Hashes.

## Kontext

- Stand nach RD1.2: `rig/{kurve,figur,umhang,animation}.ts`; `posen()` wirft für `vierbeiner` und Animationen mit
  `reittier` noch „folgt in RD1.3“. Zeit-Regel wie RD1.2 (Phase `n / frames`, Physik 1/120 s, 240 Schritte Einschwingen).
- Referenz-Verhalten im Prototyp `docs/decisions/005/prototyp-monarch-rig.html`:
  - `HL1 = 25`, `HL2 = 31`, `HLEGS`, `GAITS` (Zeilen 255–276), `ik` (277–282), `spline` (283–288), `gait` (289–335);
  - Beinpfad `drawLegs` (195–215, Breiten `LEGW` Zeile 187); Komposition `horseBack` / `horseFront` (230–236) und
    `apply` (362–381) mit Zügeln.
- Namen Prototyp → Spec: Beine `fB` → `-hf`, `nB` → `-hn`, `fF` → `-vf`, `nF` → `-vn`; `hu<bein>` → `oberbein-<bein>`,
  `hl<bein>` → `unterbein-<bein>`, `hh<bein>` → `huf-<bein>`; `neck` → `hals`, `tail` → `schweif`, `spine` → Winkel von
  `rumpf-hinten` (Pivot 0, −66); Gangart `walk|trot|gallop` → `schritt|trab|galopp`.
- Pferd-Fixture `tools/paper/fixtures/specs/pferd.paper.json`: `oberbein-*` und `unterbein-*` haben **keine Lagen**
  (der Beinpfad zeichnet sie), `huf-*` je 2 Flächen; `bein {breiten_vorn: [16, 14, 11.5, 8.2, 5.4, 6.8], breiten_hinten:
  [19, 17, 12.5, 8, 5.4, 6.8]}`; `ausgabe {hoehe_px: 192, rahmen: [-68, -122, 145, 126], massstab: 1.3}` → Frame
  221 × 192. Reihenfolge der Teile: ferne Beine, `schweif`, `rumpf-hinten`, `rumpf-vorn`, nahe Beine, `sattel`, `hals`.
- Monarch-Fixture: `reiten {frames: 8, reittier: pferd, gangart: galopp}`. Fixtures nur lesen, Tests im Temp-Ordner.
- Sprint-Festlegungen (README › Regeln): Reittier-Suche, Reit-Rahmen, Zeichenreihenfolge beim Reiten, Beinpfad-Material,
  Zügel.

## Erlaubte Dateien

- `tools/paper/rig/`
- `tools/paper/render/` (nur `pose.ts`, `svg.ts`, `raster.ts`, `ausgabe.ts` und Tests anpassen)
- `tools/paper/golden/`
- **Nicht:** `tools/paper/schema/`, `tools/paper/fixtures/`

## Nicht-Ziele

Keine Übergänge zwischen Gangarten (`blend`), kein Boden-Scroll (`ticks`), kein Boden-Schatten, keine weiteren
Vierbeiner-Rigs; die öffentliche Monarch-Spec mit fünf Animationen folgt in RD1.4.

## Schritte

Arbeitsweise: `superpowers:test-driven-development`, je Schritt erst der rote Test.

1. **IK** – Test `tools/paper/rig/ik.test.ts`:
   - `it('Bein fast gestreckt: ik(0, 56, true) liefert beide Winkel betragsmäßig < 3°')` – volle Länge `HL1 + HL2 = 56`,
     gekappt auf 55.99;
   - `it('Knie vorn/hinten spiegelt die Beugung')` – `ik(10, 40, true)[1] === -ik(10, 40, false)[1]`;
   - `it('Ziel außerhalb der Reichweite wird auf |HL1 − HL2| + 0.01 bzw. HL1 + HL2 − 0.01 gekappt')`.
   Umsetzung `tools/paper/rig/ik.ts`: `HL1 = 25`, `HL2 = 31`, `ik(dx, dy, knieVorn: boolean): [number, number]` in Grad.
2. **Gangarten** – Test `tools/paper/rig/gangarten.test.ts`:
   - `it('GANGARTEN: schritt freq 0.9, trab 1.5, galopp 2.1; stance 0.65 / 0.45 / 0.25')`;
   - `it('Standphase: Huf am Boden (ty = 0), bewegt sich von +r nach −r, r = reach · massstab')`;
   - `it('Galopp-Schwungbahn folgt spline(path.front|back)')`;
   - `it('gangPose(galopp, 0): Winkel für alle 12 Beinteile, hals = nod, schweif = tail, rumpf-hinten = spine')`;
   - `it('wind = 2r / (stance / freq)')` (Fahrtwind für den Umhang).
   Umsetzung `tools/paper/rig/gangarten.ts`: `GANGARTEN` (Werte 1:1 aus `GAITS`), `gangPose(spec: Spec, gangart, phase):
   {winkel, ride: {dy, pitch}, reiter: {versatz: Punkt, drehung: number, winkel}, wind}`; `rig/vierbeiner.ts`:
   `vierbeinerMatrizen(spec, pose): Map<string, M>` mit Ride-Matrix `translate(0, dy) · rotate(pitch um (0, −80))`,
   darin `scale(massstab)`; das Rückengelenk ist der Winkel von `rumpf-hinten`.
3. **Beinpfad** – Test `tools/paper/rig/beinpfad.test.ts`:
   - `it('Stützpunkte: [x, −72], Hüfte, Hüfte + d1·12, Knie, Knie + d2·11, Fessel')` – Bein senkrecht → x konstant;
   - `it('Breite je Stützpunkt aus bein.breiten_vorn|hinten, vorn Wulst 1.6 am Punkt 2')`;
   - `it('Beinpfad steht in der Zeichenreihenfolge an der Stelle von oberbein-<bein>')`;
   - `it('Material: erste flaeche von rumpf-vorn (vorn) bzw. rumpf-hinten (hinten), Schattenkante Ton schatten, Lichtkante falz')`.
   Umsetzung `tools/paper/rig/beinpfad.ts`: `beinpfad(bein: 'hf'|'hn'|'vf'|'vn', winkel, spec, stil): string` wie
   `drawLegs` (Körper `lgB` als flaeche mit `ds`, Schatten `lgS` als auflage, Licht `lgH` als falz).
4. **Pferd rendern** – Test `tools/paper/rig/vierbeiner.test.ts`:
   - `it('render pferd: schritt.png, trab.png, galopp.png je 8 Frames, Frame 221 × 192')`;
   - `it('Hufe der Standbeine berühren den Boden: Teil-Box huf-* unten bei y = 0 ± 1 Einheit')`.
5. **Reiten** – Test `tools/paper/rig/reiten.test.ts`:
   - `it('Reittier-Suche: erst Ordner der Spec, dann ../*/<id>.paper.json; fehlt es → Exit 1, Regel reittier-unbekannt')`;
   - `it('reittier ohne rig vierbeiner → Exit 1, Regel reittier-unbekannt')`;
   - `it('Reihenfolge: Teile des Reittiers bis vor hals, Umhang, Reiter, hals und Rest, Zügel')`;
   - `it('oberschenkel-h samt Kindern ist beim Reiten ausgeblendet')`;
   - `it('Reiter: versatz [−2, −54 + riderY], Drehung lean um (0, −50), Winkel aus gangPose')`;
   - `it('Umhang beim Reiten: Boden −103, Wind aus der Gangart')`;
   - `it('Reit-Rahmen: Hülle aus reittier.rahmen · massstab und reiter.rahmen + [−2, −54], Pixel je Einheit wie der Reiter')`
     – Monarch `reiten`: Rahmen `[-89, -199, 190, 205]` (nach außen auf ganze Einheiten gerundet), Frame
     `round(192 · 190 / 151) × round(192 · 205 / 151)` = 242 × 261; das Manifest trägt die Abweichung in
     `animationen.reiten.{rahmen, frameW, frameH}`.
   Umsetzung `tools/paper/rig/reiten.ts`: `findeReittier(specPfad, id): string`, `reitRahmen(reiter, reittier)`,
   `reitSvg(…)`; Zügel als `falz` `breite 1.3`, Material `leder`, Ton `schatten`, Kurve `M Hand Q Mitte(+8 y) Maul` mit
   Hand = (0, −50) in `unterarm-v`, Maul = (60, −91) in `hals`.
6. **Golden** – `golden.test.ts` erweitern: `it('pferd schritt, trab, galopp und monarch komplett (idle, lauf, reiten,
   je mit spieler2): zweimal byte-gleich und gleich fixtures.golden.json')`; `schreibe.ts` rendert jetzt die
   vollständigen Fixtures; Golden neu schreiben, Hashes im Ergebnis.
7. **Alle drei Reit-Gangarten** – Test `reiten.test.ts`: `it('Monarch mit schritt, trab, galopp als Reit-Animationen
   rendert ohne Fehler')` – Hilfs-Spec im Temp-Ordner (Fixture, `reiten` ersetzt durch drei Animationen mit `reittier:
   pferd`).
8. **Sichtvergleich** (AC-03): Frame 0 von `galopp.png` (Pferd) und von `reiten.png` (Monarch) mit
   `tools/paper/fixtures/render/pferd/pferd-galopp-0.png` und `monarch/monarch-galopp-0.png` per Read-Tool vergleichen;
   Abweichungen (Zügel-Deckkraft, fehlender Saum, Phase) im Ergebnis beschreiben.
9. Refactor, `superpowers:requesting-code-review` (Rig-Mathematik, > 150 Zeilen), `superpowers:verification-before-completion`.

## Fertig, wenn

- [ ] AC-03: `ik.test.ts`, `gangarten.test.ts`, `beinpfad.test.ts`, `vierbeiner.test.ts`, `reiten.test.ts` grün;
      Monarch rendert Idle, Lauf und Reiten in Schritt, Trab, Galopp; Sichtvergleich im Ergebnis.
- [ ] AC-02 (Pferd, Monarch-Fixture): `golden.test.ts` grün; `fixtures.golden.json` enthält `pferd` (3 Dateien) und
      `monarch` (6 Dateien).
- [ ] `task test -- tools/paper` grün, `task check:diff` grün.

## Prüfen

```bash
task test -- tools/paper
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-03 geprüft: `rig/ik.test.ts`, `gangarten.test.ts`, `beinpfad.test.ts`, `vierbeiner.test.ts`, `reiten.test.ts` grün (`task test` grün, `task typecheck` grün). Monarch rendert Idle, Lauf und Reiten in Schritt, Trab, Galopp (Test „Monarch mit schritt, trab, galopp …“).
- AC-02 (Pferd, Monarch-Fixture) geprüft: `golden.test.ts` – `fixtures.golden.json` mit `pferd` (`galopp.png` `91be7bce…`, `schritt.png` `dffb3d54…`, `trab.png` `98b451a3…`) und `monarch` komplett (6 Dateien, neu `reiten.png` `b128963c…`, `reiten@spieler2.png` `c1db33b1…`; idle/lauf unverändert). Golden neu geschrieben, weil die Fixture jetzt mit `reiten` gerendert wird.
- Sichtvergleich: Pferd-Galopp gegen `pferd-galopp-0.png` – gleiche Silhouette, gesammelte Beine in der Schwebephase, Satteldecke, Schweif; Reiten gegen `monarch-galopp-0.png` – Reiter im Sattel, Umhang weht im Galopp waagerecht nach hinten, Zügel zur Hand; Zügel als `falz` blasser als im Prototyp (Deckkraft 0.25), Saum fehlt (D-08).
- Befund (Prototyp-Verhalten): im Trab hebt der Körper bis `dy = −1.5`, das gestreckte Bein erreicht den Boden nicht ganz (IK kappt bei 55.99), der Huf schwebt bis 3.5 Einheiten; Test mit dieser Toleranz.
- Review (Sonnet): keine schweren Befunde; behoben: Vierbeiner ohne Beinteile oder `bein` → Exit 1 `pflicht-teil-fehlt` statt Absturz, Reittier-id nur `[a-z0-9-]` (sonst `reittier-unbekannt`), eigener Reit-Rahmen per Flag `eigen` im Manifest. Leicht, belassen: Reiter-`massstab` ≠ 1 wird ignoriert, Zügel-Material `leder` fest (D-09).
- Abweichungen: zusätzliche Module `rig/vierbeiner.ts` (Ride-Matrix), `rig/vierbeinerBild.ts`; Vierbeiner zeichnet in Welt-Einheiten mit `viewBox = rahmen · massstab` (gleiche Pixel wie der Spec-Rahmen); Reit-Animation ohne `gangart` reitet `schritt`, eine Vierbeiner-Animation ohne `gangart` wird mit `animation-unbekannt` abgelehnt; Laufzeit Monarch komplett ≈ 4 s (resvg-Filter).
