# WR2.3 · Doku, Serena, Codemap-Stand und Backup-Skript nachziehen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/wr2
- **Abhängig von:** WR2.1
- **Tickets:** B-898
- **Kriterien:** AC-05, AC-06, AC-07, AC-08

## Ziel

Doku, Serena-Memories, Codemap-Stand und Backup-Skript beschreiben nur noch das native Spiel.

## Kontext

B-898 › Anforderungen. `.claude/settings.json` ändert nur 🧑.

## Erlaubte Dateien

- `docs/arbeitsweise.md`, `docs/glossar.md`, `docs/protocol.md`, `docs/rules/**`, `docs/standards/**`
- `.serena/memories/**`, `.codemap/state.json`
- `scripts/**`

## Nicht-Ziele

- `.claude/settings.json`, Entscheidungen, archivierte Tickets, erledigte Sprints.

## Schritte

1. `docs/arbeitsweise.md` und Glossar nachziehen.
2. `docs/protocol.md` als Historie kennzeichnen, Regeln/Standards prüfen.
3. Serena-Memories und `.codemap/state.json` bereinigen.
4. `backup-saves.sh` auf `K3C_SAVES_DIR`/`dist/k3c-game/saves` umstellen, Task-Beschreibung anpassen.

## Fertig, wenn

- AC-05, AC-06, AC-07 erfüllt, `task check:diff` grün.

## Prüfen

- `check_run task:check:diff` (Worktree); `grep` aus B-898/AC-01.

## Ergebnis

- `docs/arbeitsweise.md`: Domänen SRV, CLI, PLAT, INF, DEV auf natives Spiel und Engine-CLI umgestellt; Protokoll-Grenzfall und Sperre `protokoll` als Historie; Abschnitt „Alt-Engine löschen“ entfernt; Schichtgrenzen ohne `engine/net` und `src/`; `task dev` und `k3c-load` als entfallen; Release-Checkliste ohne Pi-Image, Version und Lizenzen im Menü des nativen Spiels.
- `docs/glossar.md`: zehn Begriffe des Online-Spiels (Bot-Feed, Landingpage, Home-Kombi, Latenz, Orakel, Protokoll, RTT, Shell, Vorhersage, Zeitleiste) als Historie markiert; Lauf-Modus und Unit nachgezogen.
- `docs/protocol.md`: Verweis auf die entfallenen Beispiele; `docs/rules/welten.md` › Weitere Welten beschreibt das Laden über `engine/room` (nur technischer Text, keine Regeländerung).
- `.serena/memories` (core, conventions, suggested_commands, tech_stack) neu auf natives Spiel; `.codemap/state.json` ohne entfernte Pfade (485 Dateien, 69 Ordner) und mit aktuellen Mustern.
- `scripts/backup-saves.sh` sichert `K3C_SAVES_DIR`, sonst `dist/k3c-game/saves`, ohne Docker; `task ops:backup` beschreibt das. Gegen einen Temp-Ordner getestet (Kopie, Fehler bei fehlender Quelle).
- Nicht geändert: `.claude/settings.json` (🧑, B-898 › Notizen), Entscheidungen. `task check:diff` grün.
