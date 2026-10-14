# k3c-dev

Entwickler-Werkzeug des Spiels K3C. Diese Tools ersetzen Shell-Befehle und Dateilesen, weil sie nur das Wichtige
zurückgeben. Die Gliederung folgt der gemeinsamen Spec der Workbench-Seiten (`docs/standards/workbench-seiten.md`).

## Regeln für alle Tools

- Namen `<bereich>_<verb>`; jede Beschreibung endet mit „Nutze es bei: … Statt: …“.
- Antworten sind verdichteter Text; große Ausgaben über `limit` und Cursor (`since`, `cursor`).
- Neustarts verlangen `confirm=true`; abgelehnte Aufrufe nennen den gültigen Wertebereich.

## Übergreifend

- `workbench_status`: Adresse, Laufzeit, Aufrufe, Clients, letzte Läufe, Log-Quellen, Checkout.
- `notify_ui {level, title, text?}`: Hinweis an den Nutzer in der Oberfläche (`info`, `success`, `warn`, `error`), z. B.
  wenn eine lange Arbeit fertig ist oder eine Freigabe nötig wird.

### Worktrees

- Eine k3c-dev-Instanz (aus der Repo-Wurzel) bedient alle Worktrees. Jeder Aufruf gilt dem Checkout, aus dem die
  Session kommt (Header `X-K3C-Root` aus `.mcp.json`): Prüfläufe, Planung, Logs, Berichte, Spielstände
  und Dienste nur dort. `workbench_status` nennt den Checkout in der Zeile `Checkout:`.
- **Im Worktree immer `checkout` setzen** (B-388): Schreibende Tools (`plan_create`, `plan_set`, `plan_section`,
  `plan_delete`, `assets_set`, `assets_research`, `assets_hit_add`, `assets_hit_drop`, `assets_import`, `svc_*`, `task_start`, `task_stop`) und `check_run` nehmen das Argument `checkout` mit dem Ordnernamen
  oder dem absoluten Pfad des eigenen Checkouts (`git rev-parse --show-toplevel`). Es gilt vor dem Header; der Header
  allein genügt nicht, die Desktop-App meldet aus Worktrees die Repo-Wurzel (B-341). Ein unbekannter oder mehrdeutiger
  Wert wird abgelehnt, die Meldung nennt die Worktrees. `task_*` mit einem Worktree lehnt ab (dort `check_run`).
- Ohne `checkout` und ohne Header gilt die Repo-Wurzel. Schreibende Tools lehnen nicht ab, nennen aber immer den
  Checkout in der letzten Zeile `Checkout:`. Steht dort aus einem Worktree die Repo-Wurzel, die Änderung in der Wurzel
  zurücknehmen und mit `checkout` wiederholen.
- Dienste eines Worktrees laufen auf eigenen Ports (Versatz 10, 20 …: Vite 5183, Spielserver 8090 …); `get_urls` nennt
  sie, `server_status` fragt den Spielserver des eigenen Worktrees. Die Ports der Repo-Wurzel (5173, 8080) nie aus einem
  Worktree ansprechen.
- `task_*` bedient nur die Repo-Wurzel; im Worktree `check_run` nutzen.

## 1. Planung

- Tickets, Sprints und Sessions (`docs/backlog/`, `docs/sprints/`) **nur** über diese Tools ändern, nicht von Hand: sie
  halten Vorlage, Nummer, Index, Session-Tabelle, Fahrplan und Ablage (`archiv/`, `geplant/` → `aktiv/` → `erledigt/`) gleich.
- `plan_list {kind?, status?, domain?, sprint?, archive?}` für den Überblick, `plan_get {id}` für ein Dokument
  (`B-210`, `M8`, `M8.1`).
- `plan_create {kind, slug, title, id?, fields}`: Ticket (Felder `Domäne`, `Typ`, `Prio`), Sprint (`id`, `Domäne`) oder
  Session (`id` wie `M8.5`) als Kopie der Vorlage; danach die Abschnitte mit `plan_section {id, section, text}` füllen.
