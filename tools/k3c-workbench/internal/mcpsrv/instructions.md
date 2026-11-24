# k3c-workbench

Entwickler-Werkzeug des Spiels K3C. Diese Tools ersetzen Shell-Befehle und Dateilesen, weil sie nur das Wichtige
zurückgeben. Die Gliederung folgt der gemeinsamen Spec der Workbench-Seiten (`docs/standards/workbench-seiten.md`).
Zuständig für Entwicklung, Planung, Prüfläufe, Dienste und Logs. Welten, Inseln, Chunks, Hub, Assets, der
Level-Generator sowie Balancing und Simulation (Testläufe, Engine-Läufe, Berichte, Spielstände) liegen im MCP-Server
`k3c-studio`; dieser Server kennt nur die Tools unten.

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
  Session kommt (Header `X-K3C-Root` aus `.mcp.json`): Prüfläufe, Planung, Logs und Dienste nur dort. `workbench_status` nennt den Checkout in der Zeile `Checkout:`.
- **Im Worktree immer `checkout` setzen** (B-388): Schreibende Tools (`plan_create`, `plan_set`, `plan_section`,
  `plan_sections`, `plan_delete`, `svc_*`, `task_start`, `task_stop`) und `check_run` nehmen das Argument `checkout` mit dem Ordnernamen
  oder dem absoluten Pfad des eigenen Checkouts (`git rev-parse --show-toplevel`). Es gilt vor dem Header; der Header
  allein genügt nicht, die Desktop-App meldet aus Worktrees die Repo-Wurzel (B-341). Ein unbekannter oder mehrdeutiger
  Wert wird abgelehnt, die Meldung nennt die Worktrees. `task_*` mit einem Worktree lehnt ab (dort `check_run`).
- Ohne `checkout` und ohne Header gilt die Repo-Wurzel. Schreibende Tools lehnen nicht ab, nennen aber immer den
  Checkout in der letzten Zeile `Checkout:`. Steht dort aus einem Worktree die Repo-Wurzel, die Änderung in der Wurzel
  zurücknehmen und mit `checkout` wiederholen.
- Dienste eines Worktrees laufen auf eigenen Ports (Versatz 10, 20 …); `get_urls` nennt
  sie. Die Ports der Repo-Wurzel nie aus einem
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
- `plan_waves {wave_count?, project?, domain?}` (Standard 1 Welle) beim Planen oder Starten einer Welle (`docs/arbeitsweise.md` › Parallelität
  und Wellen): Kandidaten sind offene autonome Sessions aktiver Sprints, freigegebene Tickets ohne Sprint und
  freigegebene geplante Sprints mit `Braucht 🧑: –`. Ebene aus `Abhängig von`, Kollision bei überlappendem `Bereich`
  (Präfix der Globs, Sammeldateien ausgenommen; Sessions: Erlaubte Dateien) oder gleicher `Sperre`; Rang, Prio, Nummer
  entscheiden. Zwei Lanes je Welle: `zweit` (Second Worker, autonom) und `haupt` (Hauptrechner); jeder Knoten bekommt
  seine Lane (Session `Lane: zweit` → zweit, Sim-Domäne → zweit, sonst die freie). Sessions in Arbeit stehen in Welle 1
  und belegen ihre Lane. Je zurückgestelltem Knoten steht
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
- `plan_index {dry?}` (B-864): baut Haupt- und Archiv-Index aus den Ticket-Dateien neu (nach `git rm` oder
  Umnummerieren); `dry` ist Standard und zeigt nur die Zahlen.
- `plan_delete {id}` nur für Sprint-Entwürfe in `geplant/` und ihre Sessions; Tickets werden `verworfen`.
- Commits macht das Tool nicht. Danach `check_run task:test` mit `pattern: planning`.
- `gh_status {force?}` statt `gh pr list`/`gh run list` in der Shell: je Sprint PR, CI und Merge-Konflikt, dazu der letzte
  `develop`-Lauf; braucht eine angemeldete GitHub CLI, sonst steht der Hinweis in der Antwort.
- Die Planungs-Tools der Spec (`plan_project`, `plan_sprint`, `plan_session`, `plan_ticket`) kommen mit dem Umzug der
  Ablage; bis dahin gelten die Tools oben.

## 2. Dienste & Logs

- Dienste aus `services.json` (seit dem Rückbau des Spielservers, B-887, leer) nie per Shell starten: `svc_status` zuerst, dann `svc_start {service, waitSeconds?}`,
  `svc_stop {service, force?}`, `svc_restart {service, confirm: true}`; alle auf einmal mit `svc_start_all` und
  `svc_stop_all`.
- Läuft ein Dienst, antwortet aber nicht: `svc_health {service}`. Scheitert ein Start am Port: `ports_status`.
- Vor jedem HTTP-Aufruf und jeder Browserprüfung `get_urls {target?}`.
- Ein übernommener Dienst lief schon vor k3c-workbench (z. B. im Terminal eines Menschen): nur mit `force` stoppen, und nur,
  wenn das gewollt ist.
- Logs: `logs_services` zeigt die Dienste mit Logdatei (`logs/*.jsonl`, `k3c-workbench` ist das eigene Log). `logs_errors`
  zuerst (verdichtet, ohne `service` über alle), `logs_stats {service?, since?, groupBy?}` für Zahlen je Level oder
  Namensraum, `logs_query {service, level?, ns?, pattern?, since?, limit?}` für einzelne Zeilen,
  `logs_context {service, ts}` für die Zeilen um einen Zeitpunkt, `logs_since {service, cursor}` zum Mitlesen.
