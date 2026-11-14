# k3c-workbench

Entwickler-Werkzeug des Spiels K3C. Diese Tools ersetzen Shell-Befehle und Dateilesen, weil sie nur das Wichtige
zurückgeben. Die Gliederung folgt der gemeinsamen Spec der Workbench-Seiten (`docs/standards/workbench-seiten.md`).

## Regeln für alle Tools

- Namen `<bereich>_<verb>`; jede Beschreibung endet mit „Nutze es bei: … Statt: …“.
- Antworten sind verdichteter Text; große Ausgaben über `limit` und Cursor (`since`, `cursor`).
- Neustarts verlangen `confirm=true`; abgelehnte Aufrufe nennen den gültigen Wertebereich.

## Übergreifend

- `workbench_status`: Adresse, Laufzeit, Aufrufe, Clients, letzte Läufe, Log-Quellen, Checkout.
- `workbench_resources`: Speicher (Live-Heap, RSS, Kindprozesse), Goroutinen, Handles, Trend je Stunde, Puffer und die Vorgänge mit der größten Allokation; Heap-Profile unter `logs/profile/`, pprof unter `/debug/pprof/` des MCP-Ports.
- `notify_ui {level, title, text?}`: Hinweis an den Nutzer in der Oberfläche (`info`, `success`, `warn`, `error`), z. B.
  wenn eine lange Arbeit fertig ist oder eine Freigabe nötig wird.

### Worktrees

- Eine k3c-workbench-Instanz (aus der Repo-Wurzel) bedient alle Worktrees. Jeder Aufruf gilt dem Checkout, aus dem die
  Session kommt (Header `X-K3C-Root` aus `.mcp.json`): Prüfläufe, Planung, Logs, Berichte, Spielstände
  und Dienste nur dort. `workbench_status` nennt den Checkout in der Zeile `Checkout:`.
- **Im Worktree immer `checkout` setzen** (B-388): Schreibende Tools (`plan_create`, `plan_set`, `plan_section`,
  `plan_sections`, `plan_delete`, `svc_*`, `task_start`, `task_stop`), alle `assets_*` (auch die lesenden), `check_run` und `sim_test` nehmen das Argument `checkout` mit dem Ordnernamen
  oder dem absoluten Pfad des eigenen Checkouts (`git rev-parse --show-toplevel`). Es gilt vor dem Header; der Header
  allein genügt nicht, die Desktop-App meldet aus Worktrees die Repo-Wurzel (B-341). Ein unbekannter oder mehrdeutiger
  Wert wird abgelehnt, die Meldung nennt die Worktrees. `task_*` mit einem Worktree lehnt ab (dort `check_run`).
- Ohne `checkout` und ohne Header gilt die Repo-Wurzel. Schreibende Tools lehnen nicht ab, nennen aber immer den
  Checkout in der letzten Zeile `Checkout:`. Steht dort aus einem Worktree die Repo-Wurzel, die Änderung in der Wurzel
  zurücknehmen und mit `checkout` wiederholen.
- Dienste eines Worktrees laufen auf eigenen Ports (Versatz 10, 20 …: Vite 5193, Spielserver 8090 …); `get_urls` nennt
  sie, `server_status` fragt den Spielserver des eigenen Worktrees. Die Ports der Repo-Wurzel (5183, 8080) nie aus einem
  Worktree ansprechen.
- `task_*` bedient nur die Repo-Wurzel; im Worktree `check_run` nutzen.

## 1. Planung

- Tickets, Sprints, Sessions und Projekte (`docs/backlog/`, `docs/sprints/`, `docs/projekte/`) **nur** über diese Tools
  ändern, nie von Hand: sie halten Vorlage, Nummer, Index, Session-Tabelle, Fahrplan und Ablage (`archiv/`, `geplant/` →
  `aktiv/` → `erledigt/`) gleich. **Läuft die Workbench, lehnt der Hook `tools/hooks/planGuard.ts` jede andere Änderung
  ab** (Edit, Write, schreibende Shell-Befehle wie `sed -i`, `rm`, Umleitung, Skripte); Lesen per Shell bleibt erlaubt,
  ist aber zweite Wahl: Überblick und Zählung immer über `plan_list` (wie die Planungsseite der Workbench).
