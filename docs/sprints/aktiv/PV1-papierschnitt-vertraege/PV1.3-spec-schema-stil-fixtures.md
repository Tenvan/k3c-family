# PV1.3 · paper-spec v1: Schema, Styleguide, Render-CLI-Vertrag, Fixtures, resvg-Probe

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** pv1/3-spec-schema-stil-fixtures
- **Abhängig von:** PV1.1, PV1.6
- **Tickets:** B-699
- **Kriterien:** AC-03, AC-04, AC-05

## Ziel

Auf `develop` liegen das JSON-Schema `paper-spec` v1, die erste Stil-Datei, der Standard `docs/standards/paper-spec.md` mit dem CLI-Vertrag, positive und negative Fixtures und das Ergebnis der resvg-Probe samt vorgerenderten Fixture-PNGs.

## Kontext

- Entscheidung `docs/decisions/005-papierschnitt-assets-mcp.md` › 1–3 und › Kriterien AC-01/02.
- Prototyp `docs/decisions/005/prototyp-monarch-rig.html`: Palette `PAL`, Filter `ds`, `ds2`, `paper`, Teil-Pfade von Monarch und Pferd, IK (`gait`, `ik`), Beinpfad (`drawLegs`), Umhang (`cloth`).
- Quell-SVGs aus PV1.6 unter `tools/paper/fixtures/svg/`; diese Session braucht keinen Browser.
- JSON-Schema-Prüfung mit `ajv` (PV1.1); fachliche Regeln, die ein Schema nicht ausdrückt, in `tools/paper/schema/check.ts`. Laufzeit bun.

## Erlaubte Dateien

- `tools/paper/schema/` (Schemas, `check.ts`, `hash.ts`, Tests)
- `tools/paper/fixtures/` (Specs, PNG, Manifeste; `svg/` nur lesen)
- `tools/paper/probe/` (resvg-Probe)
- `public/grafik/stil.paper.json`
- `docs/standards/paper-spec.md`

## Nicht-Ziele

Kein Renderer und keine Rig-Logik (RD1); die Fixture-PNGs entstehen aus dem Prototyp-Export, nicht aus Spec-Code.

## Schritte

