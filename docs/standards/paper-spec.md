# Standard: Paper-Spec v1, Stil-Datei, Render-CLI

Stand: 2026-11-06 (PV1.3) · Entscheidung [`005`](../decisions/005-papierschnitt-assets-mcp.md) · Domäne PAP ·
Schemas und Regeln: `tools/paper/schema/` · Fixtures: `tools/paper/fixtures/`

Dieser Standard ist ein **Vertrag**: Renderer (RD1), MCP-Tools (MC1), Workbench (WB1, RV1) und Migration (MG*) arbeiten
dagegen. Änderungen nur über ein Ticket im Projekt PAP. Begriffe: [`glossar.md`](../glossar.md) (Paper-Spec, Stil-Datei,
Papier-Material, Rig-Typ, Teil, Lage, Ton).

## 1. Dateien und Ablage

| Datei | Inhalt |
|---|---|
| `public/grafik/stil.paper.json` | Stil-Datei: einziger Ort für Farben und Papier-Effekte |
| `public/grafik/<gruppe>/<id>.paper.json` | Paper-Spec eines Assets (`<gruppe>` z. B. `figuren`, `reittiere`, `gebaeude`) |
| `public/grafik/<gruppe>/<id>/` | gerenderte Ausgabe des Assets (Abschnitt 7), **committet** |
| `public/grafik/<gruppe>/index.paper.json` | Gruppen-Index `[{id, rig, manifest}]`; ist **keine** Spec und wird von `validate` übersprungen |
| `public/grafik/<gruppe>/golden.json` | Golden-Hashes der Gruppe (Abschnitt 6) |
| `.cache/paper/<renderKey>/` | Render-Cache (nicht im Repo, `.gitignore`) |

Es gibt **keinen** gemeinsamen Index aller Gruppen; das Spiel liest die Gruppen-Indizes (US1).

## 2. Stil-Datei (`stil.schema.json`)

| Feld | Bedeutung |
|---|---|
| `materialien` | Name (`^[a-z][a-z0-9-]*$`) → `{licht, basis, schatten}` als `#rrggbb` (klein) |
| `einzeltoene` | Name → `#rrggbb`, **ohne** Töne (`huf`, `auge`, `weiss`, `linie`, `edelstein`); kein Name zugleich Material |
| `papier.lage` | Schlagschatten jedes Papierstücks: `dx`, `dy`, `unschaerfe`, `deckkraft`, `farbe` |
| `papier.auflage` | Schatten aufgeklebter Lagen: `dx`, `dy`, `unschaerfe`, `deckkraft` (Farbe wie `lage`) |
| `papier.struktur` | Papierstruktur: `frequenz`, `oktaven` (1–4), `staerke` |
| `papier.falz_deckkraft` | Deckkraft aller Falze, höchstens 0,25 |
| `papier.bezug_px_je_einheit` | Pixeldichte, bei der `lage`, `auflage` und `struktur` gemeint sind (Prototyp: 3). Der Renderer rechnet Schatten-Versatz und -Unschärfe mit `bezug / dichte` (Faktor 1…4) um, die Rausch-Frequenz mit dem Kehrwert; `dichte` = `frameH / rahmen[3]` der Ausgabe. Deckkraft und Stärke bleiben. |
| `licht` | `rechts-oben` (einziger Wert in v1) |
| `proportionen.figur` | `kopf_anteil` (Soll Kopfhöhe / Rahmenhöhe) und `toleranz` (absolut) |
| `proportionen.vierbeiner` | `rumpfhoehe_kopflaengen`, `toleranz` (Richtwert, in v1 nicht geprüft) |
| `skalierung` | Faktor je Plattform: `pc` Pflicht, `xbox`, `mobil` optional |

Startwerte (v1): Materialien aus der Palette des Prototyps (`haut`, `haar`, `gold`, `koenigsblau`, `koenigsrot`,
`hermelin`, `leder`, `stoff`, `fell-fuchs`, `maehne`) plus `smaragd`, `purpur`, `stein`, `holz` (Töne vorläufig, B-701 D-07).

## 3. Paper-Spec (`paper-spec.schema.json`, Draft 2020-12, keine unbekannten Felder)

