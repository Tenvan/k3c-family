# Standard: Asset-MCP-Tools, Oberfläche (RPC) und Prompt-Vorlagen

Stand: 2026-11-06 (PV1.4) · Entscheidung [`005`](../decisions/005-papierschnitt-assets-mcp.md) › 4–7 · Domäne PAP ·
Spec, Stil-Datei, CLI und Ausgabe: [`paper-spec.md`](paper-spec.md) · Attrappen: `tools/paper/fixtures/`

Vertrag für MC1 (MCP-Tools), WB1 (Hülle der Workbench) und RV1 (Review). Änderungen nur über ein Ticket im Projekt PAP.
Assets werden **nur** über diese Tools geändert; die Oberfläche editiert nichts.

## 1. Gemeinsame Regeln

- Namen `assets_<verb>`; jede Beschreibung endet mit „Nutze es bei: … Statt: …“ (wie alle Tools der Workbench).
- **Jedes** Tool nimmt `checkout` (Ordnername oder absoluter Pfad des Worktrees, `git rev-parse --show-toplevel`); ohne
  Angabe gilt der Checkout aus dem Header, sonst die Repo-Wurzel. Schreibende Tools nennen in der letzten Zeile
  `Checkout: …`. Eigener Job-Mechanismus für Render-Jobs, **nicht** `task_*` (das bedient nur die Repo-Wurzel).
- Antworten sind verdichteter Text (Beispiele: `tools/paper/fixtures/mcp/*.txt`); Ablehnungen nennen je Fehler
  `feld · regel · text` (Regeln: `paper-spec.md` › Regeln) und ändern nichts.
- `id` ist die Asset-ID (= Dateiname ohne `.paper.json`), eindeutig über alle Gruppen.
- **`basis`** (wo angegeben): `HEAD` (Standard: Arbeitsstand gegen den letzten Commit), `HEAD~1`, Commit-SHA (7–40
  Hex-Zeichen) oder `origin/develop`. Bei `origin/develop` ohne Fetch in diesem Aufruf steht in der Antwort
  `Basis-Stand: lokal`. Ein anderer Wert wird mit den erlaubten Werten abgelehnt.
- Pfade in Antworten relativ zum Checkout, mit `/`.

## 2. MCP-Tools

| Tool | Eingabe | Antwort |
|---|---|---|
| `assets_guide` | `{rig?, checkout?}` | Text: Stil-Regeln, Papier-Materialien und Einzeltöne aus der Stil-Datei, Pflicht-Teile und Pivots des Rig-Typs (ohne `rig`: alle), Proportion, Lagen-Arten, Beispiel-Spec, Ablauf. Einstieg jeder Asset-Arbeit |
| `assets_list` | `{rig?, gruppe?, geaendert?, checkout?}` | Kopfzeile mit Anzahl, dann je Asset `id · rig · pfad · gültig\|ungültig[ · geändert]`; `geaendert: true` filtert auf Assets mit Änderung gegen `HEAD` |
| `assets_get` | `{id, felder?, checkout?}` | Pfad, dann die Spec als JSON; `felder` (Liste oberster Schlüssel) kürzt |
| `assets_create` | `{id, rig, beschreibung, gruppe, checkout?}` | legt `public/grafik/<gruppe>/<id>.paper.json` aus der Rig-Vorlage an (Pflicht-Teile mit leeren Lagen bzw. einem Platzhalter-Teil), Ergebnis von `validate`, Render-Ergebnis; Ablehnung bei vorhandener `id` (auch in einer anderen Gruppe und bei zwei gleichzeitigen Aufrufen) |
| `assets_set` | `{id, patch, checkout?}` | JSON-Merge-Patch (RFC 7396) auf die Spec; `teile` auch als Objekt nach Teil-ID (`{teile: {kopf: {…}}}`): Merge-Patch nur auf die genannten Teile, `null` entfernt ein Teil, Reihenfolge und übrige Teile bleiben, unbekannte ID → `teile.<id> · teil-unbekannt` mit den vorhandenen IDs (eine Liste ersetzt weiter alles). Ablauf atomar: Patch anwenden → `validate` → `render` in einen Temp-Ordner → Spec per Temp-Datei und Umbenennen ersetzen → Ausgaben umbenennen. Antwort: geänderte Felder (`pfad alt → neu`), Anzahl PNGs, Pixel-Anteil je Animation, Overlay `.cache/paper/mcp/set/<id>-<animation>-0-diff.png`; ist das Asset ein Reittier, `Reiter veraltet: <ids>` (neu rendern mit `assets_render`); Ablehnung: `feld · regel · text`, nichts geändert |
| `assets_style_get` | `{checkout?}` | Pfad, `stilHash`, Materialien, Einzeltöne, Papier-Werte (verdichtet) |
| `assets_style_set` | `{patch, checkout?}` | Merge-Patch auf die Stil-Datei, nur Sessions mit `Lane: stil`; prüft wie `assets_set`, startet den Render der betroffenen Assets als Job (nur Material- oder Einzelton-Änderung: die Assets, die sie nutzen, samt deren Reitern; sonst alle) und antwortet mit Änderungen, betroffenen Assets und `{job}` |
| `assets_render` | `{id, checkout?}` | rendert ein Asset synchron: Anzahl PNGs, Manifest, `renderKey` |
| `assets_render` | `{alle: true, checkout?}` | startet sofort einen Job: `Job <id> gestartet: n Assets` |
| `assets_render` | `{job, wait?, checkout?}` | `Job <id> läuft n/m (<asset>)`; mit `wait: true` blockierend bis zum Ende: je Asset Ergebnis, ungültige übersprungen |
| `assets_delete` | `{id, confirm, checkout?}` | ohne `confirm: true` nur Hinweis; sonst löscht Spec, Ausgabe-Ordner und zieht den Gruppen-Index nach |
| `assets_preview` | `{id, animation?, frame?, variante?, basis?, checkout?}` | MCP-ImageContent `image/png` (ein Frame) plus Text: Asset, Animation, Frame, Basis, Größe, Teil-Boxen `x,y,b,h` in Pixeln; mit `basis` der alte Stand |
| `assets_review` | `{basis?, checkout?}` | Kopfzeile `Review gegen <basis> (<sha>, Basis-Stand aktuell\|lokal): n Assets`, dann je Asset `status · id · Felder · Pixel-Anteil · Bilder`; Stiländerungen melden jedes betroffene Asset mit `stil: true`; Overlays `.cache/paper/mcp/<id>-<animation>-0-diff.png` |

