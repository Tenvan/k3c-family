# RD1.1 · Renderer-Kern: Stil, Teile-Baum, Lagen, SVG, validate und render für requisite und gebaeude

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** renderer
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** rd1/1-kern-validate-render
- **Abhängig von:** PV1.5
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02

## Ziel

`bun tools/paper/bin/paper.ts validate|render` erfüllt den CLI-Vertrag für Specs ohne Bewegung (`requisite`, `gebaeude`,
`hintergrund`): Stil auflösen, Teile-Baum, Lagen `flaeche|falz|punkt` als SVG mit Papier-Filtern, resvg-Raster,
Ausgabe-Layout mit Manifest und Gruppen-Index. Die Mauer (Stufe 1 und 2) hat feste Golden-Hashes.

## Kontext

- Vertrag (nicht ändern): `docs/standards/paper-spec.md` › 3 (Felder, Lagen-Arten), 6 (Hash, Golden, Determinismus),
  7 (Ausgabe, Manifest), 8 (CLI). Typen `tools/paper/schema/types.ts`; Regeln `checkSpecFile(path, stil)` und
  `readJson(path)` aus `tools/paper/schema/check.ts`; `canonical`, `specHash`, `stilHash`, `renderKey`, `num`,
  `RENDERER_VERSION = 'paper-0+resvg-js-2.6.2'` aus `tools/paper/schema/hash.ts`; `points(d)`, `box(forms)` aus
  `tools/paper/schema/form.ts`. Stil-Datei: `public/grafik/stil.paper.json`.
- Attrappe als Vorbild für Aufbau und Exit-Codes: `tools/paper/fixtures/cli-stub.ts` (`run(args) → {code, stdout,
  stderr}`, Aufruffehler Exit 2 nur auf stderr, `rel()` für Repo-relative Pfade mit `/`). Die echte CLI bekommt dieselbe
  in-process-Schnittstelle, damit Vitest (Node) sie ohne Kindprozess aufruft; deshalb **nur `node:`-Module, keine
  Bun-APIs** (Ausnahme `import.meta.main` im Einstieg).
- resvg: `@resvg/resvg-js` 2.6.2 (schon in `package.json`), Aufruf wie `tools/paper/probe/resvg-probe.ts`
  (`new Resvg(svg, { font: { loadSystemFonts: false } })`). Die Probe zeigt: `feDropShadow` und `feTurbulence` rendern
  unter Windows zweimal byte-gleich.
- Filter wie im Prototyp `docs/decisions/005/prototyp-monarch-rig.html` (Zeilen 320–327), Werte aus der Stil-Datei:
  `ds` = `feDropShadow dx dy stdDeviation=unschaerfe flood-color=farbe flood-opacity=deckkraft` aus `papier.lage`;
  `ds2` aus `papier.auflage` (Farbe von `lage`); `paper` = `feTurbulence type=fractalNoise baseFrequency=frequenz
  numOctaves=oktaven seed=<Seed>` → `feColorMatrix saturate 0` → `feComponentTransfer` (R, G, B linear
  `slope=staerke`, `intercept=0.86`, A `slope 0 intercept 1`) → `feComposite arithmetic k1=1`. Filter-Region wie im
  Prototyp (`-30% / 160%` bzw. `-5% / 110%`).
- Fixture-Specs `tools/paper/fixtures/specs/{mauer,monarch,pferd}.paper.json` sind Vertrag und **nur zu lesen**: Tests
  kopieren sie in einen Temp-Ordner (`mkdtempSync(join(tmpdir(), 'paper-'))`), weil `render` neben die Spec schreibt.
- Mauer: `rig: gebaeude`, ein Teil `korpus`, `animationen.stand` mit 1 Frame, `stufen` mit einem Eintrag (= Stufe 2),
  `ausgabe {hoehe_px: 128, rahmen: [-66, -112, 132, 116]}` → Frame `128 × round(128·132/116) = 146` px breit.
- Festlegungen des Sprints (README › Regeln): Seed, Frame-Streifen, Stil-Übergabe, Golden-Datei der Fixtures.

## Erlaubte Dateien

- `tools/paper/bin/paper.ts` (neu, Einstieg)
- `tools/paper/render/` (neu)
- `tools/paper/golden/` (neu)
- **Nicht:** `tools/paper/schema/`, `tools/paper/fixtures/` (Vertrag, nur lesen)

## Nicht-Ziele

Keine Bewegung (`figur`, `vierbeiner`, Keyframes werden nur durchgereicht, siehe Schritt 6), kein `frame`, `diff`,
`pixeldiff` (RD1.4), kein `gen:paper`, keine Dateien unter `public/grafik/`, kein Render-Cache `.cache/paper/`.

## Schritte

Arbeitsweise: `superpowers:test-driven-development`, je Schritt erst der rote Test.