- Begriffe wie auf der Planungsseite: **offen** = Status offen, **eingeplant** = Status eingeplant, **ungeplant** = ohne
  Sprint (`plan_list {kind: "ticket", sprint: "ungeplant"}`).
- `plan_list {kind?, status?, domain?, sprint?, archive?}` für den Überblick, `plan_get {id}` für ein Dokument
  (`B-210`, `M8`, `M8.1`).
- `plan_next {domain?}` zu Beginn einer autonomen Session: die nächste Session nach `docs/arbeitsweise.md` › Autonomer
  Ablauf, Schritt 1 (Rang, aktive Sprints, offen und autonom, Abhängigkeiten erledigt, keine Kollision mit einer Session
  in Arbeit), mit Pfad, Branch `sprint/<präfix>` (ein Feld `Branch: sprint/cu1` bleibt, `w15/1-x` wird `sprint/w15`),
  Session-Datei, ihren Kriterien aus der Sprint-README und den parallel möglichen Sessions; sonst „Keine Session frei“
  mit Grund je Projekt. Kollision wie bei `plan_waves`: gleiche Sperre der Sprint-README oder überlappende Erlaubte
  Dateien; fehlt einer Session die Dateimenge, gilt noch die Sperre nach Domäne und Lane. Liest nur die lokalen Dateien:
  die Sperre über `origin` (`git ls-remote --heads origin 'sprint/*'`) prüft der Agent selbst.
- `plan_waves {slots?, project?, domain?}` beim Planen oder Starten einer Welle (`docs/arbeitsweise.md` › Parallelität
  und Wellen): Kandidaten sind offene autonome Sessions aktiver Sprints, freigegebene Tickets ohne Sprint und
  freigegebene geplante Sprints mit `Braucht 🧑: –`. Ebene aus `Abhängig von`, Kollision bei überlappendem `Bereich`
  (Präfix der Globs, Sammeldateien ausgenommen; Sessions: Erlaubte Dateien) oder gleicher `Sperre`; Rang, Prio, Nummer
  entscheiden. Sessions in Arbeit stehen in Welle 1 und belegen Plätze (höchstens 4). Je zurückgestelltem Knoten steht
  der Grund da; „In keiner Welle“ nennt `Bereich fehlt`, `wartet auf …` und als `Fehler:` unbekannte IDs.
  `project`/`domain` grenzen nur die Anzeige ein.
- `plan_check {id}` vor der Freigabe einer Spec und vor `plan_waves`: nennt je Ticket oder Sprint die Mängel der
  Planungsangaben (Zeilen fehlen, `Bereich` leer, `Umgebung: ?`, `Größe: L`, unbekannte ID oder Freitext in
  `Abhängig von`, `Bereich` überlappt mit einem anderen offenen Ticket); archivierte Tickets prüft es nicht.
- `plan_queue {device?}` legt 🧑 gesammelt vor (Ralf-Queue): Freigaben (Specs im Entwurf mit `Braucht 🧑: Freigabe`),
  Entscheidungen, Abnahmen je Gerät (PC, Xbox/TV, Pi, Mini-PC, Workshop, Veröffentlichung, `unbekannt`). Das Gerät einer
  Mensch-Session steht in einer Zeile `Gerät: …` im Abschnitt Kontext, sonst gilt `unbekannt`.