Koordinaten in **Spec-Einheiten**, Boden `y = 0`, Blick nach rechts, `y` wächst nach unten. Keine Farben, keine Filter.

| Feld | Pflicht | Bedeutung |
|---|---|---|
| `$schema` | nein | Pfad zum Schema (Editor-Hilfe); zählt nicht zum Hash |
| `id` | ja | = Dateiname ohne `.paper.json` |
| `rig` | ja | `figur`, `vierbeiner`, `gebaeude`, `requisite`, `hintergrund` |
| `version` | ja | ganze Zahl ≥ 1; erhöhen bei gewollter sichtbarer Änderung |
| `beschreibung` | ja | Absicht in Worten; verbindlich sind die Teile |
| `teile` | ja | Liste der Teile; **Zeichenreihenfolge = Reihenfolge der Liste** (Eltern müssen nicht vorher stehen) |
| `umhang` | nein | `{anker, punkt:[x,y], material, laenge, breite:[oben, unten], segmente?}` – Umhang-Physik des Rigs |
| `bein` | nur `vierbeiner` | `{breiten_vorn:[6], breiten_hinten:[6]}` – Breite des Beinpfads an Schulter/Hüfte, Oberschenkel, Knie, Unterbein, Fessel, Huf |
| `proportionen` | nein | `vierbeiner`: `{fuss?: huf|pfote|elefantenfuss, gangarten?: [schritt, trab, galopp]}`; `figur`: `{arme?, beine?, rumpf?, kopf?}` (Faktoren 0,1–3, Standard 1). Felder am falschen Rig lehnt `validate` ab (`proportionen-rig`) |
| `animationen` | nein | Name → `{frames (1–64), fps? (Standard 8), reittier?, gangart?: stand|schritt|trab|galopp, ebene?: reiter|hinten|vorn, reiter?, keyframes?}` |
| `varianten` | nein | Name → `{tausch: {material: material}}` |
| `stufen` | nur `gebaeude` | Delta je weiterer Ausbaustufe: `stufen[0]` = Stufe 2, `{tausch?, zusaetze?: [Teil]}` |
| `ausgabe` | ja | `{hoehe_px (8–1024), rahmen:[x, y, b, h], massstab?}` |

- **Teil:** `{id, eltern?, pivot:[x,y], lagen:[Lage]}`. Ein Teil dreht um seinen Pivot, Kinder erben die Drehung.
- **`keyframes`:** `[{t: 0..1, winkel: {teilId: Grad}, sichtbar?: {teilId: bool}}]`, zyklisch, linear interpoliert; Grad positiv = im Uhrzeigersinn.
  `sichtbar` blendet ein Teil (nur dieses, nicht seine Kinder) ohne Übergang aus (`false`) oder ein (`true`); es gilt der letzte Keyframe
  mit Eintrag für das Teil und `t` ≤ Phase, vor dem ersten der letzte der Animation (zyklisch); ohne Eintrag ist das Teil sichtbar.
  Ein ausgeblendetes Teil fehlt im Frame und in den Teil-Boxen von `frame`. Gilt für `figur`, `requisite`, `gebaeude` und `hintergrund`.
  Bewegung der Rig-Typen (Posen, Gangarten mit IK, Rückengelenk, Umhang) ist Code im Rig, die Spec setzt nur Parameter.
- **`proportionen` (vierbeiner):** Beinlängen folgen den Pivots von `oberbein-*`, `unterbein-*` und `huf-*` (IK: Oberbein = Knie − Hüfte,
  Unterbein = Hüfthöhe − Oberbein, das Bein reicht also immer bis zum Boden). `fuss` setzt die Spitze des Beinpfads über der Fessel:
  `huf` 3,4 (Standard, wie RD1), `pfote` 2,2, `elefantenfuss` 1,2. `gangarten` (Standard alle drei) nennt die Gangarten des Tiers: jede braucht eine Animation
  mit dieser `gangart` (`gangart-fehlt`), keine Animation nutzt eine andere (`gangart-nicht-erlaubt`). `stand` (alle Hufe am Boden, Atmen; Reit-Idle) ist immer erlaubt und nie Pflicht. Ein Elefant ohne `galopp`
  setzt `gangarten: [schritt, trab]`; ein Pferd ohne `galopp`-Animation ist ungültig.