- Standardwerte: `animation` = erste Animation der Spec, `frame` = 0, `variante` = keine.
- Ersetzt werden die alten Register-Tools `assets_list`, `assets_get`, `assets_set`, `assets_catalog` (Register
  `data/assets.json` ab MC1 eingefroren, Entfernung B-700). **Bleiben:** `assets_research`, `assets_hits`,
  `assets_hit_add`, `assets_hit_drop`, `assets_import`.
- Die MCP-Tools rufen die Render-CLI (`paper-spec.md` › CLI-Vertrag) über eine gemeinsame Anbindung mit dem Review
  (`internal/review` › `Aufruf`): Skript aus `K3C_PAPER_CLI`, sonst `tools/paper/bin/paper.ts`, sonst die Attrappe
  `tools/paper/fixtures/cli-stub.ts`; Zeitlimit 2 min je Aufruf; fehlt `bun`, nennt der Fehler `requirements.md`.
  Tests in-process gegen die Attrappe (`run(args)`).
- **Sammelaufruf (B-856):** `assets_set` (und `Schreibe`) startet bun **einmal**: `batch <schritte.json>` mit
  `[{args, stopp?, braucht?}]` führt `validate`, `diff`, je Animation `frame` alt, `frame` neu und `pixeldiff`, zuletzt
  `render` in einem Prozess aus (`tools/paper/cli/batch.ts`, auch in der Attrappe). Antwort
  `{ok: true, schritte: [{code, antwort} | {code: 2, stderr} | {uebersprungen: true}]}`; jeder Schritt wird wie ein
  Einzelaufruf gedeutet (`review.Auswerten`). `stopp` (bei `validate`, `diff`, `render`) überspringt nach einem Fehler
  den Rest, `braucht` überspringt `frame` neu und `pixeldiff`, wenn ein früherer Schritt scheiterte. Das Zeitlimit gilt
  für den ganzen Sammelaufruf. Die Antwort von `assets_set` bleibt unverändert.
- `assets_list` cacht `validate` je Spec, solange Spec, Stil-Datei und die Specs ihrer Reittiere unverändert sind.
- Temp-Ordner und beiseitegelegte Ausgaben liegen unter `.cache/paper/tmp/`, nie neben der Ausgabe.

### MCP-Prompts

Jede Vorlage aus `data/asset-prompts.json` wird als MCP-Prompt `asset-<id>` angeboten. Argumente = die Platzhalter, die
im Text vorkommen (alle Pflicht, außer `zustand` und `basis`, die leer bleiben dürfen); der Prompt liefert den Text mit
eingesetzten Werten als eine User-Nachricht.

## 3. Prompt-Vorlagen (`data/asset-prompts.json`, Schema `tools/paper/schema/prompts.schema.json`)