1. **Stil** – Test `tools/paper/render/stil.test.ts`:
   - `it('farbe: Material mit Ton, ohne Ton basis')` – `farbe(stil, 'stein', 'licht') === '#c2bdb1'`,
     `farbe(stil, 'stein') === '#9a9386'`;
   - `it('farbe: Einzelton ohne Ton')` – `farbe(stil, 'huf') === '#3a2c2a'`;
   - `it('tausche ersetzt Materialien in Lagen und Umhang')` – `tausche(spec, {koenigsblau: 'smaragd'})` liefert eine
     Kopie, Original unverändert.
   Umsetzung `tools/paper/render/stil.ts`: `farbe(stil: Stil, material: string, ton?: Ton): string`,
   `tausche(spec: Spec, tausch: Record<string, string>): Spec`, `ladeStil(pfad?: string): Stil` (Standard
   `public/grafik/stil.paper.json` relativ zur Repo-Wurzel `resolve(import.meta.dirname, '../../..')`).
2. **Matrix und Teile-Baum** – Test `tools/paper/render/baum.test.ts`:
   - `it('Kind erbt die Drehung des Elternteils um dessen Pivot')` – Spec mit `a` (Pivot 0,0) und `b` (eltern `a`,
     Pivot 10,0), Winkel `{a: 90}` → Punkt (20, 0) von `b` landet bei (0, 20) (y nach unten, positiv = Uhrzeigersinn);
   - `it('Zeichenreihenfolge = Listenreihenfolge, Eltern dürfen später stehen')`;
   - `it('ohne Winkel ist jede Matrix die Einheit')`.
   Umsetzung `tools/paper/render/matrix.ts` (`type M = [a, b, c, d, e, f]`, `mul`, `drehung(grad, px, py)`,
   `anwenden(m, [x, y])`, `verschiebe`, `skaliere`) und `tools/paper/render/baum.ts`:
   `weltMatrizen(spec: Spec, winkel: Record<string, number>, basis?: M): Map<string, M>` (Kette über `eltern`,
   je Teil `drehung(winkel[id] ?? 0, ...pivot)`).
3. **Lagen → SVG** – Test `tools/paper/render/svg.test.ts`:
   - `it('flaeche: Pfad mit Füllung und filter ds, auflage mit ds2')`;
   - `it('falz: ohne Füllung, stroke-width = breite, opacity = falz_deckkraft, runde Enden')`;
   - `it('punkt: circle ohne Filter')`;
   - `it('Zahlen im SVG über num(): höchstens 2 Nachkommastellen, nie -0')` – Pivot `[0.005, -0.001]` erzeugt kein
     `-0` und keine dritte Nachkommastelle in `transform`;
   - `it('Seed der Papierstruktur hängt nur an der Asset-ID')` – `seed('mauer')` ist stabil, `seed('mauer') !==
     seed('monarch')`, Wertebereich 1..1000.
   Umsetzung `tools/paper/render/svg.ts`: `seed(id: string): number` = erste 4 Byte von SHA-256(id) als uint32,
   `% 1000 + 1`; `defs(stil, id): string`; `lageSvg(lage, stil): string`; `teileSvg(spec, stil, matrizen): string`
   (eine `<g transform="matrix(…)">` je Teil in Listenreihenfolge, Formen unverändert als `d`).
4. **Frame-Streifen und Raster** – Test `tools/paper/render/raster.test.ts`:
   - `it('frameGroesse: Breite = round(hoehe_px · b / h)')` – Mauer → `{frameW: 146, frameH: 128}`;
   - `it('streifen: N Frames nebeneinander, Bildbreite N · frameW')`;
   - `it('frameHashes: je Frame SHA-256 der RGBA-Pixel des Ausschnitts')`;
   - `it('zwei Läufe byte-gleich')`.
   Umsetzung `tools/paper/render/raster.ts`: `frameGroesse(spec)`, `streifenSvg(frames: string[], spec): string`
   (ein SVG `width = N·frameW`, je Frame ein verschachteltes `<svg x=i·frameW width=frameW height=frameH
   viewBox="rahmen">`, damit Schatten nicht in den Nachbar-Frame laufen), `rastere(svg): {png: Buffer, rgba:
   Uint8Array, breite, hoehe}`, `frameHashes(rgba, breite, frameW, frameH, n): string[]`.
5. **Stufen und Varianten** – Test `tools/paper/render/stufen.test.ts`:
   - `it('Stufe n wendet stufen[0..n-2] kumulativ an: tausch und zusaetze')`;
   - `it('Ausgabedateien: stand.png, stand.s2.png, stand@v.png, stand@v.s2.png')` – Hilfs-Spec mit Variante `v`.
   Umsetzung `tools/paper/render/stufen.ts`: `stufenSpec(spec, n): Spec`, `ausgaben(spec): {schluessel, animation,
   variante?, stufe}[]` (Schlüssel = Dateiname ohne `.png`, Reihenfolge: Animationen alphabetisch, je Animation
   ohne Variante zuerst, Stufen aufsteigend).