- **`proportionen` (figur):** Faktoren auf die Winkel der Posen `idle` und `lauf` (`arme`: oberarm-*/unterarm-*, `beine`:
  oberschenkel-*/unterschenkel-*, `kopf`: kopf/krone) und auf den senkrechten Versatz (`rumpf`). 1 = Grundform; kleiner bei längeren
  Gliedern (gleicher Ausschlag in Einheiten). Keyframe-Winkel der Spec bleiben unverändert.
- **`reittier`:** `id` eines `vierbeiner`-Assets; beim Reiten gilt der Rahmen des Reittiers.
- **`ebene`** (B-752): Reiter und Reittier getrennt rendern, der Client setzt sie nach `rahmen` zusammen (Reihenfolge hinten, Umhang + Reiter, vorn mit Zügeln). `reiter` am `figur` (braucht `reittier` und `gangart`): Umhang und Reiter; `hinten` und `vorn` am `vierbeiner` (brauchen `reiter` = id der Figur, deren Hand die Zügel hält, und `gangart`). Alle Ebenen teilen Koordinaten (Fußpunkt des Reittiers) und Pixel je Einheit, jede hat einen eigenen engen Rahmen mit Rand. Der Tausch der Spielerfarbe gilt nur für die Reiter-Ebene (Satteldecke ohne Spielerfarbe, 🧑 2026-11-08). Fehler: `ebene-angabe`.
- **Varianten** (B-711, entschieden 2026-11-14): Der `tausch` einer Variante gilt für alle Lagen der Spec, beim Reiten
  ohne `ebene` auch für das Reittier. `galopp@spieler2.png` des Monarchen zeigt die Satteldecke des Pferds also in der
  Farbe von `spieler2` (`tools/paper/rig/reiten.ts` › `reitBild`). Reittiere brauchen dafür keine eigenen Varianten. Die
  getrennten Ebenen (`ebene`, B-752) bleiben davon unberührt: Dort gilt der Tausch nur für die Reiter-Ebene.
- **`massstab`:** Faktor Spec-Einheiten → Welt-Einheiten (Standard 1). Das Pferd steht in eigenen Einheiten mit `1.3`
  (wie `scale(1.3)` im Prototyp); Pivots und Beinbreiten gelten in Spec-Einheiten.
- **`rahmen`:** Ausschnitt eines Frames in Spec-Einheiten; Frame-Breite = `round(hoehe_px · b / h)`.

### Lagen-Arten

| `art` | Felder | Zeichnung |
|---|---|---|
| `flaeche` | `form`, `material`, `ton?`, `auflage?` | Papierstück mit Schatten `papier.lage`; mit `auflage: true` aufgeklebte Lage mit Schatten `papier.auflage` |
| `falz` | `form`, `breite` (≤ 4), `material`, `ton?` | Linie ohne Füllung, Deckkraft `papier.falz_deckkraft`, runde Enden |
| `punkt` | `mitte:[x,y]`, `radius`, `material`, `ton?` | Kreis ohne Schatten (Tupfer, Pupille) |

`form` ist ein SVG-Pfad (`M L H V C S Q T Z`, absolut oder relativ, keine Bögen). `material` ist ein Material- oder
Einzelton-Name; `ton` fehlt = `basis`; Einzeltöne haben keinen Ton.

## 4. Rig-Typen, Pflicht-Teile und Standard-Pivots

