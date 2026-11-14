# WR2.3 · Doku, Serena, Codemap-Stand und Backup-Skript nachziehen

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
