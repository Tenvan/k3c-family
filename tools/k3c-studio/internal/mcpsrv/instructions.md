# k3c-studio

Werkzeug für Game- und Level-Designer von K3C: Welten, Inseln, Chunks, Hub (Gebäude-Planer) und Assets (Papierschnitt).
Diese Tools ersetzen Shell-Befehle und Dateilesen, weil sie nur das Wichtige zurückgeben. Entwicklung, Planung, Prüfläufe,
Dienste und Balancing liegen im MCP-Server `k3c-workbench`; dieser Server kennt nur die Tools unten.

## Regeln für alle Tools

- Namen `<bereich>_<verb>`; jede Beschreibung endet mit „Nutze es bei: … Statt: …“.
- Antworten sind verdichteter Text; große Ausgaben über `limit` und Cursor.
- Löschen verlangt `confirm=true`; abgelehnte Aufrufe nennen den gültigen Wertebereich und ändern nichts.
- Geschrieben wird nur über diese Tools, nie von Hand: sie prüfen wie das Spiel und schreiben atomar und format-stabil.

### Worktrees

- Eine k3c-studio-Instanz (aus der Repo-Wurzel) bedient alle Worktrees. Jeder Aufruf gilt dem Checkout, aus dem die
  Session kommt (Header `X-K3C-Root` aus `.mcp.json`).
- **Im Worktree immer `checkout` setzen** (B-388): alle `world_*`, `assets_*` und `hub_*` (auch die lesenden) nehmen das
  Argument `checkout` mit dem Ordnernamen oder dem absoluten Pfad des eigenen Checkouts (`git rev-parse --show-toplevel`).
  Es gilt vor dem Header; der Header allein genügt nicht, die Desktop-App meldet aus Worktrees die Repo-Wurzel (B-341).
  Ein unbekannter oder mehrdeutiger Wert wird abgelehnt, die Meldung nennt die Worktrees.
- Ohne `checkout` und ohne Header gilt die Repo-Wurzel. Schreibende Tools lehnen nicht ab, nennen aber immer den
  Checkout in der letzten Zeile `Checkout:`. Steht dort aus einem Worktree die Repo-Wurzel, die Änderung in der Wurzel
  zurücknehmen und mit `checkout` wiederholen.

## Weltdesign

### Assets (Papierschnitt)

Assets sind Paper-Specs `public/grafik/<gruppe>/<id>.paper.json` (Vertrag `docs/standards/paper-mcp.md`); geändert wird
**nur** über diese Tools, jedes nimmt `checkout`. Neue Tools gelten erst nach Merge und Neustart des Studios aus der
Repo-Wurzel (B-701 D-10).

- Einstieg `assets_guide {rig?}`: Stil-Regeln, Materialien, Pflicht-Teile, Pivots, Vorlage, Ablauf.
- Lesen: `assets_list {rig?, gruppe?, geaendert?}`, `assets_get {id, felder?}`, `assets_style_get`.
- Ändern: `assets_set {id, patch}` (JSON-Merge-Patch, atomar; abgelehnt = nichts geändert, je Fehler `feld · regel · text`),
  `assets_create {id, rig, beschreibung, gruppe}`, `assets_delete {id, confirm}`, `assets_style_set {patch}` (nur Lane `stil`).
- Ablauf: `assets_get` → `assets_set` → `assets_preview {id, animation?, frame?, variante?, basis?}` (Bild `image/png` plus
  Teil-Boxen) → `assets_review {basis?}` (Basis `HEAD`, `HEAD~1`, Commit-SHA oder `origin/develop`).
- Rendern: `assets_render {id}` synchron, `{alle: true}` startet einen Job (`j-<n>`), `{job}` Stand, `{job, wait: true}`
  wartet bis zum Ende (eigener Mechanismus, nicht `task_*`).
- Prompts: jede Vorlage aus `data/asset-prompts.json` ist der MCP-Prompt `asset-<id>`, Argumente = Platzhalter.

### Gebäude-Planer (B-923)

`data/hub.json` (Hub-Stufen, Bauplätze, Händler, Baugrund) samt Block der Regeltabelle `docs/rules/materialien-gebaeude.md` § 3;
jedes Tool nimmt `checkout`. Keine Grafik: Assets ändern nur die `assets_*`-Tools.

- `hub_get`: Matrix Hub-Stufe × Bauplatz, Befunde der Prüfung (`sim.ValidateHubData`), Abweichung der Regeltabelle, Zeile `Stand`.
- `hub_check {patch | candidate}`: prüft eine Änderung ohne zu schreiben; `patch` ist ein JSON-Merge-Patch, Listen ganz angeben.
- `hub_save {patch | candidate, stand}`: schreibt `data/hub.json` format-stabil (ein Wert, eine Zeile) und die Regeltabelle,
  nur ohne Befund und solange die Datei dem `stand` aus `hub_get` entspricht. REG-Änderung: Commit mit Begründung.

### Welten (B-954)

