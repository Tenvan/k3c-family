# 005 · Papierschnitt-Grafik aus Specs, Assets nur über MCP, Review in der Workbench

Stand: 2026-11-06 · Status: **beschlossen (🧑, 2026-11-06, Chat)** · Projekt: PAP · Ticket: B-699 · Prototyp: [`005/prototyp-monarch-rig.html`](005/prototyp-monarch-rig.html)
· Löst ab: Pixel-Pipeline (DB32, `tools/buildings`), Studio als eigenes Programm aus 004 (die Datei-Schnittstelle aus 004 bleibt)

## Kontext

- Der Prototyp (Monarch mit Pferd, Cutout-Rig in SVG) hat gezeigt: Claude kann Figuren als Vektor-Teile beschreiben,
  mit IK-Gangarten (Schritt, Trab, Galopp), Rückengelenk und Umhang-Physik animieren und im **Papierschnitt-Look**
  (keine Konturen, Lagen mit schmalem Schlagschatten, Papierstruktur) deterministisch als PNG ausgeben.
- Heute: Gebäude entstehen aus Daten-JSON als DB32-Pixel (`tools/buildings`), alle anderen Grafiken sind gekaufte
  Pixel-Assets. Das Studio (`tools/k3c-studio`) ist ein eigenes Programm mit Edit-Seiten und Chat; 🧑 findet das Editieren
  dort unübersichtlich.
- 🧑 hat am 2026-11-06 entschieden:
  - **Alle** Grafiken bekommen die Papierschnitt-Optik.
  - Jede Grafik ist durch eine Spec-Datei so festgelegt, dass eine KI sie jederzeit neu erzeugen kann, ohne dass sie
    anders aussieht.
  - Kein Bildmodell: Die KI (Claude) schreibt die Vektor-Spec, ein deterministischer Renderer erzeugt den Look
    („so wie beim Monarchen“). Auf dem Entwicklungsrechner gibt es keine NVIDIA-GPU.
  - Sprites zuerst für den PC-Browser; Skalierung für Xbox und Mobile wird am Ende entschieden.
  - Das Studio zieht als Bereich **Assets** in die Workbench, der Rest heißt **Development**.
  - Erstellen und Ändern von Assets nur über MCP-Tools; der Chat entfällt. In der Workbench wird nichts editiert,
    sie dient dem **Review** mit Markierung der Änderungen im Git-Diff und bietet **Prompt-Buttons** für Änderungen.
  - Der Plan wird so geschnitten, dass **parallele Agenten** ihn abarbeiten können.

## Optionen

| Option | Bewertung |
|---|---|
| **A · JSON-Spec + Rig-Typen in TS, Render mit resvg** | Daten statt Code, zentraler Stil, feldgenaue Diffs, reproduzierbar, gleiche Logik für Renderer und Vorschau |
| B · Spec als TS-Modul je Asset (wie der Prototyp) | schnell am Anfang, Stil driftet je Asset, Review nur als Code-Diff |
| C · SVG-Datei + Animations-JSON | einfach, aber Pfad-Diffs unlesbar, Filter in jeder Datei, wenig geteilte Logik |
| D · Bildmodell (lokal oder Cloud) für Texturen | reicher Look, aber nicht reproduzierbar, GPU oder Kosten nötig; verworfen |

## Entscheidung

Option **A**.

### 1. Pipeline

`<asset>.paper.json` → Rig-Bibliothek (TS unter bun, `tools/paper/`) → SVG → **resvg** (feste Version) → PNG-Frames + `manifest.json`.
Gleiche Spec ergibt gleiche PNGs (Golden-Regel in `docs/standards/paper-spec.md`). PNGs, Manifeste und je Gruppe ein
`index.paper.json` werden committet und vom Spiel geladen; der Server beobachtet `public/` weiter (004) und ignoriert `*.paper.json`.
Cache- und Golden-Schlüssel `renderKey` = Hash aus Spec, Stil-Datei und Renderer-Version.

### 2. Styleguide `public/grafik/stil.paper.json` (einziger Ort für Farben und Effekte)

- **Materialien** mit genau drei Tönen `licht`, `basis`, `schatten` (z. B. `koenigsblau`, `koenigsrot`, `hermelin`,
  `fell-fuchs`, `stein-grau`).