| Rig | Pflicht-Teile | Standard-Pivots |
|---|---|---|
| `figur` | `rumpf`, `kopf`, `oberarm-v`, `unterarm-v`, `oberarm-h`, `unterarm-h`, `oberschenkel-v`, `unterschenkel-v`, `oberschenkel-h`, `unterschenkel-h` | Hüfte (0, −50) = `rumpf`, `oberschenkel-*`; Schulter (0, −84) = `oberarm-*`; Ellbogen (0, −70) = `unterarm-*`; Knie (0, −28) = `unterschenkel-*`; Hals (0, −90) = `kopf` |
| `vierbeiner` | `rumpf-vorn`, `rumpf-hinten`, `hals`, `schweif` | Hüfte (−26/−30 hinten, 26/30 vorn; −56) = `oberbein-*`; Knie −31 = `unterbein-*`; Fessel −9 = `huf-*`; Rückengelenk (0, −66) = `rumpf-hinten` |
| `gebaeude` | `korpus` | frei; Stufen-Zusätze hängen an Teilen der Basis |
| `requisite` | mindestens ein Teil | frei |
| `hintergrund` | mindestens ein Teil | frei |

Seiten-Kürzel: `-v`/`-h` = vorn/hinten (nah/fern am Betrachter bei der Figur), beim Vierbeiner `-vn`, `-vf`, `-hn`, `-hf`
(vorn/hinten, nah/fern). `-h`-Teile werden vor dem Rumpf gezeichnet.

## 5. Regeln (`check.ts`)

Fehler `{feld, regel, text}`; `feld` als Pfad wie `teile[2].lagen[0].material`. Prüfreihenfolge: `hex-farbe` → `schema`
→ fachliche Regeln (eine Stufe mit Fehlern beendet die Prüfung; fachliche Regeln werden alle gemeldet).

| Regel | Fehler |
|---|---|
| `json` | Datei ist kein gültiges JSON |
| `hex-farbe` | Hex-Farbe (`#` mit 3–8 Hex-Ziffern) in einem Wert außerhalb von `form` (Farben gehören in die Stil-Datei) |
| `schema` | Verstoß gegen das JSON-Schema (unbekanntes Feld, Typ, Wertebereich, `bein`/`stufen` am falschen Rig) |
| `material-unbekannt` | Material weder in `materialien` noch in `einzeltoene` (Lagen, Umhang, `tausch`) |
| `ton-ohne-toene` | `ton` außer `basis` an einem Einzelton |
| `pflicht-teil-fehlt` | Pflicht-Teil des Rig-Typs fehlt (bzw. kein Teil bei `requisite`, `hintergrund`) |
| `teil-doppelt` | zwei Teile mit gleicher `id` |
| `eltern-unbekannt` | `eltern` nennt kein Teil (Stufen-Zusätze sehen die Basis-Teile) |
| `teil-unbekannt` | `umhang.anker` oder ein Keyframe-Winkel nennt kein Teil |
| `zyklus` | ein Teil ist sein eigener Vorfahr |
| `id-dateiname` | `id` ≠ Dateiname ohne `.paper.json` |
| `proportion` | Figur: Höhe der Lagen von `kopf` / `rahmen[3]` außerhalb `kopf_anteil ± toleranz` |
| `material-doppelt` | Stil-Datei: Name zugleich Material und Einzelton |

## 6. Hash- und Golden-Regel (`hash.ts`)

- **Kanonische Form:** JSON mit nach Codepunkt sortierten Schlüsseln, ohne Leerraum; CRLF in Texten wird LF; `$schema`
  zählt nicht. Zeilenenden und Schlüsselreihenfolge der Datei ändern den Hash also nicht.
- `specHash` = SHA-256 (hex) der kanonischen Spec, `stilHash` ebenso für die Stil-Datei.
- `renderKey` = SHA-256 über `specHash:stilHash:rendererVersion`; `rendererVersion` steht in `hash.ts`
  (`RENDERER_VERSION`, v1 `paper-0+resvg-js-2.6.2`) und steigt mit jeder Änderung, die Pixel ändern kann.
  `renderKey` ist Cache-Schlüssel (`.cache/paper/<renderKey>/`) und Golden-Schlüssel; eine Stiländerung ändert den
  `renderKey` aller Assets, die die Stil-Datei nutzen.
- **SVG-Ausgabe:** Koordinaten mit höchstens 2 Nachkommastellen (`num()`), nie `-0`.
- **Determinismus:** kein Zufall in Spec oder Renderer; Seed der Papierstruktur aus der Asset-ID; Physik mit festem
  Zeitschritt 1/120 s und 240 Schritten Einschwingen vor Frame 0.
