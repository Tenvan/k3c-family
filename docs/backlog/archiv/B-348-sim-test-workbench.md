# B-348 · Jeder Testlauf startet und läuft über das MCP-Tool `sim_test`

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** TR1
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑 („annehmen und sofort als Sprint anlegen und starten“, ein Tool `sim_*` mit Parametern)

## Ausgangslage

Mehrere Messwerkzeuge laufen getrennt und von Hand. `task balance` und `k3c-balance` simulieren offline im Prozess, `task load` bzw. `cmd/k3c-load` lässt Bots gegen einen Server laufen, Stabilität prüft niemand gezielt. Jedes Werkzeug hat eigene Parameter, eigene Berichte und eine lange Ausgabe. Agenten starten sie in der Shell und überwachen sie nicht; ein Lauf mit echten Clients, deren Monarchen Bots steuern, fehlt ganz. Die Workbench (`tools/k3c-dev`) hat dafür nur `sim_run` (ein Lauf, ein Seed) und `check_run`.

## Ziel

Ein MCP-Tool `sim_test` der Workbench startet und überwacht jeden Testlauf: Massen- und Stresstests für Performance (Server und Client), Balancing und Stabilität, tokenfreundlich und in allen sinnvollen Kombinationen der Lauf-Modi.

## Beteiligte und Zielgruppen

Agenten und 🧑 als Nutzer der Workbench; REG (Balancing), SRV (Performance), CLI/PLAT (Client-Leistung). 🧑 entscheidet.

## Anforderungen

- **Ein Tool:** `sim_test` mit Parameter `action`: `start`, `status`, `stop`, `list`. `start` kehrt sofort mit einer Lauf-ID zurück, der Lauf arbeitet im Hintergrund der Workbench weiter. Mit `status` wird er überwacht, mit `stop` abgebrochen.
- **Lauf-Modi und Kombinationen** (Parameter von `start`):
  - `mode`: `offline` (Simulation im Prozess, Geräte und Server als Mock) oder `online` (gegen den Spielserver des eigenen Checkouts, `svc_start`).
  - `clients`: `0` = headless ohne Client, `1`–`4` = laufende Clients (Browser), deren Monarchen Bots über die Bot-Eingabe steuern (B-349).
  - `players` je Raum (1–4), `rooms`, `bots` (Profil aus dem Balancing-Tester), `seeds`, `days` bzw. `duration`.
  - `focus`: `balance`, `perf`, `stability` oder mehrere; bestimmt, welche Kennzahlen bewertet werden.
  - Unmögliche Kombinationen (z. B. `offline` mit `clients` > 0) lehnt `start` mit einer Zeile Grund ab.
- **Tokenfreundlich:** `start` antwortet in einer Zeile, `status` in höchstens 10 Zeilen (Phase, Fortschritt, Kennzahlen bisher, Fehlerzahl, Bericht). Der volle Bericht liegt unter `reports/` (`.json` und `.md`) und ist mit `report_read` lesbar.
- **Bots steuern Monarchen:** headless online als WebSocket-Gerät, im Client-Modus über die Bot-Eingabe des Clients. Die Entscheidung trifft in allen Modi derselbe Bot (`tools/k3c-dev/internal/balance`).
- **Fester Katalog:** Parameter werden geprüft (Positivliste, Grenzen); kein freier Befehl, keine Shell (wie `check_run`, B-046).
- **Überwachung:** Ein Lauf endet nach Plan, durch `stop` oder durch ein Zeitlimit; Absturz eines Prozesses, Trennung oder `💥` im Log zählt der Lauf als Stabilitätsfehler.

## Nicht-Ziele

Neue Bot-Strategien (BAL5, B-347), Wertänderungen in `data/`, Messung auf Xbox oder Pi (Hardware-Sessions), Client-Modus offline.

## Regeln und Einschränkungen

Domäne SRV (`tools/k3c-dev/`). Der Client-Anteil (Bot-Eingabe) ist B-349 (PLAT). Keine neue Abhängigkeit; den Browser für den Client-Modus nimmt die Workbench aus der Installation (Edge oder Chrome, headless). Dienste und Ports gelten je Worktree (Versatz 10, 20 …). Logging mit Emojis. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

- `sim_test {action: start, mode: offline, focus: balance, seeds: 100, days: 10}` → `▶ run-3 offline·headless·balance gestartet`. `status run-3` → `✅ fertig 100/100 · Burg hält 4 % (Fail) · Bericht reports/simtest-…md`.
- `sim_test {action: start, mode: online, clients: 0, rooms: 4, players: 3, focus: perf,stability, duration: 10m}` → 4 Räume mit je 3 Bot-Geräten gegen den Server, Tick p99 und Fehler im Status.
- `sim_test {action: start, mode: online, clients: 2, players: 2, focus: perf}` → 2 Browser-Clients mit je 2 lokalen Spielern, Bots steuern die Monarchen, FPS und Latenz der Clients im Status.

## Ausnahme- und Fehlerfälle

- Spielserver läuft nicht → `start` lehnt mit Hinweis auf `svc_start` ab.
- Kein Browser gefunden → `start` mit `clients` > 0 lehnt mit Grund ab.
- Workbench beendet → laufende Läufe werden gestoppt, ihr Bericht vermerkt den Abbruch.

## Akzeptanzkriterien

- **AC-01** `sim_test` mit `start`, `status`, `stop` und `list` (Test mit Fake-Lauf): `start` kehrt sofort mit ID zurück, `status` hat höchstens 10 Zeilen, `stop` beendet den Lauf und schreibt einen Bericht.
- **AC-02** `mode: offline` (headless, Mocks) führt Balancing-Läufe über viele Seeds aus und bewertet die Ziele aus `data/balance-targets.json` (Test).
- **AC-03** `mode: online`, `clients: 0`: Bot-Geräte steuern ihre Monarchen über WebSocket gegen den Spielserver; Tick-Dauer, CPU, Trennungen und Fehler stehen im Status (Test gegen einen Server im Testprozess).
- **AC-04** `mode: online`, `clients: 1`–`4`: Die Workbench startet die Clients oder hängt sich an laufende, Bots steuern deren Monarchen; FPS und Latenz der Clients stehen im Status (Test mit Fake-Client, Nachweis im Browser nach B-349).
- **AC-05** Ungültige Parameter und Kombinationen werden mit einer Zeile Grund abgelehnt (Test).
- **AC-06** `CLAUDE.md` und `docs/arbeitsweise.md` schreiben vor: Jeder autonome Testlauf geht über `sim_test`.

## Offene Fragen

keine

## Notizen

Anstoß 🧑 2026-10-07 vor weiteren Balancing- und Performance-Tests (BR1, LT1). `sim_run` und `replay_run` bleiben für Einzelanalysen.