- `plan_create {kind, slug, title, id?, fields}`: Ticket (Felder `Domäne`, `Typ`, `Prio`), Sprint (`id`, `Domäne`) oder
  Session (`id` wie `M8.5`) als Kopie der Vorlage; danach die Abschnitte mit `plan_sections {id, sections}` füllen
  (`sections`: Abschnitt → Text). Mehrere Abschnitte immer mit `plan_sections`; ein unbekannter Abschnitt bricht ab,
  ohne etwas zu schreiben. Einen einzelnen Abschnitt ersetzt auch `plan_section {id, section, text}`. Beim Ticket nennt
  die Antwort bis zu fünf Tickets mit ähnlichem Titel (offen und Archiv, nur Hinweis, B-696).
- `plan_set {id, fields}` setzt Kopf-Felder, z. B. `{"Status": "fertig"}`; Ticket `erledigt`/`verworfen` wandert ins
  Archiv, Sprint-`Status` verschiebt den Ordner. `Spec: freigegeben` nur mit `Freigabe` (Datum, Quelle) und nur nach
  Zustimmung von 🧑.
- Relationen (B-696): `plan_set B-741 {"Duplikat von": "B-823"}` verwirft und archiviert B-741; `{"Ersetzt": "B-210, B-211"}`
  im neueren Ticket. Unbekannte ID, Selbstverweis und Zyklus lehnt das Tool ab, ältere Tickets bekommen das Feld
  eingefügt. `plan_get` zeigt die Gegenrichtung (`Ersetzt durch`, `Duplikate`) berechnet unter dem Kopf.
- `plan_digest {älter_als?, dry?, neu?}` (B-696): je erledigtem Sprint eine `ZUSAMMENFASSUNG.md` (Ziel, Abnahme,
  Kriterien mit Status, Tickets); `dry` ist Standard, vorhandene bleiben ohne `neu`. Statt eines erledigten Sprint-Ordners
  zuerst die Zusammenfassung lesen.
- `plan_delete {id}` nur für Sprint-Entwürfe in `geplant/` und ihre Sessions; Tickets werden `verworfen`.
- Commits macht das Tool nicht. Danach `check_run task:test` mit `pattern: planning`.
- `gh_status {force?}` statt `gh pr list`/`gh run list` in der Shell: je Sprint PR, CI und Merge-Konflikt, dazu der letzte
  `develop`-Lauf; braucht eine angemeldete GitHub CLI, sonst steht der Hinweis in der Antwort.
- Die Planungs-Tools der Spec (`plan_project`, `plan_sprint`, `plan_session`, `plan_ticket`) kommen mit dem Umzug der
  Ablage; bis dahin gelten die Tools oben.

## 2. Dienste & Logs

- Dienste (Vite, Spielserver) nie per Shell starten: `svc_status` zuerst, dann `svc_start {service, waitSeconds?}`,
  `svc_stop {service, force?}`, `svc_restart {service, confirm: true}`; alle auf einmal mit `svc_start_all` und
  `svc_stop_all`.
- Läuft ein Dienst, antwortet aber nicht: `svc_health {service}`. Scheitert ein Start am Port: `ports_status`.
- Vor jedem HTTP-Aufruf und jeder Browserprüfung `get_urls {target?}`.
- Der Spielserver startet von selbst neu, sobald sich Go-Code oder Daten ändern (`watch` in `services.json`). Die
  Konsole (`console_tail {service: Spielserver}`) nennt die Datei; ein Build-Fehler steht dort, und der Dienst heißt
  `fehlgeschlagen`, bis die nächste Änderung ihn wieder startet.
- Ein übernommener Dienst lief schon vor k3c-workbench (z. B. im Terminal eines Menschen): nur mit `force` stoppen, und nur,
  wenn das gewollt ist.