1. **Tests zuerst** `tools/paper/schema/schema.test.ts`: `fixtures/specs/{monarch,pferd,mauer}.paper.json` gültig; `fixtures/specs/ungueltig/{hex-farbe,unbekanntes-feld,fremdes-material,pflicht-teil-fehlt,proportion}.paper.json` liefern genau `hex-farbe`, `schema`, `material-unbekannt`, `pflicht-teil-fehlt`, `proportion`; `stil.paper.json` gültig; `hash.test.ts`: `specHash` gleich für LF/CRLF und für umsortierte Schlüssel. Rot.
2. **Stil-Datei** `public/grafik/stil.paper.json`: `materialien` Name → `{licht, basis, schatten}` mit den Werten aus `PAL` (`haut`, `haar`, `gold`, `koenigsblau`, `koenigsrot`, `hermelin`, `leder`, `stoff`, `fell-fuchs`, `maehne`) plus `smaragd`, `purpur`, `stein`, `holz` (je drei Töne, neu festlegen); `einzeltoene` Name → Hex (`huf`, `auge`, `weiss`) – ausdrücklich ohne Töne; `papier: {lage: {dx:0.5, dy:0.9, unschaerfe:0.45, deckkraft:0.55, farbe:"#1a0f14"}, auflage: {dx:0.3, dy:0.5, unschaerfe:0.3, deckkraft:0.35}, struktur: {frequenz:1.3, oktaven:2, staerke:0.18}, falz_deckkraft:0.25}`; `licht: "rechts-oben"`; `proportionen: {figur: {kopf_anteil: 0.286, toleranz: 0.15}, vierbeiner: {rumpfhoehe_kopflaengen: 2, toleranz: 0.2}}`; `skalierung: {pc: 1}`.
3. **Asset-Schema** `paper-spec.schema.json` (Draft 2020-12, `additionalProperties: false`): Pflicht `id` (= Dateiname ohne `.paper.json`), `rig` (`figur|vierbeiner|gebaeude|requisite|hintergrund`), `version`, `beschreibung`, `teile`, `ausgabe`. Teil `{id, eltern?, pivot:[x,y], lagen:[Lage]}`; **Lage** nach `art`: `flaeche` `{form, material, ton?}`, `falz` `{form, breite, material, ton?}` (Linie, Deckkraft aus `papier.falz_deckkraft`), `punkt` `{mitte:[x,y], radius, material, ton?}`; `material` ist ein Material- oder Einzelton-Name. Optional `umhang {anker, punkt, material, laenge, breite:[oben,unten], segmente?}`, `bein {breiten_vorn:[6], breiten_hinten:[6]}` (nur `vierbeiner`), `animationen` Name → `{frames, fps?, reittier?, gangart?: schritt|trab|galopp, keyframes?}` mit `keyframes: [{t: 0..1, winkel: {teilId: Grad}}]` (zyklisch, linear interpoliert), `varianten` Name → `{tausch: {material: material}}`, `stufen` (nur `gebaeude`) `[{tausch?, zusaetze?: [Teil]}]`, `ausgabe {hoehe_px, rahmen:[x,y,b,h]}`. Koordinaten in Einheiten, Boden y = 0, Blick nach rechts.
4. **Regeln** in `check.ts` (Fehler `{feld, regel, text}`, `feld` als JSON-Pfad): `pflicht-teil-fehlt` (`figur`: `rumpf, kopf, oberarm-v, unterarm-v, oberarm-h, unterarm-h, oberschenkel-v, unterschenkel-v, oberschenkel-h, unterschenkel-h`; `vierbeiner`: `rumpf-vorn, rumpf-hinten, hals, schweif`; `gebaeude`: `korpus`; sonst mindestens ein Teil), `hex-farbe` (jedes `#` außerhalb von `form`), `material-unbekannt`, `eltern-unbekannt`, `zyklus`, `id-dateiname`, `proportion` (Figur: Höhe Teil `kopf` / Höhe `rahmen` außerhalb `kopf_anteil ± toleranz`).
5. **Hash** `hash.ts`: `specHash` = SHA-256 der kanonischen JSON-Form (Schlüssel sortiert, ohne Leerraum, LF); `renderKey` = SHA-256 aus `specHash + stilHash + rendererVersion` (Cache- und Golden-Schlüssel); Koordinaten in der SVG-Ausgabe auf 2 Nachkommastellen.
6. **Fixtures** aus dem Prototyp: Monarch (alle Teile, Falze, Hermelin-Tupfer als `punkt`, Umhang, Animationen `idle` 8, `lauf` 8, `reiten` 8, Variante `spieler2` mit `smaragd`), Pferd (Rumpfhälften, Hals mit Kopf, Schweif, `bein`-Breiten `[16,14,11.5,8.2,5.4,6.8]`/`[19,17,12.5,8,5.4,6.8]`, Animationen `schritt`, `trab`, `galopp`), Mauer (gebaeude, zwei Stufen, `stein`). Pfade unverändert übernehmen, Farben durch Material-Namen ersetzen.
7. **resvg-Probe** `tools/paper/probe/resvg-probe.ts`: rendert jede SVG aus `fixtures/svg/` mit `@resvg/resvg-js` (`fitTo: {mode: 'height', value: 192}`) nach `fixtures/render/<asset>/<name>.png`, zweimal, SHA-256 gleich. Test `probe.test.ts` prüft zusätzlich gegen `fixtures/render/hashes.json` (unter Windows erzeugt); Die CI führt ihn ohne Änderung auf Linux aus (`task test` im Job `ubuntu-latest`, nach dem Merge). Bei Abweichung Linux↔Windows: Vergleich auf Pixel-Toleranz umstellen (höchstens 0,1 % Pixel mit Kanal-Differenz > 2) und das als **Golden-Regel** in den Standard schreiben.
8. **Fixture-Manifeste** `fixtures/render/{monarch,pferd,mauer}/manifest.json` im Format aus Schritt 9.
9. **Standard** `docs/standards/paper-spec.md`: Felder, Regeln, Pflicht-Teile, Standard-Pivots (Figur: Hüfte (0,-50), Schulter (0,-84), Ellbogen (0,-70), Knie (0,-28), Hals (0,-90); Vierbeiner: Hüfte (±26/30, -56), Knie -31, Fessel -9, Rückengelenk (0,-66)), Hash- und Golden-Regel, Ausgabe-Layout und CLI:
   - Ausgabe neben der Spec: `<gruppe>/<id>/<animation>.png` (Frames horizontal), `<animation>@<variante>.png`, `manifest.json` = `{id, version, specHash, stilHash, renderer, renderKey, rahmen, frameW, frameH, animationen: {name: {frames, fps, datei, hashes[]}}, varianten[]}`. **PNGs und Manifeste werden committet** (das Spiel lädt sie direkt). Je Gruppenordner `<gruppe>/index.paper.json` = `[{id, rig, manifest}]` und `<gruppe>/golden.json`; **kein** gemeinsamer Index im Repo (das Spiel liest die Gruppen-Indizes, US1).
   - Aufruf `bun tools/paper/bin/paper.ts <befehl>`; Antwort JSON auf stdout; Exit 0 ok, 1 fachlich abgelehnt (`ok:false, fehler[]`), 2 Aufruf-/Systemfehler (stderr).
   - `validate <spec>` → `{ok, fehler[]}`; `render <spec|ordner>` → `{ok, manifeste[], dateien[]}`; `frame <spec> --animation a --frame n [--variante v] --out datei.png` → `{ok, datei, teile: {teilId: [x,y,b,h] px}}`; `diff <alt.json> <neu.json> [--stil-alt s1 --stil-neu s2]` → `{felder: [{pfad, alt, neu}], teile: [ids], stil: bool}`; `pixeldiff <a.png> <b.png> --out overlay.png` → `{anteil, boxen: [[x,y,b,h]]}`.
10. Tests grün, Commit `feat(pap): paper-spec v1, Stil-Datei, Fixtures, resvg-Probe`.

## Fertig, wenn

- [ ] AC-03: `task test -- paper` grün (drei gültige, fünf ungültige Fixtures mit genau der erwarteten Regel; Stil-Datei gültig; Hash-Tests).
- [ ] AC-04: Standard enthält Felder, Regeln, Pflicht-Teile, Pivots, Hash-/Golden-Regel, Ausgabe-Layout und alle fünf CLI-Befehle.
- [ ] AC-05: `probe.test.ts` grün lokal; der Linux-Lauf der CI nach dem Merge wird in PV1.5 geprüft; Golden-Regel (exakt oder Toleranz) festgehalten.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