`[{id, titel, ort, text}]`:

| Feld | Regel |
|---|---|
| `id` | `^[a-z][a-z0-9-]*$`, eindeutig |
| `titel` | Text des Prompt-Buttons, höchstens 40 Zeichen |
| `ort` | `asset` (am Asset im Katalog), `teil` (am angeklickten Teil der Vorschau), `review` (am Review-Eintrag); Styleguide (B-727): `material` (gewähltes Material), `ton` (gewählter Einzelton), `papier`; Sound (B-728): `sound` (gewählter Slot) |
| `text` | Orte `asset`, `teil`, `review`: beginnt mit „Rufe zuerst `assets_guide {rig: {rig}}` auf“, endet mit „Prüfe das Ergebnis mit `assets_preview` und rufe zum Schluss `assets_review` auf.“ Alle anderen Orte: kurzer Prompt in einer Zeile ohne Arbeitsanweisungen (B-639), höchstens 120 Zeichen |

Platzhalter (nur diese): `{asset}` ID, `{rig}` Rig-Typ, `{teil}` angeklicktes Teil (Pflicht bei `ort: teil`),
`{pfad}` Spec-Pfad, `{zustand}` kurzer Zustand (z. B. „gültig, Version 1“ oder die Feldänderungen des Reviews), `{basis}`
Review-Basis, `{eingabe}` Freitext aus einem Eingabefeld des Buttons (Pflicht bei `ort: review`). Die Oberfläche
setzt die Werte ein und kopiert den fertigen Prompt in die Zwischenablage. Vorlagen v1: `material-aendern`,
`proportion-anpassen`, `detail-ergaenzen`, `variante-anlegen`, `animation-aendern`, `neues-wie-dieses` (asset);
`teil-material`, `teil-form` (teil); `aenderung-zuruecknehmen`, `nachbessern` (review).

Styleguide (B-727): `{material}` Material-Name (Pflicht bei `ort: material`), `{ton}` Name des Einzeltons (Pflicht bei
`ort: ton`). Vorlagen je Ort „ändern“, „neu anlegen“, „Nutzer zeigen“: `stil-material-aendern`, `stil-material-neu`,
`stil-material-nutzer`, `stil-ton-aendern`, `stil-ton-neu`, `stil-ton-nutzer`, `stil-papier-aendern`, `stil-papier-neu`,
`stil-papier-nutzer`.

Sound (B-728): `{slot}` Slot-ID aus `data/sounds.json`, `{datei}` zugeordnete Dateien (kommagetrennt, sonst „keine
Datei“), beide Pflicht bei `ort: sound`. Vorlagen: `sound-suchen`, `sound-vergleichen`, `sound-lautstaerke`.

Die MCP-Prompts `asset-<id>` kennen dieselben Platzhalter.

## 4. Oberfläche (RPC)

Die Oberfläche der Workbench spricht **kein REST**: Wails-Bindings an `App` bzw. `POST /api/dev/rpc/{Method}` mit der
Allowlist `uiMethods` (`tools/k3c-workbench/uiapi.go`), gleich gehalten mit `frontend/src/api/wails.ts`
(`TestUIMethodsWieWailsTS`). Bilder als Data-URL (`data:image/png;base64,…`). Besitzer: RV1; Stubs mit den
Beispielantworten `tools/paper/fixtures/ui/<Methode>.json`: PV1.8.

