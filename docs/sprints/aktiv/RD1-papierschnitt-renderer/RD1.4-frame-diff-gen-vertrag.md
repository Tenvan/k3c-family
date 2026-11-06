# RD1.4 · frame, diff, pixeldiff, gen:paper und Vertragstest gegen die Attrappe

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** renderer
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** rd1/4-frame-diff-gen-vertrag
- **Abhängig von:** RD1.3
- **Tickets:** B-699
- **Kriterien:** AC-02, AC-04, AC-05

## Ziel

Die CLI beantwortet alle fünf Befehle des Vertrags; ein gemeinsamer Vertragstest läuft gegen Attrappe und Renderer.
`task gen:paper` rendert alle Gruppen unter `public/grafik/` samt `index.paper.json` und `golden.json`; Monarch (fünf
Animationen) und Pferd liegen gerendert in `public/grafik/figuren/` und `public/grafik/reittiere/`.

## Kontext

- Vertrag: `docs/standards/paper-spec.md` › 8 (Antworten von `frame`, `diff`, `pixeldiff`, Exit 0/1/2), › 6
  (`golden.json` = `{"<id>": {"renderKey", "dateien": {"<datei>.png": "<sha256>"}}}`), › 1 (Gruppen-Index
  `[{id, rig, manifest}]`, kein gemeinsamer Index).
- Attrappe `tools/paper/fixtures/cli-stub.ts` (`run`, `felder`, `teileDiff`) und `fixtures/cli-pixel.ts` (`pixeldiff`,
  `lies`) zeigen das erwartete Verhalten von `diff` und `pixeldiff`; der Renderer importiert **nicht** aus `fixtures/`,
  sondern hat eigene Module (Logik darf übernommen werden). Erwartete Werte: `fixtures/ui/AssetsPixelDiff.json`
  (`anteil`, `boxen` für `monarch-galopp-0.png` → `monarch-galopp-0-purpur.png`).
- Bestehender Vertragstest der Attrappe: `tools/paper/fixtures/stub.test.ts` (nicht ändern, bleibt grün).
- Stand nach RD1.3: `run(args, {stil?})` kann `validate`, `render`; `frame`, `diff`, `pixeldiff` antworten Exit 2.
  `render` liefert je Teil die Welt-Matrizen (`vierbeinerMatrizen`, `weltMatrizen`) – daraus die Teil-Boxen.
- Taskfile: Namensraum `gen` = `tools/Taskfile.yml` (wie `gen:atlas`, `gen:gebaeude`, mit `sources`/`generates`).
  `public/grafik/gebaeude/*.json` liest `gen:gebaeude` (alte Gebäude) – deshalb kommt die Mauer **nicht** nach
  `public/grafik/gebaeude/` (Sprint-Festlegung), ihre Golden bleiben in `tools/paper/golden/fixtures.golden.json`.
- MC1 ruft die CLI über `K3C_PAPER_CLI` (Standard `tools/paper/bin/paper.ts`, mit bun); nach dieser Session zeigt der
  Standard auf den echten Renderer.

## Erlaubte Dateien

- `tools/paper/bin/`, `tools/paper/cli/` (neu), `tools/paper/render/`, `tools/paper/golden/`, `tools/paper/vertrag/` (neu)
- `tools/Taskfile.yml` (nur der neue Task `paper`)
- `public/grafik/figuren/monarch.paper.json` samt Ausgabe `public/grafik/figuren/monarch/`, `index.paper.json`, `golden.json`
- `public/grafik/reittiere/pferd.paper.json` samt Ausgabe `public/grafik/reittiere/pferd/`, `index.paper.json`, `golden.json`
- **Nicht:** `tools/paper/schema/`, `tools/paper/fixtures/`, `public/grafik/stil.paper.json`, `public/grafik/gebaeude/`

## Nicht-Ziele

Kein Render-Cache `.cache/paper/` (MC1/RV1), keine MCP-Tools, keine Migration weiterer Assets (MG*), keine Änderung am
Vertrag – Lücken als Befund ins Ergebnis.

## Schritte

Arbeitsweise: `superpowers:test-driven-development`, je Schritt erst der rote Test.

1. **Vertragstest zuerst** – `tools/paper/vertrag/cli.vertrag.test.ts` mit
   `describe.each([['Attrappe', stubRun], ['Renderer', run]])` (`stubRun` aus `fixtures/cli-stub.ts`, `run` aus
   `bin/paper.ts`), alle Specs im Temp-Ordner:
   - `it('validate: ok-Form und Exit 1 mit fehler[{feld, regel, text}]')`;
   - `it('render: {ok, manifeste: string[], dateien: string[]}, Pfade relativ mit /')`;
   - `it('frame: {ok, datei, teile: {<teilId>: [x, y, b, h]}} mit ganzzahligen Pixeln; unbekannte Animation Exit 1 animation-unbekannt')`;
   - `it('diff: felder[{pfad, alt, neu}], teile über id, stil true bei anderem stilHash')` – Fälle wie
     `stub.test.ts` (Umhang `purpur`, Kopf `gold`, `falz_deckkraft 0.2`) → `umhang.material` in `felder`, `teile = ['kopf']`;
   - `it('pixeldiff: anteil und boxen wie fixtures/ui/AssetsPixelDiff.json, Overlay geschrieben')`;
   - `it('Aufruffehler: Exit 2, stdout leer')` – fehlende Optionen, unbekannter Befehl, fehlende Datei.
   Beim Renderer rot, bei der Attrappe grün.