- **Golden-Regel:** Golden ist der SHA-256 jeder PNG-Datei (`golden.json` je Gruppe, Manifest je Asset). Ein Render ist
  gleich, wenn die Hashes **exakt** gleich sind. **Festgelegt in PV1.5 (2026-11-06): exakt.** Windows rendert
  zweimal byte-gleich (resvg-Probe, PV1.3), und der CI-Lauf auf Linux (`ubuntu-latest`, Run 42259675603) liefert dieselben
  SHA-256 wie `fixtures/render/hashes.json`. Rückfall, falls eine Plattform später abweicht (neues Ticket): gleich, wenn
  höchstens 0,1 % der Pixel eine Kanal-Differenz > 2 haben.
- Golden neu schreiben nur bei gewollter Änderung (neue `version` der Spec, Stil- oder Renderer-Änderung) mit Begründung im Commit.

`golden.json` = `{"<id>": {"renderKey": "…", "dateien": {"<datei>.png": "<sha256>"}}}`.

## 7. Ausgabe-Layout und Manifest

Ausgabe neben der Spec unter `<gruppe>/<id>/`:

| Datei | Inhalt |
|---|---|
| `<animation>.png` | alle Frames der Animation **horizontal** nebeneinander (Frame-Größe `frameW × frameH`) |
| `<animation>@<variante>.png` | dasselbe mit Material-Tausch der Variante |
| `<animation>.s<n>.png` | Gebäude: Stufe `n` ≥ 2 (Stufe 1 ohne Zusatz); mit Variante `<animation>@<variante>.s<n>.png` |
| `manifest.json` | Beschreibung der Ausgabe (unten) |

```json
{
  "id": "monarch", "version": 1,
  "specHash": "…", "stilHash": "…", "renderer": "paper-0+resvg-js-2.6.2", "renderKey": "…",
  "rahmen": [-60, -145, 85, 151], "frameW": 108, "frameH": 192,
  "animationen": { "idle": { "frames": 8, "fps": 8, "datei": "idle.png", "hashes": ["<sha256 je Frame>", "…"] } },
  "varianten": ["spieler2"]
}
```

`hashes` hat je Frame einen Eintrag (SHA-256 der RGBA-Pixel des Frames). Schlüssel in `animationen` sind die
Dateinamen ohne `.png` (also auch `idle@spieler2`, `stand.s2`). Die Fixture-Manifeste unter
`tools/paper/fixtures/render/<asset>/manifest.json` folgen dem Format, enthalten aber nur Frame 0 aus dem
Prototyp-Export (`frames: 1`, Hash der PNG-Datei).

## 8. CLI-Vertrag

Aufruf `bun tools/paper/bin/paper.ts <befehl> …` (Implementierung RD1, Attrappe für Tests: PV1.4). Antwort **ein**
JSON-Objekt auf stdout. Exit 0 = ok, 1 = fachlich abgelehnt (`{"ok": false, "fehler": [{feld, regel, text}]}`),
2 = Aufruf- oder Systemfehler (Text auf stderr, nichts auf stdout). Pfade in Antworten relativ zum Repo, mit `/`.

| Befehl | Antwort bei Exit 0 |
|---|---|
| `validate <spec>` | `{"ok": true, "fehler": []}` |
| `render <spec\|ordner> [--stil datei]` | `{"ok": true, "manifeste": ["…/manifest.json"], "dateien": ["…/idle.png", …]}`; ein Ordner rendert alle Specs darin (ohne `index.paper.json`) und schreibt den Gruppen-Index |
| `frame <spec> --animation a --frame n [--variante v] [--stufe s] [--ohne t1,t2] [--stil datei] --out datei.png` | `{"ok": true, "datei": "…", "teile": {"<teilId>": [x, y, b, h]}}` – Teil-Boxen in Pixeln des Frames; `--ohne` rendert die genannten Teile ohne Lagen (Gelenke bleiben, B-734), ein unbekanntes Teil lehnt mit `teil-unbekannt` ab |
| `diff <alt.json> <neu.json> [--stil-alt s1 --stil-neu s2]` | `{"ok": true, "felder": [{"pfad", "alt", "neu"}], "teile": ["<teilId>"], "stil": false}` |
| `pixeldiff <a.png> <b.png> --out overlay.png` | `{"ok": true, "anteil": 0.0123, "boxen": [[x, y, b, h]], "datei": "overlay.png"}` |