- `plan_set {id, fields}` setzt Kopf-Felder, z. B. `{"Status": "fertig"}`; Ticket `erledigt`/`verworfen` wandert ins
  Archiv, Sprint-`Status` verschiebt den Ordner. `Spec: freigegeben` nur mit `Freigabe` (Datum, Quelle) und nur nach
  Zustimmung von 🧑.
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
- Ein übernommener Dienst lief schon vor k3c-dev (z. B. im Terminal eines Menschen): nur mit `force` stoppen, und nur,
  wenn das gewollt ist.
- Logs: `logs_services` zeigt die Dienste mit Logdatei (`logs/*.jsonl`, `k3c-dev` ist das eigene Log). `logs_errors`
  zuerst (verdichtet, ohne `service` über alle), `logs_stats {service?, since?, groupBy?}` für Zahlen je Level oder
  Namensraum, `logs_query {service, level?, ns?, pattern?, since?, limit?}` für einzelne Zeilen,
  `logs_context {service, ts}` für die Zeilen um einen Zeitpunkt, `logs_since {service, cursor}` zum Mitlesen.
- `console_tail {service, limit?, since?}`: flüchtige Ausgabe eines Dienstes oder Laufs (`check:<ziel>`, `task:<name>`).

## 3. MCP

- Die Seite zeigt die Nutzung aller Tools; eigene Tools hat sie nicht. Kennzahlen des Servers liefert `workbench_status`.

## 4. Tasks

- `task_list {namespace?, query?}`: Taskfile-Baum; `*` = Schloss offen, `task_start` erlaubt. Das Schloss setzt 🧑 auf
  der Tasks-Seite (Datei `tools/k3c-dev/task-freigaben.json` plus Schalter bis zum Beenden).
- `task_start {task, args?}` kehrt sofort zurück; `task_status`, `task_output {task, limit?, since?}`, `task_stop {task}`.
- Wer auf das Ende warten will, nimmt `check_run`. Testläufe nie über `task balance`/`task load`, sondern `sim_test`.

## Projektspezifisch

### Prüfen

- `check_run` statt `task check`, `task test`, `task typecheck`, `task lint`, `task build`, `task check:go`, `go test`
  oder `golangci-lint` in der Shell. Ziele: `task:check`, `task:test`, `task:typecheck`, `task:lint`, `task:build`,
  `task:check:go`, `go:test`, `go:lint`, `dev:test`. Ein Testmuster (`pattern`) geht bei `task:test`, `go:test` und `dev:test`.
- Die Antwort ist bei grünem Lauf eine Zeile, sonst Kopfzeile und nur die Fehlerzeilen.
- Dauert ein Lauf länger als ca. 45 s, antwortet `check_run` mit „läuft im Hintergrund“ und einer Lauf-ID; der Lauf
  läuft weiter, auch wenn der Client abbricht. `check_run {run: "<ID>"}` liefert Exit-Code und Fehlerzeilen (oder „läuft noch“).
- Mehr Kontext zu einem Lauf: `console_tail` mit `check:<ziel>`. Läufe verschiedener Worktrees sperren sich nicht.

### Spieldaten

- `reports_list` und `report_read` statt `reports/*.json` zu öffnen: Xbox-Berichte der Gamepad-Testseite.
- `saves_list` statt `saves/` zu durchsuchen: Stufe, Tag, Spieler und Datum je Spielstand.

### Grafik- und Sound-Slots

- `assets_list {art?, kategorie?, status?}` zeigt die Asset-Slots des Slot-Registers `data/assets.json` (je Zeile ID,
  Kategorie, Status `zugeordnet`/`luecke`/`fehlt`, aktive Zuordnung); `assets_get {id}` einen Slot mit allen Präferenzen.
- `assets_catalog {query?, art?, limit?}` sucht Dateien unter `public/grafik`, `public/sprites`, `public/audio` mit Pack,
  Gruppe, Maßen und Lizenz; Kennzeichen `[NC]`, `[SA]`, `[ND]` nach Q70.
- `assets_set {id, wahl}` ersetzt die Zuordnungen (Reihenfolge = Präferenz, nur die erste ist im Spiel aktiv); jede
  `datei` relativ zu `public/`, Ausschnitt `{x,y,w,h}` innerhalb des Bildes. Ein ungültiges Register wird nie überschrieben.