`data/worlds/<id>.json` des Checkouts (docs/rules/welten.md); jedes Tool nimmt `checkout`. Geschrieben wird atomar und
format-stabil (nur geänderte Werte, ein Wert eine Zeile), vorher prüft die Engine wie das Laden im Spiel (`sim.ParseWorld`,
Werte aller Nachfolger); bei Fehler bleibt die Datei. Keine Simulation. Das Biom folgt der Tiefe (`biomes` im Patch = Kurzform von `depths`).

- `world_list`, `world_get {id}`: Welten mit Inseln, Tiefen → Biome, geplante Chunks, Geschichte (fehlende Einträge als
  „fehlt“) und Text der Datei.
- `world_create {id, name, extends?, story?}`: neue Welt mit einer Insel `island1` (Tiefen 0-4); überschreibt nie. `story`
  wie die Felder von `world_story_set`; ohne sie endet die Antwort mit „Geschichte fehlt: world_story_set“.
- `world_delete {id, confirm}`: löscht die Datei einer Welt endgültig; abgelehnt werden feste Welten und Welten mit Erben (`extends`), Spielstände bleiben (Warnung).
- `world_set {id, patch}`: `name`, `notes`, `extends`, `combat {mode, groupSize}` (null entfernt) und `islands` (Liste; bekannte `id` ändert
  nur genannte Felder, neue `id` wird angehängt; `depths` oder `biomes`, `depthScaling` wird gemischt). Jede Insel außer der letzten braucht
  in Tiefe 0 einen Hafen-Chunk (im selben Aufruf als `plannedChunks` mitgeben oder vorher `world_chunks_set`).
- `world_override_set {id, file, path, value}`: Override eines Basis-Felds (z. B. `economy` › `purse.startGold`); null setzt zurück.
- `world_chunks_set {id, island, depth, chunks?, count?}`: `plannedChunks` und `chunkCounts` einer Tiefe; `[]` bzw. 0 entfernt.
- `world_story_set {id, lexicon?, start?, etappen?, end?, bottles?, islands?}`: Geschichte der Welt (docs/rules/welten.md
  § 2.2) in einem Aufruf oder je Teil: `lexicon` Liste Kunstwörter, `start`/`end` je 1 bis 2 Zeilen, `etappen {"1": [Zeilen]}`,
  `bottles` Liste von Einträgen, `islands {<insel-id>: {arrival?, boss?, done?}}`. Fehlt ein Feld, bleibt es; `[]` entfernt
  den Eintrag; `lexicon` und `bottles` ersetzen die ganze Liste. Geschrieben wird nur, wenn die Prüfung der Engine besteht
  (sonst nennt der Fehler Feld und Zeile); die Antwort nennt gesetzte, entfernte und noch fehlende Einträge. `world_set`
  nimmt `story` nicht an.
- `world_check {id}`: prüft die Datei wie das Laden im Spiel, ändert nichts; fehlende Einträge der Geschichte kommen als
  `Hinweis: Geschichte fehlt (…)`, Verstöße als Fehler.

### Recherche für Grafik- und Sound-Slots

Das Slot-Register ist seit CU1.2 `data/sounds.json` (nur Sound-Slots); `id` ist hier ein Slot daraus.

- Recherche (Treffer in `data/assets-recherche.json`): `assets_research {id, query?}` fragt freesound (Token in
  `K3C_FREESOUND_TOKEN` oder `.work/freesound-token`), Wikimedia Commons und OpenGameArt; `assets_hits {id}` liest,
  `assets_hit_add {id, titel, link, …}` legt einen eigenen Fund ab (z. B. itch.io, Kenney), `assets_hit_drop {id, hit}`
  verwirft. Lizenz-Status nach Q70: `erkannt`, `pruefen` (bis 🧑 bestätigt), `ausgeschlossen` (ND, Pixabay, Mixkit …).
- `assets_import {id, hit, path?}` übernimmt einen Treffer: `path` = Datei im eigenen Checkout, z. B. `.work/` (Endung
  und Inhalt geprüft, höchstens 20 MB), ohne `path` der direkte Download (nur `https`, erkannte Lizenz, nur von
  `upload.wikimedia.org`). Ablage `public/grafik/kandidaten/` bzw.
  `public/audio/kandidaten/`, Credit-Zeile im Abschnitt „Importe (k3c-workbench)“, Zuordnung als letzte Präferenz. Die Lizenz
  bestätigt nur 🧑 in der Oberfläche; ein Agent nie.

### Rechnen ohne Server

- `level_generate {seed, biome?}` erzeugt ein Level im Prozess des Studios (`engine/simcli`, B-924), ohne laufenden
  Server und ohne Spiel. Gleiche Eingabe ergibt denselben Text wie im Spiel.
- Es rechnet **Code und Daten dieses Prozesses** (Stand beim Start des Studios), nicht die eines Worktrees; aus einem
  anderen Checkout lehnt es ab. Für einen Worktree dort ein eigenes Studio starten (`k3c-studio -headless` mit eigenem
  `K3C_STUDIO_PORT`). Die Welten- und Planer-Tools (`world_*`, `hub_*`) lesen `data/` des Checkouts dagegen frisch von
  der Platte. Ein Absturz der Engine ist ein Tool-Fehler (`💥 Absturz in der Engine`, Stack im Log), nie ein Absturz des
  Studios.