- **Papier:** Lagenschatten (dx 0.5, dy 0.9, Unschärfe 0.45, Deckkraft 0.55), Schatten aufgeklebter Lagen
  (0.3 / 0.5 / 0.3 / 0.35), Struktur (Frequenz 1.3, Stärke 0.18), Falz-Deckkraft ≤ 0.25, Licht von rechts oben.
- **Proportionsraster je Rig-Typ** (Figur: Kopf ≈ 1/3,5 der Höhe, Krone ≈ halbe Kopfhöhe; Vierbeiner: Rumpf ≈ zwei
  Reiterkopflängen hoch).
- **Skalierung:** globaler Faktor je Plattform (vorerst nur `pc = 1`).
- Assets enthalten **keine** Hex-Farben und keine Filterwerte, nur Material-Namen.

### 3. Asset-Spec `<asset>.paper.json`

Beispiel gekürzt (`…` steht für ausgelassene Pfaddaten):

```json
{
  "id": "monarch", "rig": "figur", "version": 1,
  "beschreibung": "König, bärtig, Hermelinkragen, Krone mit Lilienspitzen",
  "teile": [
    { "id": "rumpf", "pivot": [0, -50], "lagen": [
      { "form": "M-12 -91C…Z", "material": "koenigsblau" },
      { "form": "M-12 -91C…Z", "material": "koenigsblau", "ton": "schatten" } ] },
    { "id": "kopf", "eltern": "rumpf", "pivot": [0, -90], "lagen": [] }
  ],
  "umhang": { "anker": "rumpf", "punkt": [-4, -91], "material": "koenigsrot", "laenge": 53, "breite": [4, 17] },
  "animationen": { "idle": { "frames": 8 }, "lauf": { "frames": 8 },
                   "reiten": { "reittier": "pferd", "gangart": "galopp", "frames": 8 } },
  "ausgabe": { "hoehe_px": 192 },
  "varianten": { "spieler2": { "tausch": { "koenigsblau": "smaragd" } } }
}
```

- **`beschreibung`** ist die Absicht in Worten, **`teile`** der verbindliche Zustand. Eine KI ändert gezielte Felder.
- **Rig-Typen** `figur`, `vierbeiner`, `gebaeude`, `requisite`, `hintergrund` geben Pflicht-Teile und Pivots vor;
  Bewegung (Posen, Gangarten mit IK, Rückengelenk, Umhang-Physik) ist Code im Rig-Typ, die Spec setzt nur Parameter.
  Eigene Posen nur als Winkel-Keyframes je Teil.
- **Gebäude-Stufen** als Delta wie heute (`stufen: [{ tausch, zusaetze }]`).
- **Determinismus:** kein Zufall in der Spec; Papierstruktur-Seed aus der Asset-ID; Physik mit festem Zeitschritt und
  Einschwingzeit.
- `paper validate` lehnt unbekannte Felder, fremde Materialien, fehlende Pflicht-Teile und Proportionsverstöße ab.
- Nicht enthalten (YAGNI): Vererbung zwischen Specs, Filter je Asset, Rastertexturen.

### 4. MCP-Tools (k3c-workbench, jedes mit `checkout`; Vertrag `docs/standards/paper-mcp.md`)

| Tool | Zweck |
|---|---|
| `assets_guide {rig?}` | Anleitung: Styleguide, Materialien, Pflicht-Teile, Proportionen, Beispiel-Spec; Einstieg jeder Asset-Arbeit |
| `assets_list {rig?, ordner?, geaendert?}`, `assets_get {id, felder?}` | lesen |
| `assets_create {id, rig, beschreibung, ordner}` | neue Spec aus der Rig-Vorlage |
| `assets_set {id, patch}` | JSON-Merge-Patch; atomar: validieren, rendern, erst dann schreiben |
| `assets_style_get`, `assets_style_set {patch}` | Styleguide |
| `assets_render {id \| alle \| job}` | rendern; `alle` und Stil-Render als Job mit Abfrage |
| `assets_preview {id, animation?, frame?, variante?}` | Bild als MCP-Image, damit der Agent sein Ergebnis selbst prüft |
| `assets_review {basis?}` | geänderte Assets gegen `HEAD` / Commit / `origin/develop` mit Feldern und Pixel-Anteil |
| `assets_delete {id, confirm}` | löschen |

- **Prompt-Vorlagen** in `data/asset-prompts.json` (z. B. „Material ändern“, „Proportion anpassen“, „Detail ergänzen“,
  „Variante anlegen“, „Animation ändern“, „Neues Asset wie dieses“, „Änderung zurücknehmen“, „Nachbessern …“), zugleich
  als **MCP-Prompts** angeboten.