- Logs: `logs_services` zeigt die Dienste mit Logdatei (`logs/*.jsonl`, `k3c-workbench` ist das eigene Log). `logs_errors`
  zuerst (verdichtet, ohne `service` über alle), `logs_stats {service?, since?, groupBy?}` für Zahlen je Level oder
  Namensraum, `logs_query {service, level?, ns?, pattern?, since?, limit?}` für einzelne Zeilen,
  `logs_context {service, ts}` für die Zeilen um einen Zeitpunkt, `logs_since {service, cursor}` zum Mitlesen.
  **Entfernte Server (B-771):** Alle `logs_*`-Tools, `server_status`, `reports_list` und `report_read` nehmen `server`
  (`pi` = pi-gaming:8080, weitere Namen aus der Umgebungsvariable `K3C_REMOTE_SERVERS="name=host:port,…"`, oder `host:port`
  im Heimnetz). Sie lesen dann die Dev-Endpunkte `/api/dev/*` dieses Servers (nur GET, Dev-Mode am Server nötig, Log-Ordner
  `K3C_LOG_DIR` am Server gesetzt) statt der lokalen Dateien. Es gelten nur die Dienste `k3c-server` und `k3c-client`; je
  Datei kommen die letzten 1000 Zeilen. `logs_since` nutzt dann die Byte-Position im Log des Servers als Cursor. Die Antwort
  nennt oben die Quelle. Ohne `server` bleibt alles lokal.
- `console_tail {service, limit?, since?}`: flüchtige Ausgabe eines Dienstes oder Laufs (`check:<ziel>`, `task:<name>`).

## 3. MCP

- Die Seite zeigt die Nutzung aller Tools; eigene Tools hat sie nicht. Kennzahlen des Servers liefert `workbench_status`.

## 4. Tasks

- `task_list {namespace?, query?}`: Taskfile-Baum; `*` = Schloss offen, `task_start` erlaubt. Das Schloss setzt 🧑 auf
  der Tasks-Seite (Datei `tools/k3c-workbench/task-freigaben.json` plus Schalter bis zum Beenden).
- `task_start {task, args?}` kehrt sofort zurück; `task_status`, `task_output {task, limit?, since?}`, `task_stop {task}`.
- Wer auf das Ende warten will, nimmt `check_run`. Testläufe nie über `task balance`/`task k3c-load`, sondern `sim_test`.

## Projektspezifisch

### Prüfen

- `check_run` statt `task check`, `task test`, `task typecheck`, `task lint`, `task build`, `task check:go`, `go test`
  oder `golangci-lint` in der Shell. Ziele: `task:check`, `task:test`, `task:typecheck`, `task:lint`, `task:build`,
  `task:check:go`, `task:check:diff`, `task:check:diff:plan`, `task:check:diff:js`, `task:check:diff:go`,
  `task:check:race`, `go:test`, `go:lint`, `dev:test`. Ein Testmuster (`pattern`) geht bei `task:test`, `go:test` und
  `dev:test`.
- `task:check:diff` prüft nur, was die geänderten Dateien gegenüber der Basis `HEAD` brauchen (vor jedem Commit), in zwei
  Stufen: Stufe 1 Regel-Tests, Lint, Typecheck, `go vet`; Stufe 2 nur nach grüner Stufe 1 die Tests der geänderten Pakete.
  Nach einem roten Lauf wiederholt `task:check:diff:js` bzw. `task:check:diff:go` nur diese Bahn; `task:check:diff:plan`
  zeigt nur Basis und Schritte. `task:check:race` ist der langsame Race-Detector für `engine/` (Review-Session, CI).
- **Sim-Sperre (B-788):** `sim_test start` und `sim_run` lehnen ab, solange Simulationen und Balancing gesperrt sind
  (Beschluss 🧑 2026-11-11). Nur 🧑 hebt die Sperre auf: Workbench mit `K3C_SIM_ENTSPERRT=1` starten.
- Die Antwort ist bei grünem Lauf eine Zeile, sonst Kopfzeile und nur die Fehlerzeilen.
- `check_run` wartet bis zum Ende des Laufs und sendet alle 20 s Fortschritt (B-520); ein Aufruf liefert das Ergebnis.
  Nie in Schleifen nachfragen. Nur ohne progressToken antwortet es nach 4 min mit „läuft noch“ und einer Lauf-ID;
  `check_run {run: "<ID>"}` wartet dann weiter. Der Lauf läuft auch weiter, wenn der Client abbricht.