| Methode | Antwort |
|---|---|
| `AssetsList() []AssetEintrag` | `[{id, rig, gruppe, pfad, gueltig, geaendert, fehler?}]`; `geaendert` gegen `HEAD`, `fehler` wie bei `validate` |
| `AssetsIndex() []AssetEintrag` | wie `AssetsList`, aber ohne Prüfung (B-732): Einträge ohne gemerktes Prüfergebnis tragen `offen: true` und gelten bis dahin als gültig |
| `AssetsCheck(pfade []string) []AssetEintrag` | prüft die Specs unter `pfade` (aus `AssetsIndex`) und liefert ihre Einträge wie `AssetsList`, nur ohne `geaendert` (immer `false`, steht im Index); ein Pfad außerhalb des Arbeitsstands wird nicht geprüft und meldet `fehlt` |
| `AssetsGet(id string) Spec` | Spec als Objekt (ohne `$schema`) |
| `AssetsStyle() Stil` | Stil-Datei als Objekt (ohne `$schema`) |
| `AssetsPrompts() []Vorlage` | Inhalt von `data/asset-prompts.json` |
| `AssetsReview(basis string) Review` | `{basis, basisCommit, basisStand: aktuell\|lokal, assets: [{id, rig, pfad, status: neu\|geaendert\|geloescht\|ungueltig, fehler?, felder: [{pfad, alt, neu}], teile: [id], stil, pixelAnteil}]}` |
| `AssetsFrame(id, animation string, frame int, variante, basis string) Frame` | `{bild, teile: {teilId: [x, y, b, h]}}`; `variante` und `basis` dürfen leer sein (leere Basis = Arbeitsstand) |
| `AssetsPixelDiff(id, animation string, frame int, basis string) PixelDiff` | `{anteil, boxen: [[x, y, b, h]], overlay}` – Arbeitsstand gegen `basis` |
| `AssetsFrameOhne(id, animation string, frame int, ohne []string) Frame` | wie `AssetsFrame` (Arbeitsstand, ohne Variante), die Teile in `ohne` ausgeblendet (`paper frame --ohne`, B-734) |
| `AssetsGroesse(id string) Groesse` | `{pxProEinheit, quelle}`: Spiel-Pixel je Einheit von `ausgabe.rahmen` – Figur mit Spielrolle aus `data/paper.json` + `data/sprites.json` (`height · scale`), Figur ohne Rolle mit dem Maßstab des Monarchen, sonst 1 (B-733) |
| `AssetsHintergruende() []string` | Biome mit Hintergrund, Namensteil `hg-<teil>-…` unter `public/grafik/hintergruende/` (B-733) |
| `AssetsHintergrund(teil string) Hintergrund` | `{fern, mitte, nah}`: die Ebenen der Variante 1 als Data-URLs (je 768 × 480 Spiel-Pixel, Fuß unten) |

- `status`: `neu` (fehlt in der Basis), `geaendert` (Spec, Stil oder Ausgabe anders), `geloescht` (fehlt im Arbeitsstand),
  `ungueltig` (Arbeitsstand verletzt eine Regel; `fehler` gesetzt, `felder` gegen die Basis). `stil: true`, wenn sich
  der `stilHash` gegenüber der Basis geändert hat (dann `status: geaendert` auch ohne Feldänderung).
- `pixelAnteil`: `pixeldiff`-Anteil des ersten Frames der ersten Animation (0..1); `neu` und `geloescht` = 1,
  `ungueltig` = 0.
- „Vorher“ rendert `git show <basis>:<pfad>` (Spec und Stil-Datei der Basis) mit demselben Renderer in den Cache
  `.cache/paper/<renderKey>/`; ist der Cache-Eintrag da, wird nicht neu gerendert.
- **Fehler:** ungültige Basis → Fehlertext `Basis „x“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop`;
  unbekannte `id` → `Asset „x“ fehlt`; unbekannte Animation → Text mit den vorhandenen Animationen.

## 5. Attrappen und Beispiele (`tools/paper/fixtures/`)

| Pfad | Inhalt |
|---|---|
| `cli-stub.ts` | Attrappe der CLI: `run(args) → {code, stdout, stderr}`, auch als `bun tools/paper/fixtures/cli-stub.ts <befehl> …`. `validate` prüft echt (`check.ts`); `render`/`frame` liefern für `monarch`, `pferd`, `mauer` die PNGs aus `render/`, für gültige unbekannte Specs `render/platzhalter.png`; `diff` und `pixeldiff` rechnen echt (Pixel über resvg gelesen). Zusätzliche fachliche Regel der CLI: `animation-unbekannt` |
| `ui/<Methode>.json` | Antwort je RPC-Methode (Stand des Review-Test-Repos, Basis `HEAD~1`); erzeugt mit `bun tools/paper/fixtures/beispiele.ts` |
| `mcp/<tool>[-fall].txt` | Beispielantwort je MCP-Tool (u. a. `assets_set-abgelehnt`, `assets_render-job`, `-laeuft`, `-wait`, `assets_review-lokal`) |
| `review-repo/erzeuge.ts` | erzeugt ein Test-Git-Repo: Commit 1 Basis (Stil, monarch, mauer, fass, banner), Commit 2 (pferd neu, fass gelöscht, Stil `stein` heller), Arbeitsstand (Umhang purpur, banner mit Material `scharlach`). Basis `HEAD` → `geaendert` + `ungueltig`; Basis `HEAD~1` → alle Fälle. Feste Autor- und Commit-Zeit, daher ohne Commit-Signatur reproduzierbar (`HEAD~1` = `32eb7a7`; `beispiele.ts` übernimmt den echten Wert) |
| `render/monarch/pixeldiff-umhang.png`, `render/mauer/mauer-1-stil.png` | Overlay „Umhang purpur“ und Mauer nach der Stiländerung (für RV1) |

Prüfen: `task test -- paper` (`prompts.test.ts`, `stub.test.ts`, dazu die Tests aus PV1.3).