- Sound- und Quellen-Tools (`assets_research`, `assets_hits`, `assets_import` …) bleiben unverändert.
- Der Studio-Chat und seine API entfallen.

### 5. Workbench

- Ein Programm (Wails und Web) mit den Bereichen **Development** (alle heutigen Seiten) und **Assets**:
  Review (Startseite), Katalog, Styleguide, Sound, Quellen. **Keine Edit-Controls.**
- **Prompt-Buttons** am Asset, am angeklickten Teil (Klick in die Vorschau wählt das Teil) und am Review-Eintrag:
  kopieren den fertigen Prompt (Asset-ID, Teil, Zustand, Verweis auf `assets_guide`) in die Zwischenablage.
- Aus dem Studio bleibt nur Lesendes, u. a. „Im Spiel ansehen“ mit Testspielstand (Engine als Bibliothek im
  Workbench-Backend). Edit-Seiten und Planer-UI entfallen.
- Entfällt: `tools/k3c-studio/`, EXE `k3c-studio` und `k3c-studio-web`, Ports 8091 und 8182, Studio-Tasks.

### 6. Review

- Basis wählbar: `HEAD` (uncommittet), letzter Commit, `origin/develop` (ganzer Sprint).
- Liste geänderter Assets (`neu`, `geändert`, `gelöscht`) mit Pixel-Anteil und Feldänderungen in Klartext.
- Vorher/Nachher nebeneinander, Überblendregler, **Pixel-Diff-Overlay** mit Umrandung, **Teil-Markierung** der
  geänderten Teile, Animationen synchron mit Frame-Schieber. Abzeichen im Katalog für Assets im aktuellen Diff.
- „Vorher“ rendert `git show <basis>:<pfad>` mit demselben Renderer in den Cache `.cache/paper/<renderKey>/`; Stiländerungen
  erscheinen als geänderte Assets.
- Die Oberfläche bekommt die Daten als **RPC-Methoden** (`AssetsReview`, `AssetsFrame`, `AssetsPixelDiff`, `AssetsList`, `AssetsGet`,
  `AssetsStyle`, `AssetsPrompts`) über Wails-Bindings bzw. `/api/dev/rpc/…`, Bilder als Data-URL; kein REST.

### 7. Fehler

- `assets_set` ist atomar; Antworten nennen Feld und Regel.
- Ungültige Spec auf der Platte: roter Eintrag in Assets, die übrigen Assets rendern weiter.
- Spielserver behält bei ungültigem Stand den alten (wie bei `data/`, 004).

## Umsetzung: Wellen und Lanes für parallele Agenten

Grundsatz **Contract-first**: Welle 0 legt die Verträge fest, danach arbeiten die Lanes in getrennten Ordnern gegen
Fixtures. Ein Sprint je Lane (`docs/arbeitsweise.md`); parallele Sessions derselben Domäne tragen verschiedene `Lane`
(Sperre je Domäne und Lane, PV1.1/PV1.2). Kein Agent fasst Dateien einer anderen Lane an; Vertragsänderungen nur über ein Ticket an Welle 0.