- Mehr Kontext zu einem Lauf: `console_tail` mit `check:<ziel>`. Läufe verschiedener Worktrees sperren sich nicht.

### Spieldaten

- `reports_list` und `report_read` statt `reports/*.json` zu öffnen: Xbox-Berichte der Gamepad-Testseite.
- `saves_list` statt `saves/` zu durchsuchen: Stufe, Tag, Spieler und Datum je Spielstand.

### Assets (Papierschnitt)

Assets sind Paper-Specs `public/grafik/<gruppe>/<id>.paper.json` (Vertrag `docs/standards/paper-mcp.md`); geändert wird
**nur** über diese Tools, jedes nimmt `checkout`. Neue Tools gelten erst nach Merge und Neustart der Workbench aus der
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

### Laufender Server

- `server_status`, `rooms_list` und `room_snapshot <Raum>` lesen `/api/status` des Go-Servers (nur lesend). Sie brauchen
  `K3C_STATUS_TOKEN`; Adresse `K3C_SERVER_URL`, sonst `127.0.0.1:K3C_HTTP_PORT` (8080). Ist der Server aus oder das
  Token falsch, steht das in der Meldung; kein Grund, den Server selbst abzufragen.

### Rechnen ohne Server

- `level_generate {seed, biome?}` und `sim_run {seed, ticks, biome?, inputs?}` rechnen im Kindprozess `bin/k3c-sim`
  (Engine-CLI, B-726), ohne laufenden Server und ohne den Browser. Gleiche Eingabe ergibt denselben Text; `ticks` höchstens 100000
  (30 pro Sekunde). Für Balancing-Vergleiche statt eigener Skripte.
- `replay_run {path}` spielt eine Replay-Datei (`task balance:run -- --replay-dir DIR`) ohne Bot ab: Endzustand-Hash,
  Burgfall-Tick, Vergleich mit der Aufnahme; `path` relativ zur Repo-Wurzel, Dateien außerhalb werden abgelehnt.
- Diese drei rechnen mit `bin/k3c-sim` des Checkouts der Session (Daten aus dessen `data/`). Fehlt die EXE oder ist sie
  älter als `engine/`, `cmd/k3c-sim`, `go.mod`: Meldung mit `task k3c-sim:build`. `sim_test` offline baut sich
  `bin/k3c-sim-<platz>` selbst; online rechnen die Bots in `k3c-sim bot` (ebenfalls `bin/k3c-sim`).

### Testläufe

- **Jeder Testlauf** (Balancing, Performance, Stabilität) läuft über `sim_test`, nie über `task balance`/`task k3c-load` in
  der Shell. `sim_test {action: start, mode, clients, focus, …}` kehrt sofort mit einer ID zurück; `sim_test {action:
  status, id}` zeigt den Lauf in höchstens 10 Zeilen, `stop` bricht ab, `list` zeigt die Läufe des Checkouts.
- `mode: offline` (Standard, Mocks im Prozess) baut den Balancing-Tester im Checkout und bewertet die Ziele aus
  `data/balance-targets.json`. Ohne `seeds` gilt dort `seedMode`: `dev` rechnet nur den ersten Seed `k3c-dev`, `all`
  alle (B-634). `seeds: N` nimmt die ersten N, `seedList: "k3c-dev,7"` genau diese; `bots` (z. B. `coop2`) ersetzt
  offline den Bot aller Ziele. Der Bericht nennt Seed-Anzahl und Seed-Modus.
- **BAS-Standardlauf** (Projekt BAS, Meilenstein-Läufe `docs/rules/zielkorridore.md` § 6): `sim_test {action: start,
  bots: coop2}`. Bis zur Abnahme der neuen Werte rechnet er nur `k3c-dev` und ist in Minuten fertig.
