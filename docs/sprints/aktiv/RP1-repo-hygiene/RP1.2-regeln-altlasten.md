# RP1.2 · Regeln und Altlasten

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** rp1/2-regeln-altlasten
- **Abhängig von:** RP1.1
- **Tickets:** B-288, B-094, B-058
- **Kriterien:** AC-02, AC-03, AC-04

## Ziel

Die Regeln stimmen mit den Beschlüssen überein (Spielstand ohne Rücksicht auf alte Stände, Execution-Policy nur als zweite Wahl begründet) und im Repo liegen keine Alt-Binaries und keine `npm run`-Verweise außerhalb der Ausnahmen mehr.

## Kontext

Spec: B-288, B-094, B-058 (je Anforderungen) und RP1/AC-02 bis AC-04. B-288: Beschluss 🧑 2026-10-05 (bis auf Widerruf keine Rücksicht auf alte Spielstände); `docs/arbeitsweise.md` › „Spielstand-Format ändern“ und Release-Zeile „Spielstand-Migration“ verlangen noch Migration und Fixtures je Version; Migrationscode in `engine/sim/` bleibt unangetastet (SIM, offene Frage in B-288). B-094: Die Binärdateien `k3c-server`, `k3c-server.exe`, `k3c-tui.exe` sind seit 2026-10-18 nicht mehr versioniert und in `.gitignore` eingetragen; offen ist der Nachweis `git grep -n "npm run" -- . ':!tools' ':!docs/backlog' ':!docs/sprints'` und `task check:dev`; die `scripts` in den Frontend-`package.json` bleiben (Entscheidung 🧑 2026-10-18). B-058: `requirements.md` rät unter „Hinweise“ zuerst zu `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned`; die Antwort von 🧑 steht aus, bis dahin gilt die vorsichtige Vorgabe des Tickets (zuerst `npm.cmd`, Policy-Änderung nur als zweite Wahl mit Begründung, im Ergebnis als „vorläufig“ vermerkt).

## Erlaubte Dateien

- `docs/arbeitsweise.md` (Abschnitte „Spielstand-Format ändern“, „Release“)
- `requirements.md` (Abschnitt Hinweise)
- `.gitignore`, Dateien mit `npm run`-Treffern außerhalb von `tools/`, `docs/backlog/`, `docs/sprints/`
- Planungsdateien dieses Sprints, B-288, B-094, B-058

## Nicht-Ziele

Kein Löschen von Migrationscode oder Fixtures in `engine/sim/` und `testdata/saves/`; keine Änderung an den `scripts` der Frontends; Git-Historie bleibt.

## Schritte

1. `docs/arbeitsweise.md` › „Spielstand-Format ändern“: Beschluss 2026-10-05 voranstellen (bis auf Widerruf keine Migration, kein neues Fixture nur für alte Stände); die Release-Zeile „Spielstand-Migration“ entsprechend anpassen.
2. `requirements.md`: Reihenfolge ändern: zuerst `npm.cmd` (oder Git Bash), `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` nur als zweite Wahl mit Begründung (dauerhaft, gilt für alle Skripte des Kontos).
3. `git grep -n "npm run" -- . ':!tools' ':!docs/backlog' ':!docs/sprints'` ausführen; Treffer bereinigen. `git ls-files k3c-server k3c-server.exe k3c-tui.exe` muss leer sein.
4. `task check:dev` und `task check` ausführen.

## Fertig, wenn

- [ ] AC-02: `docs/arbeitsweise.md` › „Spielstand-Format ändern“ und „Release“ nennen den Beschluss vom 2026-10-05 (B-288/AC-01).
- [ ] AC-03: `git ls-files k3c-server k3c-server.exe` ist leer, `git grep -n "npm run" -- . ':!tools' ':!docs/backlog' ':!docs/sprints'` ohne Treffer, `task check:dev` grün (B-094/AC-01, B-094/AC-02).
- [ ] AC-04: `requirements.md` nennt zuerst den Weg ohne Systemänderung, die Policy-Änderung nur mit Begründung (B-058/AC-01).
- [ ] `task check` grün.

## Prüfen

```bash
git ls-files k3c-server k3c-server.exe k3c-tui.exe
git grep -n "npm run" -- . ':!tools' ':!docs/backlog' ':!docs/sprints'
task check:dev
task check
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
