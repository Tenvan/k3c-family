# tools/paper/schema/

## Responsibility
Verträge der Papierschnitt-Grafik als Code (`docs/standards/paper-spec.md`): JSON-Schemas und TypeScript-Typen der Paper-Spec v1 und der Stil-Datei, Prüfung von Specs und Stil (Schema plus fachliche Regeln) sowie die Hash-Regel mit `renderKey` für Cache und Golden.

## Design
- Schemas: `paper-spec.schema.json` (Spec v1, `$schema` jeder `*.paper.json`), `stil.schema.json` (`public/grafik/stil.paper.json`), `prompts.schema.json` (`data/asset-prompts.json`, geprüft in `prompts.test.ts`).
- `types.ts`: Typen `Punkt`, `Ton`, `Lage`, `Teil`, `Animation`, `Fuss`, `GangartName`, `Proportionen`, `Spec`, `Stil`; Quelle der Wahrheit bleiben die Schemas.
- `check.ts`:
  - erst JSON-Schema (ajv), dann Regeln, die ein Schema nicht ausdrückt: `Regel` (`hex-farbe`, `material-unbekannt`, `material-doppelt`, `ton-ohne-toene`, `pflicht-teil-fehlt`, `teil-doppelt`, `eltern-unbekannt`, `teil-unbekannt`, `zyklus`, `id-dateiname`, `proportion`, `proportionen-rig`, `ebene-angabe`).
  - `Fehler` = `{feld, regel, text}`, `feld` als Pfad wie `teile[2].lagen[0].material`; `PFLICHT_TEILE` je Rig.
  - `checkSpec(spec, stil, datei?)`, `checkStil(stil)`, `readJson(path)`, `checkSpecFile(path, stil)`.
- `form.ts`: `box(d)` = konservativer Begrenzungsrahmen eines SVG-Pfads aus End- und Kontrollpunkten (M L H V C S Q T Z, absolut und relativ, keine Bögen); für die Proportionsregel.
- `hash.ts`: `canonical` (kanonische JSON-Form: Schlüsselreihenfolge und Zeilenenden zählen nicht, `num` für Zahlen), `specHash`, `stilHash`, `RENDERER_VERSION` (`paper-0+resvg-js-<version>`), `renderKey(spec, stil, renderer)`.

## Flow
1. `bin/paper.ts validate <spec>` → `checkSpecFile` → `readJson` → ajv → fachliche Regeln → `Fehler[]` (leer = gültig).
2. Rendern: `render/ausgabe.ts` prüft vor dem Rendern; Manifest bekommt `specHash`, `stilHash`, `renderKey`; `cli/genCache.ts` vergleicht den `renderKey`.

## Integration
- Aufrufer: `bin/paper.ts`, `render/*`, `rig/*`, `cli/*`, `fixtures/cli-stub.ts`, `fixtures/aus-prototyp.ts`, `probe/resvg-probe.ts`; Workbench-Tests in `internal/paperassets` und `internal/mcpsrv` lesen die Schemas.
- Abhängigkeit: `ajv` (Wurzel-`package.json`).
- Tests: `schema.test.ts` (mit den Negativ-Specs aus `fixtures/specs/ungueltig/`), `hash.test.ts`, `prompts.test.ts`.