2. **frame** – Test `tools/paper/cli/frame.test.ts`:
   - `it('frame monarch idle 0: PNG frameW × frameH, gleich dem Ausschnitt 0 von idle.png')`;
   - `it('teile: Box der transformierten Lagen in Pixeln des Frames; Teile ohne Lagen fehlen')`;
   - `it('--variante und --stufe wirken wie im Ausgabe-Layout')`.
   Umsetzung `tools/paper/cli/frame.ts`: `frame(pos, opt): Ergebnis`; Box = `box()`-Logik aus `schema/form.ts` über die
   mit der Welt-Matrix transformierten Punkte, umgerechnet `(x − rahmen[0]) · hoehe_px / rahmen[3]`, gerundet.
3. **diff** – Test `tools/paper/cli/diff.test.ts`: `it('Teil umsortiert: kein Feld, kein Teil')`, `it('Teil neu und
   gelöscht: in teile')`, `it('$schema zählt nicht')`. Umsetzung `tools/paper/cli/diff.ts`.
4. **pixeldiff** – Test `tools/paper/cli/pixeldiff.test.ts`: `it('gleiche Bilder: anteil 0, boxen []')`,
   `it('höchstens 32 Boxen, größte zuerst')`, `it('unterschiedliche Größe: fehlende Pixel zählen als Abweichung')`.
   Umsetzung `tools/paper/cli/pixeldiff.ts` (Raster 8 px, Kanal-Differenz > 2, `anteil` auf 6 Stellen wie Attrappe).
5. **Material ändert Hashes** (AC-04) – Test `tools/paper/golden/stil.test.ts`:
   - `it('stein.basis geändert: renderKey und PNG-Hashes der Mauer ändern sich, Spec unverändert')`;
   - `it('purpur geändert: renderKey der Mauer ändert sich, PNG-Hashes nicht')` (Material wird nicht benutzt);
   - `it('koenigsrot geändert: Monarch-PNGs ändern sich (Umhang), Pferd-PNGs nicht')`.
   Stil-Kopie in-process über `run(args, {stil})`.
6. **gen:paper** – Test `tools/paper/cli/gen.test.ts` (Temp-Wurzel mit zwei Gruppen):
   - `it('gen schreibt je Gruppe index.paper.json und golden.json, keinen gemeinsamen Index')`;
   - `it('Gruppen ohne *.paper.json werden übersprungen')`;
   - `it('zwei Läufe hintereinander: keine Datei ändert sich')`.
   Umsetzung `tools/paper/bin/gen.ts` (`bun tools/paper/bin/gen.ts [wurzel = public/grafik]`): je Unterordner mit
   Specs `render <ordner>`, dann `golden.json` aus den PNG-Hashes. Task in `tools/Taskfile.yml`:
   ```yaml
   paper:
     desc: Papierschnitt-Grafik rendern (public/grafik/*/*.paper.json → PNG, Manifest, Gruppen-Index, golden.json; RD1)
     sources: [tools/paper/**/*.ts, public/grafik/stil.paper.json, public/grafik/*/*.paper.json]
     cmds: [bun tools/paper/bin/gen.ts public/grafik]
   ```
   (`sources` ohne `tools/paper/fixtures/**` und Tests, sonst passend kürzen).
7. **Öffentliche Specs** – `public/grafik/reittiere/pferd.paper.json` = Kopie der Pferd-Fixture;
   `public/grafik/figuren/monarch.paper.json` = Kopie der Monarch-Fixture, `reiten` ersetzt durch `schritt`, `trab`,
   `galopp` (je `frames: 8`, `reittier: pferd`, gleichnamige `gangart`) – also Animationen `idle`, `lauf`, `schritt`,
   `trab`, `galopp`. `$schema` = `../../../tools/paper/schema/paper-spec.schema.json`. `task gen:paper` laufen lassen,
   Ausgabe committen.
8. **Golden öffentlich** (AC-02) – Test `tools/paper/golden/public.test.ts`:
   `it.each(gruppen)('%s: Render im Temp-Ordner gleich golden.json (GOLDEN_REGEL)')` und `it('monarch hat idle, lauf,
   schritt, trab, galopp; pferd hat schritt, trab, galopp')`.
9. Refactor, `superpowers:requesting-code-review` (Eingabe- und Pfad-Code, > 150 Zeilen), dann
   `superpowers:verification-before-completion`.

## Fertig, wenn

- [ ] AC-05: `cli.vertrag.test.ts` grün für Attrappe **und** Renderer; `stub.test.ts` unverändert grün.
- [ ] AC-04: `stil.test.ts` grün.
- [ ] AC-02: `public.test.ts` grün; `public/grafik/figuren/monarch/galopp.png` und `public/grafik/reittiere/pferd/galopp.png` existieren.
- [ ] `task gen:paper` zweimal hintereinander → `git status --short public/grafik` leer nach dem Commit.
- [ ] `task test -- tools/paper` grün, `task check:diff` grün.

## Prüfen

```bash
task gen:paper
task test -- tools/paper
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
