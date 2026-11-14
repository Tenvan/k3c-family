# B-898 · Prozess-Doku, Rechte und Mocks nennen keinen Spielserver, keinen Browser-Client und keine Xbox-Seiten mehr

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WR2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WR1 hat CLAUDE.md, README, requirements und Codemaps nachgezogen. Noch nicht: `docs/arbeitsweise.md` und weitere Doku (Regeln, Entscheidungen, Glossar) mit Verweisen auf Spielserver, `task dev`/`task start`, Browser-Pane, Xbox-Edge; `.claude/settings.json` gibt noch `mcp__k3c-workbench__server_status`, `rooms_list`, `room_snapshot` frei (Rechte-Datei, ändert nur 🧑); `.serena/memories/*` und `.codemap/state.json` nennen entfernte Ordner; `scripts/backup-saves.sh` und `task ops:backup` sind für das Docker-Volume des Pi gebaut; `testdata/protocol` und `docs/protocol.md` sind nur noch Referenz.

## Ziel

Doku und Konfiguration beschreiben nur noch das native Spiel.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `docs/arbeitsweise.md`, Glossar und Regeln auf Verweise prüfen und nachziehen.
- 🧑: drei veraltete Rechte in `.claude/settings.json` entfernen.
- `scripts/backup-saves.sh`/`ops:backup` auf Spielstände des nativen Spiels umstellen oder entfernen.

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

- **AC-01** `grep -r "task start\|task serve\|k3c-server" docs/arbeitsweise.md docs/glossar.md` findet nichts mehr außer Historie.
- **AC-02** `.claude/settings.json` nennt keine entfernten MCP-Tools.

## Offene Fragen

Backup-Skript umstellen oder entfernen? Entscheidet 🧑.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
