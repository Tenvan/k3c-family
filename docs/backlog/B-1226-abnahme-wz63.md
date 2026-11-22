# B-1226 · Abnahme WZ63: Studio startet allein und bearbeitet eine Welt

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WZ63
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint WZ63 trennt das Studio als `k3c-studio.exe` von der Workbench. Automatische Prüfungen sind grün, die Bedienung am echten Fenster hat noch niemand gesehen.

## Ziel

Ralf bestätigt am echten Fenster, dass das Studio allein funktioniert.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Siehe Akzeptanzkriterien (manuelle Abnahme durch 🧑).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** 🧑 `bin/k3c-studio.exe` startet ohne laufende Workbench (`task k3c-studio:build`, dann EXE starten); Icon (Insel) in Taskleiste und Datei sichtbar.
- **AC-02** 🧑 Welten-Editor, Assets (4 Reiter) und Hub-Planer zeigen echte Daten; eine Welt lässt sich anlegen und ändern (Oberfläche oder MCP `world_create`/`world_set`).
- **AC-03** 🧑 MCP-Server `k3c-studio` (Port 5181) ist in Claude Code verbunden und listet nur Studio-Tools; die Workbench listet sie nicht.
- **AC-04** 🧑 Workbench-Planungsseite sieht unverändert aus (CSS-Klassen `pl-*` aus `planning.css` und `planer.css` wurden getrennt); Balancing steht als letzte Reiterseite.
- **AC-05** 🧑 Workbench-Icon (Zahnrad mit Krone) nach `task k3c-workbench:build` sichtbar.

## Offene Fragen

`.claude/settings.json` Allow-Liste nennt noch `mcp__k3c-workbench__level_generate/assets_*` (5 Treffer); auf `mcp__k3c-studio__…` umstellen, `assets_catalog` streichen (🧑 entscheidet, Permission-Konfiguration). Icon-Einbettung in `k3c-game.exe` braucht `winresource` (neue Abhängigkeit).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