- `console_tail {service, limit?, since?}`: flüchtige Ausgabe eines Dienstes oder Laufs (`check:<ziel>`, `task:<name>`).

## 3. MCP

- Die Seite zeigt die Nutzung aller Tools; eigene Tools hat sie nicht. Kennzahlen des Servers liefert `workbench_status`.

## 4. Tasks

- `task_list {namespace?, query?}`: Taskfile-Baum; jeder Task ist mit `task_start` startbar, es gibt kein Freigabe-Schloss.
- `task_start {task, args?}` kehrt sofort zurück; `task_status`, `task_output {task, limit?, since?}`, `task_stop {task}`.
- Wer auf das Ende warten will, nimmt `check_run`. Testläufe nie über `task balance`, sondern über den MCP-Server `k3c-studio`.

## 5. Spieltest

Auswertung eines Spieltests (B-1224): Die Workbench ruft keine KI; Claude liest, schlägt Befunde vor, Ralf hakt an.

- `playtest_list`: Sessions mit Stand, Dauer, Markern, Befunden. `playtest_get {id, limit?, cursor?}`: Kopf, Fragebogen,
  Befunde und die Zeitleiste (Sätze und Marker mit Spielkontext, Bildpfad, `ts` in Unix-ms), seitenweise.
  Ohne Transkript drückt Ralf zuerst Auswerten auf der Seite Spieltest.
- `playtest_frame {id, t}`: Bild an Zeitpunkt `t` (Unix-ms) aus dem Video; den Pfad danach mit Read ansehen.
- `playtest_findings_set {id, findings}`: Befunde (`id`, `quote`, `ts`, Vorschlag für `type`, `prio`, `project`, `sprint`,
  `duplicate_of`) schreiben oder aktualisieren; vorher jeden Befund gegen offene Tickets prüfen (`plan_list`), Treffer als
  `duplicate_of`. Haken und Ticket setzt Claude nie.
- `playtest_tickets_create {id, domain?, type?, prio?, project?, sprint?}`: nur angehakte Befunde ohne Ticket werden Tickets
  (Zitat, Bild, Kontext) über die Planungslogik; `duplicate_of` ergibt einen Hinweis im vorhandenen Ticket. Projekt und
  Sprint nie erfinden: vom Befund, aus den Argumenten oder bei Ralf erfragen; fehlt der Sprint, zuerst `plan_create` kind sprint.

## Projektspezifisch

### Prüfen

- `check_run` statt `task check`, `task test`, `task typecheck`, `task lint`, `task build`, `task check:go`, `go test`
  oder `golangci-lint` in der Shell. Ziele: `task:check`, `task:test`, `task:typecheck`, `task:lint`, `task:build`,
  `task:check:go`, `task:check:diff`, `task:check:diff:plan`, `task:check:diff:js`, `task:check:diff:go`,
  `task:check:race`, `go:test`, `go:lint`, `dev:test`, `task:check:dev`. Ein Testmuster (`pattern`) geht bei
  `task:test`, `go:test` und `dev:test`.
- Workbench (B-831): `dev:test` baut das Frontend bei Bedarf und fährt `go test` der Workbench ohne Sim-Pakete;
  `task:check:dev` ist die volle Prüfung der Review (Frontend, Vitest, `go test`, Lint). Beide brauchen `task install`
  im Checkout und brechen ohne Frontend-Abhängigkeiten mit diesem Hinweis ab. `console_tail check:<ziel>` liefert nur
  Läufe des eigenen Checkouts (im Worktree `checkout` setzen, B-855).
- `task:check:diff` prüft nur, was die geänderten Dateien gegenüber der Basis `HEAD` brauchen (vor jedem Commit), in zwei
  Stufen: Stufe 1 Regel-Tests, Lint, Typecheck, `go vet`; Stufe 2 nur nach grüner Stufe 1 die Tests der geänderten Pakete.
  Nach einem roten Lauf wiederholt `task:check:diff:js` bzw. `task:check:diff:go` nur diese Bahn; `task:check:diff:plan`
  zeigt nur Basis und Schritte. `task:check:race` ist der langsame Race-Detector für `engine/` (Review-Session, CI).
- Die Antwort ist bei grünem Lauf eine Zeile, sonst Kopfzeile und nur die Fehlerzeilen.
- `check_run` wartet bis zum Ende des Laufs und sendet alle 20 s Fortschritt (B-520); ein Aufruf liefert das Ergebnis.
  Nie in Schleifen nachfragen. Nur ohne progressToken antwortet es nach 4 min mit „läuft noch“ und einer Lauf-ID;
  `check_run {run: "<ID>"}` wartet dann weiter. Der Lauf läuft auch weiter, wenn der Client abbricht.
- Mehr Kontext zu einem Lauf: `console_tail` mit `check:<ziel>`. Läufe verschiedener Worktrees sperren sich nicht.

### Balancing und Simulation

- Testläufe, Engine-Läufe, Replays, Berichte und Spielstände (Balancing, Simulation, Spieldaten) liegen seit WZ63.5 im
  MCP-Server `k3c-studio`; dort stehen auch die Regeln dazu, die Sim-Sperre (Beschluss 🧑 2026-11-11) eingeschlossen.