| Welle | Lane | Inhalt | Ordner | parallel zu |
|---|---|---|---|---|
| 0 | **PV1 · Verträge** | JSON-Schema `paper-spec` v1 und `stil.paper.json`, Render-CLI-Vertrag (`paper render` → PNGs + `manifest.json`), MCP-Vertrag (Tool-Signaturen, Antworten), Review-API `/api/assets/review`, Fixtures (Beispiel-Specs mit vorgerenderten PNGs), Glossar-Begriffe | `docs/standards/paper-spec.md`, `tools/paper/schema/`, `tools/paper/fixtures/` | – |
| 1 | **RD1 · Renderer** | Rig-Typen, IK, Gangarten, Rückengelenk, Physik, resvg-Export, `paper validate`, Golden-Tests, Task `gen:paper` | `tools/paper/` | M, W, A |
| 1 | **MC1 · MCP** | Asset-Tools aus Abschnitt 4, MCP-Prompts, Abbau der alten Grafik-Tools und des Chats | `tools/k3c-workbench/internal/mcpsrv/tools_assets*.go`, `tools/k3c-workbench/internal/paperassets/` (neu) | R, W, A |
| 1 | **WB1 · Workbench-Hülle** | Bereiche Development/Assets, Katalog, Styleguide-Seite, Prompt-Buttons, Übernahme lesender Studio-Teile, Abbau `tools/k3c-studio` | `tools/k3c-workbench/frontend/` (ohne `assets/review/`), `tools/k3c-workbench/internal/studioview/` (neu), `tools/k3c-workbench/app_studio.go`, `tools/k3c-studio/` | R, M, A |
| 1 | **RV1 · Review** | Review-Seite und Review-API, Pixel-Diff, Teil-Markierung, Render-Cache | `tools/k3c-workbench/frontend/src/assets/review/`, `tools/k3c-workbench/internal/review/` (neu) | R, M, W |
| 2 | **MG1…MG8 · Migration** | ein Agent je Asset-Gruppe: Figuren, Reittiere, Truppen, Gegner, Bosse, Gebäude, Hintergründe, Icons/Items; nur Specs über MCP | je Gruppe ein Ordner unter `public/grafik/` | untereinander |
| 3 | **US1 · Spiel-Umstellung** | Loader auf Gruppen-Indizes, Engine-Schatten, Server-Check und Watch, Tasks, Doku | `src/`, `cmd/k3c-server/`, `engine/watch/`, Doku | – |
| 4 | **PS1 · Sichtprüfungen** | alle manuellen Prüfungen und die gesammelten Fragen (B-701) gebündelt | – | – |
| 5 | **CU1 · Cleanup** | alte Pixel-Grafik, Register, `tools/buildings` entfernen (B-700) | `public/`, `data/`, `tools/buildings/` | – |

Modell-Empfehlung: RD1 und RV1 Opus (Rig-Mathematik, Bild-Diff), MC1 und WB1 Sonnet, Migration Sonnet mit `assets_preview`
zur Selbstkontrolle.

## Kriterien

- **AC-01** `paper validate` lehnt eine Spec mit Hex-Farbe, unbekanntem Feld, fremdem Material oder fehlendem
  Pflicht-Teil mit Feld und Regel ab.
- **AC-02** Gleiche Spec ergibt auf zwei Läufen und in der CI byte-gleiche PNGs (Golden-Test).
- **AC-03** (Lane RD1, Monarch und Pferd sind ihre Referenz-Fixtures) Der Monarch aus dem Prototyp ist als Spec abgebildet und rendert Idle, Lauf, Schritt, Trab und Galopp
  mit IK-Hufen, Rückengelenk und Umhang-Physik im Papierschnitt-Look.
- **AC-04** Eine Materialänderung im Styleguide ändert alle betroffenen Assets, ohne eine Asset-Spec anzufassen.
- **AC-05** Ein Agent kann ein neues Asset nur mit `assets_guide`, `assets_create`, `assets_set` und
  `assets_preview` anlegen und sieht dabei das gerenderte Bild.
- **AC-06** `assets_set` mit ungültigem Patch lässt die Datei unverändert.
- **AC-07** Die Workbench zeigt die Bereiche Development und Assets; Assets enthält keine Edit-Controls.
- **AC-08** Die Review-Seite zeigt für eine geänderte Spec Vorher/Nachher, Pixel-Diff-Overlay und markiert die
  geänderten Teile; Basis `HEAD`, letzter Commit und `origin/develop` sind wählbar.
- **AC-09** Prompt-Buttons kopieren einen vollständigen Prompt mit Asset-ID und Teil; dieselben Vorlagen sind als
  MCP-Prompts verfügbar.
- **AC-10** `tools/k3c-studio`, seine EXEs, Ports und Tasks sowie der Chat sind entfernt; `task buildall` und
  `task check:all` laufen grün.
- **AC-11** (manuell, 🧑, in PS1) Review einer echten Änderung am Monarchen in Assets ist ohne Öffnen von Dateien möglich.

## Folgen

- `docs/arbeitsweise.md`, `CLAUDE.md` (Struktur, Befehle) und die Codemaps werden in Welle 0 bzw. 3 angepasst;
  neue Begriffe (Papierschnitt, Paper-Spec, Rig-Typ, Lage, Material, Ton) kommen zuerst ins Glossar.
- Gekaufte Pixel-Assets und `CREDITS.md`-Einträge entfallen mit der Migration; Sound bleibt.
- Offen für spätere Specs: Skalierung für Xbox und Mobile, Engine-Schatten-Technik in Phaser 4, Laufzeit-Kosten
  großer Spritesheets.
