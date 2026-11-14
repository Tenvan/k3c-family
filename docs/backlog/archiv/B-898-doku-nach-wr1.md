# B-898 · Prozess-Doku, Rechte und Mocks nennen keinen Spielserver, keinen Browser-Client und keine Xbox-Seiten mehr

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WR2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle; Entscheidung B-850)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WR1 hat CLAUDE.md, README, requirements und Codemaps nachgezogen. Noch nicht: `docs/arbeitsweise.md` und weitere Doku (Regeln, Entscheidungen, Glossar) mit Verweisen auf Spielserver, `task dev`/`task start`, Browser-Pane, Xbox-Edge; `.claude/settings.json` gibt noch `mcp__k3c-workbench__server_status`, `rooms_list`, `room_snapshot` frei (Rechte-Datei, ändert nur 🧑); `.serena/memories/*` und `.codemap/state.json` nennen entfernte Ordner; `scripts/backup-saves.sh` und `task ops:backup` sind für das Docker-Volume des Pi gebaut; `testdata/protocol` und `docs/protocol.md` sind nur noch Referenz.

## Ziel

Doku und Konfiguration beschreiben nur noch das native Spiel.

## Beteiligte und Zielgruppen

- Wer entwickelt: Claude (autonom) und Ralf; Agenten lesen `docs/arbeitsweise.md` in jeder Session.
- Entscheidet: 🧑 Ralf (Rechte-Datei `.claude/settings.json`).

## Anforderungen

- `docs/arbeitsweise.md` und `docs/glossar.md`: Verweise auf Spielserver, `task dev`/`task start`/`task serve`, Browser-Pane, Xbox-Edge, `engine/net`, `k3c-load`, `testdata/protocol` nachziehen oder als Historie kennzeichnen.
- Weitere Doku (`docs/rules`, `docs/standards`) auf solche Verweise prüfen und nachziehen; Entscheidungen (`docs/decisions/`) bleiben als Historie unverändert.
- `docs/protocol.md` als Historie kennzeichnen (Spielserver entfallen, B-850).
- `.serena/memories/*` und `.codemap/state.json` nennen keine entfernten Ordner.
- `scripts/backup-saves.sh`/`task ops:backup` sichern die Spielstände des nativen Spiels (`K3C_SAVES_DIR`, sonst `dist/k3c-game/saves`), ohne Docker.
- 🧑: drei veraltete Rechte in `.claude/settings.json` entfernen (`server_status`, `rooms_list`, `room_snapshot`); nicht Teil der Sessions.

## Nicht-Ziele

- `.claude/settings.json` (ändert nur 🧑).
- Archivierte Tickets, erledigte Sprints und Entscheidungen (Historie).
- `CLAUDE.md`, README, requirements, Codemaps (WR1).

## Regeln und Einschränkungen

- Entscheidung B-850 und 009; Glossar ist verbindlich, Begriffe dort zuerst.
- Prozess steht nur in `docs/arbeitsweise.md`.

- **Bereich:** `docs/arbeitsweise.md`, `docs/glossar.md`, `docs/protocol.md`, `docs/rules/**`, `docs/standards/**`, `.serena/memories/**`, `.codemap/state.json`, `scripts/**`
- **Abhängig von:** WR1
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

- Agent liest `docs/arbeitsweise.md` › Domänen → SRV nennt `engine/room`, `engine/store`, keinen `engine/net` und kein Docker.
- `task ops:backup -- E:/` → Kopie von `dist/k3c-game/saves` nach `E:/k3c-saves-<Datum>/`.

## Ausnahme- und Fehlerfälle

- Quelle fehlt → `backup-saves.sh` meldet Fehler mit Zeit auf stderr, Exit ≠ 0.

## Akzeptanzkriterien

- **AC-01** `grep -n "task start\|task serve\|k3c-server" docs/arbeitsweise.md docs/glossar.md` findet nichts außer Historie mit Ticket-Verweis.
- **AC-02** `scripts/backup-saves.sh` sichert die Spielstände des nativen Spiels ohne Docker; `task ops:backup` beschreibt das.
- **AC-03** `.serena/memories/*` und `.codemap/state.json` nennen keine von WR1 entfernten Ordner.

## Offene Fragen

keine. Backup-Skript wird umgestellt statt entfernt (autonom entschieden: geringer Aufwand, Sicherung bleibt möglich). Die Rechte in `.claude/settings.json` entfernt 🧑 Ralf.

## Notizen

- 🧑 Ralf: `mcp__k3c-workbench__server_status`, `rooms_list`, `room_snapshot` aus `.claude/settings.json` entfernen (Rechte-Datei, ändert nur Ralf).