- Recherche (Treffer in `data/assets-recherche.json`): `assets_research {id, query?}` fragt freesound (Token in
  `K3C_FREESOUND_TOKEN` oder `.work/freesound-token`), Wikimedia Commons und OpenGameArt; `assets_hits {id}` liest,
  `assets_hit_add {id, titel, link, …}` legt einen eigenen Fund ab (z. B. itch.io, Kenney), `assets_hit_drop {id, hit}`
  verwirft. Lizenz-Status nach Q70: `erkannt`, `pruefen` (bis 🧑 bestätigt), `ausgeschlossen` (ND, Pixabay, Mixkit …).
- `assets_import {id, hit, path?}` übernimmt einen Treffer: `path` = lokale Datei (Endung und Inhalt geprüft, höchstens
  20 MB), ohne `path` der direkte Download (nur `https` und erkannte Lizenz). Ablage `public/grafik/kandidaten/` bzw.
  `public/audio/kandidaten/`, Credit-Zeile im Abschnitt „Importe (k3c-dev)“, Zuordnung als letzte Präferenz. Die Lizenz
  bestätigt nur 🧑 in der Oberfläche; ein Agent nie.

### Laufender Server

- `server_status`, `rooms_list` und `room_snapshot <Raum>` lesen `/api/status` des Go-Servers (nur lesend). Sie brauchen
  `K3C_STATUS_TOKEN`; Adresse `K3C_SERVER_URL`, sonst `127.0.0.1:K3C_HTTP_PORT` (8080). Ist der Server aus oder das
  Token falsch, steht das in der Meldung; kein Grund, den Server selbst abzufragen.

### Rechnen ohne Server

- `level_generate {seed, biome?}` und `sim_run {seed, ticks, biome?, inputs?}` rechnen in-process mit `engine/level` und
  `engine/sim`, ohne laufenden Server und ohne den Browser. Gleiche Eingabe ergibt denselben Text; `ticks` höchstens 100000
  (30 pro Sekunde). Für Balancing-Vergleiche statt eigener Skripte.
- `replay_run {path}` spielt eine Replay-Datei (`task balance:run -- --replay-dir DIR`) ohne Bot ab: Endzustand-Hash,
  Burgfall-Tick, Vergleich mit der Aufnahme; `path` relativ zur Repo-Wurzel, Dateien außerhalb werden abgelehnt.
- Diese drei rechnen mit der Engine, mit der k3c-dev gebaut ist (Repo-Wurzel), nicht mit geänderter Engine im
  Worktree; dafür `check_run go:test`.

### Testläufe

- **Jeder Testlauf** (Balancing, Performance, Stabilität) läuft über `sim_test`, nie über `task balance`/`task load` in
  der Shell. `sim_test {action: start, mode, clients, focus, …}` kehrt sofort mit einer ID zurück; `sim_test {action:
  status, id}` zeigt den Lauf in höchstens 10 Zeilen, `stop` bricht ab, `list` zeigt die Läufe des Checkouts.
- `mode: offline` (Standard, Mocks im Prozess) baut den Balancing-Tester im Checkout und bewertet die Ziele aus
  `data/balance-targets.json` (`seeds` optional). `mode: online, clients: 0` startet `rooms` × `players` Bot-Geräte
  (`bots`, Standard 1 × 2 `saver`, `duration` 5m) gegen den Spielserver des Checkouts (vorher `svc_start`) und
  misst Tick p99, CPU, Trennungen und Fehler.
- `mode: online, clients: 1`–`4` öffnet je Client einen Raum mit einem Beobachter-Bot der Workbench (belegt einen Platz,
  daher `players` 1–3) und startet Edge oder Chrome headless mit `game.html?room=…&players=…&botfeed=…` am Vite-Port
  des Checkouts (vorher `svc_start` für Vite und Spielserver). Bots steuern die Monarchen über den Bot-Feed
  (`/bot/<id>/<n>`, nur Loopback); FPS und Latenz kommen aus dem Log `k3c-client`. `attach: <Raumcode>` mit `clients: 1`
  startet keinen Browser: den Client auf diesem Rechner mit der Feed-Adresse aus dem Status öffnen.
- Der volle Bericht liegt unter `reports/simtest-<id>/` (`simtest.md` und der Bericht des Testers), lesbar mit `report_read`.