- `diff`: `pfad` im Format der Fehler-Felder; Teile werden über ihre `id` zugeordnet (Reihenfolge egal), `teile` nennt
  neue, gelöschte und geänderte Teile; `stil` ist `true`, wenn die beiden Stil-Dateien verschiedene `stilHash` haben.
- `pixeldiff`: `anteil` = Anteil der Pixel mit Kanal-Differenz > 2 (0..1); `boxen` = Rahmen zusammenhängender
  Abweichungen (höchstens 32, größte zuerst); das Overlay zeigt `b` mit rot markierten Abweichungen.
- `render` schreibt atomar: erst in einen Temp-Ordner, dann Umbenennen; bei Fehler bleibt der alte Stand.
- `--stil <datei>` (`render`, `frame`; B-705): rendert mit dieser Stil-Datei statt `public/grafik/stil.paper.json`
  (Standard ohne Option); Manifest-`stilHash` und `renderKey` folgen ihr. Fehlt die Datei → Exit 2 mit Pfad auf
  stderr; kein JSON oder Verstoß gegen `stil.schema.json` → Exit 1, `feld` mit Dateinamen (`stil.json:materialien`).
- `frame` bei Reit-Animationen (B-709): Teile des Reittiers stehen mit Präfix `reittier:` (`reittier:hals`), die des
  Reiters ohne Präfix. Auch Ober- und Unterbeine (vom Beinpfad gezeichnet, Box aus seinen Stützpunkten samt Breite) und
  Teile hinter dem Reiter haben eine Box; die verdeckte Bein-Kette des Reiters (`oberschenkel-h` …) wird nicht
  gezeichnet und hat keine Box. Die Bein-Box der Vorderbeine umfasst den Wulst (B-845).
- `frame --ohne` bei Reit-Animationen (B-845): `reittier:<teil>` blendet ein Teil des Reittiers aus, ids ohne Präfix
  die des Reiters. Ober- oder Unterbein nimmt den ganzen Beinpfad des Beins (ein Papierstück), beide Bein-Boxen fehlen.
  Ein unbekanntes `reittier:`-Teil lehnt mit `teil-unbekannt` und der Liste der Reittier-Teile ab, ebenso
  `reittier:<teil>` bei einer Animation ohne Reittier.
- `frame --ohne` bei Vierbeinern (B-851): `oberbein-<b>` oder `unterbein-<b>` nimmt auch hier den ganzen Beinpfad des Beins
  (Animationen eines Vierbeiners und seine Reittier-Ebenen); die anderen Beine und Teile bleiben.
- Fehler zu Reit-Angaben nennen den vollen Pfad: `animationen.<name>.reittier` bzw. `animationen.<name>.reiter` (B-708).

## 9. Fixtures (`tools/paper/fixtures/`)

| Pfad | Inhalt |
|---|---|
| `svg/*.svg` | Quell-SVGs aus dem Prototyp-Export (PV1.6), nur lesen |
| `specs/{monarch,pferd,mauer}.paper.json` | gültige Specs; erzeugt mit `bun tools/paper/fixtures/aus-prototyp.ts` |
| `specs/ungueltig/*.paper.json` | je ein Fehler: `hex-farbe`, `unbekanntes-feld` (Regel `schema`), `fremdes-material` (`material-unbekannt`), `pflicht-teil-fehlt`, `proportion` |
| `render/<asset>/*.png`, `render/hashes.json` | resvg-Render der Quell-SVGs (192 px hoch), Golden-Hashes unter Windows erzeugt |
| `render/<asset>/manifest.json` | Manifest-Beispiele (Abschnitt 7) |

Prüfen: `task test -- tools/paper` (Schema-, Hash- und Probe-Tests). resvg-Probe von Hand:
`bun tools/paper/probe/resvg-probe.ts [--write]`.