- **Insel-Lauf / Meilenstein** (BS11, B-642): `sim_test {action: start, bots: coop2, world: standard, days: 40}` spielt die
  ganze erste Insel der Welt (alle Tiefen; die Bots steigen ab und kämpfen). `world` (Welt-Datei aus `data/worlds/`) und
  `days` (1 bis 60) gelten nur offline und für alle Ziele des Laufs; der Bericht zeigt je Tiefe Ankunftstag und Burgfall, je
  Boss den Tag und den Endboss (`kennzahlen.island`). Ein 40-Tage-Lauf dauert mehrere Minuten je Seed.
- **Spieler und Varianten offline** (BS3.4): `players` (1–4) setzt die Spieleranzahl aller Ziele (Single-Player-Meilenstein: `bots: wirtschaft, players: 1, world: standard, days: 40`); `vary: "datei.json:pfad=wert;…"` ändert Werte nur für diesen Lauf (z. B. `economy.json:purse.startGold=500`), `data/` bleibt unverändert.
- **Delta-Runde nach einem Meilenstein** (B-640, BS7): Jedes Ziel und jeder Hebel gehört zu einer Werte-Gruppe (`basis`, `holz`,
  `schiff`; `group` in `data/balance-targets.json` und `balance-hints.json`), abgenommene Gruppen stehen in `frozen`. Ein durchgelaufener
  Offline-Lauf wird mit `sim_test {action: baseline, id: run-N, saveBaseline: M1}` unter einem Namen gesichert (`reports/baseline/M1/`
  mit Bericht und `run.json`; nie überschrieben, nur bei `Pass`); `sim_test {action: baseline}` listet sie. Die Delta-Runde nach einer
  neuen Mechanik ist `sim_test {action: start, bots: coop2, baseline: M1}`: Der Bericht vergleicht je Gruppe (Ampel, Trend); nur die
  freie Gruppe der neuen Mechanik wird justiert, eingefrorene melden sich nur, wenn sie kippen („eingefrorene Gruppe kippt“ ist ein
  Befund: Ticket, 🧑 fragen), und Tuning-Hinweise nennen keine Hebel eingefrorener Gruppen.
- **Warten wie `check_run`** (B-644): `sim_test {action: start, …, wait: true}` bzw. `sim_test {action: wait, id}` wartet auf das Ende
  des Laufs und liefert den Endstatus. Mit `progressToken` meldet der Aufruf alle 20 s ein Lebenszeichen und wartet bis zum Ende;
  ohne antwortet er nach 4 min mit „läuft noch“ und der ID. Der Lauf überlebt den Aufruf; `stop` bricht ihn ab.
- `mode: online, clients: 0` startet `rooms` × `players` Bot-Geräte
  (`bots`, Standard 1 × 2 `saver`, `duration` 5m) gegen den Spielserver des Checkouts (vorher `svc_start`) und
  misst Tick p99, CPU, Trennungen und Fehler.
- `mode: online, clients: 1`–`4` öffnet je Client einen Raum mit einem Beobachter der Workbench (ohne Platz, B-352,
  daher `players` 1–4) und startet Edge oder Chrome headless mit `game.html?room=…&players=…&botfeed=…` am Vite-Port
  des Checkouts (vorher `svc_start` für Vite und Spielserver). Bots steuern die Monarchen über den Bot-Feed
  (`/bot/<id>/<n>`, nur Loopback); FPS und Latenz kommen aus dem Log `k3c-client`. `attach: <Raumcode>` mit `clients: 1`
  startet keinen Browser: den Client auf diesem Rechner mit der Feed-Adresse aus dem Status öffnen.
- Der volle Bericht liegt unter `reports/simtest-<id>/` (`simtest.md` und der Bericht des Testers), lesbar mit `report_read`.