6. **Pose-Schnittstelle** – Test `tools/paper/render/pose.test.ts`: `it('requisite mit keyframes: Winkel linear
   interpoliert, zyklisch')` – Keyframes `[{t: 0, winkel: {k: 0}}, {t: 0.5, winkel: {k: 90}}]`, 4 Frames → Winkel
   0, 45, 90, 45. Umsetzung `tools/paper/render/pose.ts`: `type Pose = {winkel: Record<string, number>; basis?: M}`,
   `keyframeWinkel(anim, phase): Record<string, number>`, `posen(spec, animation): Pose[]` (Phase = `n / frames`);
   `figur` und `vierbeiner` werfen hier `Aufruf`-Fehler „Rig … folgt in RD1.2/RD1.3“ (Exit 2).
7. **render und Manifest** – Test `tools/paper/render/render.test.ts`:
   - `it('render mauer: manifest.json nach Vertrag, Dateien stand.png und stand.s2.png')` – Felder `id, version,
     specHash, stilHash, renderer, renderKey, rahmen, frameW, frameH, animationen{stand, stand.s2}, varianten: []`;
     `renderKey === renderKey(specHash, stilHash)`;
   - `it('render ordner: alle Specs außer index.paper.json, schreibt Gruppen-Index [{id, rig, manifest}]')`;
   - `it('render ist atomar: bei Fehler bleibt der alte Ausgabeordner')` – zweiter Lauf mit ungültiger Spec im Ordner
     → Exit 1, alter `<id>/` unverändert, kein Temp-Ordner übrig;
   - `it('Pfade in der Antwort relativ zum Repo mit /')`.
   Umsetzung `tools/paper/render/ausgabe.ts`: `renderSpec(pfad, {stil?: Stil}): {manifest, dateien}`,
   `renderOrdner(ordner, opts)`; Schreiben erst nach `<ordner>/.<id>.tmp-<pid>/`, dann `renameSync` (alten Ordner
   vorher nach `.<id>.alt` umbenennen, nach Erfolg löschen).
8. **CLI** – Test `tools/paper/bin/paper.test.ts`:
   - `it('validate monarch: Exit 0, {ok: true, fehler: []}')`;
   - `it('validate ungültige Fixtures: Exit 1 mit genau der Regel')` – `it.each` über `specs/ungueltig/*.paper.json`
     → `hex-farbe`, `schema`, `material-unbekannt`, `pflicht-teil-fehlt`, `proportion`;
   - `it('unbekannter Befehl und fehlende Datei: Exit 2, nichts auf stdout')`.
   Umsetzung `tools/paper/bin/paper.ts`: `export function run(args: string[], opts?: {stil?: Stil}): Ergebnis`
   (Typ wie Attrappe), Befehle `validate`, `render`; `frame`, `diff`, `pixeldiff` antworten bis RD1.4 mit Exit 2
   „folgt in RD1.4“.
9. **Golden Mauer** – Test `tools/paper/golden/golden.test.ts`:
   - `it('mauer: stand und stand.s2 zweimal byte-gleich und gleich fixtures.golden.json')`;
   - `it('gleich: exakt vergleicht SHA-256, toleranz erlaubt ≤ 0,1 % Pixel mit Kanal-Differenz > 2')` – zwei
     Hilfs-PNGs mit 1 abweichendem Pixel von 10 000 → `exakt` false, `toleranz` true.
   Umsetzung `tools/paper/golden/regel.ts`: `GOLDEN_REGEL: 'exakt' | 'toleranz'` (Wert aus `paper-spec.md` › 6 nach
   PV1.5), `gleich(a: Buffer, b: Buffer, regel = GOLDEN_REGEL): boolean`; `tools/paper/golden/schreibe.ts`
   (`bun tools/paper/golden/schreibe.ts` rendert die Fixture-Specs im Temp-Ordner und schreibt
   `tools/paper/golden/fixtures.golden.json` im Format von `golden.json`); Golden einmal schreiben, Hash im Ergebnis
   nennen.
10. Refactor (Dateien ≤ 400, Funktionen ≤ 60 Zeilen), `superpowers:requesting-code-review` (Risiko: neuer Code
    > 150 Zeilen), dann `superpowers:verification-before-completion` mit den Befehlen unter Prüfen.

## Fertig, wenn

- [ ] AC-01: `bun tools/paper/bin/paper.ts validate tools/paper/fixtures/specs/monarch.paper.json` → Exit 0
      `{"ok":true,"fehler":[]}`; jede Datei unter `specs/ungueltig/` → Exit 1 mit genau ihrer Regel (`paper.test.ts`).
- [ ] AC-02 (Mauer): `golden.test.ts` grün; `fixtures.golden.json` enthält `mauer` mit `stand.png` und `stand.s2.png`.
- [ ] `bun tools/paper/bin/paper.ts render <temp>/mauer.paper.json` schreibt `<temp>/mauer/{stand.png,stand.s2.png,manifest.json}`.
- [ ] `task test -- tools/paper` grün (inklusive der Vertrags-Tests aus PV1), `task check:diff` grün.

## Prüfen

```bash
task test -- tools/paper
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
