# k3c-dev

Entwickler-Werkzeug des Spiels K3C. Diese Tools ersetzen Shell-Befehle und Dateilesen, weil sie nur das Wichtige
zurückgeben.

## Prüfen

- `check_run` statt `task check`, `task test`, `task typecheck`, `task lint`, `task build`, `task check:go`, `go test`
  oder `golangci-lint` in der Shell. Ziele: `task:check`, `task:test`, `task:typecheck`, `task:lint`, `task:build`,
  `task:check:go`, `go:test`, `go:lint`, `dev:test`. Ein Testmuster (`pattern`) geht bei `task:test`, `go:test` und `dev:test`.
- Die Antwort ist bei grünem Lauf eine Zeile, sonst Kopfzeile und nur die Fehlerzeilen.
- Mehr Kontext zu einem Lauf: `console_tail` mit `check:<ziel>`.

## Logs

- `logs_sources` zeigt alle Quellen. Log-Dateien liegen unter `logs/*.jsonl`, `k3c-dev` ist das eigene Log.
- `logs_errors` zuerst: Warnungen und Fehler, gleichartige zu einer Zeile verdichtet.
- `logs_query` für einzelne Einträge (Filter `minLevel`, `ns`, `pattern`, `since`, `limit`).
- `logs_since` zum Mitlesen: den `cursor` aus der letzten Antwort wieder mitgeben.

## Dienste

- Dienste (Vite-Dev-Server, Heimnetz-Server) nie per Shell starten, sondern mit `svc_start`; stoppen mit `svc_stop`.
- Der Heimnetz-Server startet von selbst neu, sobald sich Go-Code oder Daten ändern (`watch` in `services.json`); ein Neustart per Hand ist dafür nicht nötig. Die Konsole (`console_tail Heimnetz`) nennt die Datei; ein Build-Fehler steht dort und der Dienst heißt `fehlgeschlagen`, bis die nächste Änderung ihn wieder startet.
- `svc_status` zeigt Zustand, PID, CPU, Speicher und Log-Level; die Ausgabe eines Dienstes steht in
  `console_tail <Dienst>`.
- Ein übernommener Dienst lief schon vor k3c-dev (z. B. im Terminal eines Menschen): nur mit `force` stoppen, und nur,
  wenn das gewollt ist.

## Spieldaten

- `reports_list` und `report_read` statt `reports/*.json` zu öffnen: Xbox-Berichte der Gamepad-Testseite.
- `saves_list` statt `saves/` zu durchsuchen: Stufe, Tag, Spieler und Datum je Spielstand.

## Laufender Server

- `server_status`, `rooms_list` und `room_snapshot <Raum>` lesen `/api/status` des Go-Servers (nur lesend). Sie brauchen
  `K3C_STATUS_TOKEN`; Adresse `K3C_SERVER_URL`, sonst `127.0.0.1:K3C_HTTP_PORT` (8080). Ist der Server aus oder das
  Token falsch, steht das in der Meldung; kein Grund, den Server selbst abzufragen.

## Rechnen ohne Server

- `level_generate {seed, biome?}` und `sim_run {seed, ticks, biome?, inputs?}` rechnen in-process mit `engine/level` und
  `engine/sim`, ohne laufenden Server und ohne den Browser. Gleiche Eingabe ergibt denselben Text; `ticks` höchstens 100000
  (30 pro Sekunde). Für Balancing-Vergleiche statt eigener Skripte.

## Planung

- Tickets, Sprints und Sessions (`docs/backlog/`, `docs/sprints/`) **nur** über diese Tools ändern, nicht von Hand: sie
  halten Vorlage, Nummer, Index, Session-Tabelle, Fahrplan und Ablage (`archiv/`, `geplant/` → `aktiv/` → `erledigt/`) gleich.
- `plan_list {kind?, status?, domain?, sprint?, archive?}` für den Überblick, `plan_get {id}` für ein Dokument
  (`B-205`, `M8`, `M8.1`).
- `plan_create {kind, slug, title, id?, fields}`: Ticket (Felder `Domäne`, `Typ`, `Prio`), Sprint (`id`, `Domäne`) oder
  Session (`id` wie `M8.5`) als Kopie der Vorlage; danach die Abschnitte mit `plan_section {id, section, text}` füllen.
- `plan_set {id, fields}` setzt Kopf-Felder, z. B. `{"Status": "fertig"}`; Ticket `erledigt`/`verworfen` wandert ins
  Archiv, Sprint-`Status` verschiebt den Ordner. `Spec: freigegeben` nur mit `Freigabe` (Datum, Quelle) und nur nach
  Zustimmung von 🧑.
- `plan_delete {id}` nur für Sprint-Entwürfe in `geplant/` und ihre Sessions; Tickets werden `verworfen`.
- Commits macht das Tool nicht. Danach `check_run task:test` mit `pattern: planning`.

## Zustand

- `workbench_status`: Adresse, Laufzeit, Aufrufe, Clients, letzte Läufe, Log-Quellen.
