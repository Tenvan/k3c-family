# B-821 · Version in LIZENZEN.txt und EXE können im Worktree auseinanderlaufen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG19
- **Projekt:** NAT
- **Erstellt:** 2026-11-12
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

`native/Taskfile.yml` (`game:build`, sources Zeile 69) beobachtet `../../.git/logs/HEAD`, um nach einem Commit neu zu bauen. Im Worktree ist `.git` eine Datei, der Pfad fehlt; die EXE wird nach einem Commit nicht neu gebaut und behält die alte Version (`-ldflags -X menu.Version`). `game:paket` schreibt `LIZENZEN.txt` dagegen immer mit der aktuellen `GAME_VERSION` (Zeile 123). Folge: Version in `LIZENZEN.txt` und im Hauptmenü der EXE können im Worktree auseinanderlaufen. Befund aus dem Review NG13.6.

## Ziel

EXE und `LIZENZEN.txt` eines Pakets tragen immer dieselbe Version, im Hauptcheckout wie im Worktree.

## Beteiligte und Zielgruppen

Claude und 🧑 Ralf beim Bauen der Pakete für den Mini-PC.

## Anforderungen

- `game:build` baut neu, wenn sich die Version (`git describe`) ändert, auch im Worktree.
- `game:paket` nimmt für EXE und `LIZENZEN.txt` dieselbe Versionsangabe.

## Nicht-Ziele

Neues Versionsschema, Release-Prozess.

## Regeln und Einschränkungen

Klasse A (Schuld, CLI, ohne Spielregeln, Protokoll, Spielstand-Format, neue Abhängigkeit). Tasks nur über `task`.

- **Bereich:** `native/Taskfile.yml`
- **Abhängig von:** NG13
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Im Worktree: Commit, dann `task native:game:paket` → Hauptmenü der EXE und `LIZENZEN.txt` nennen dieselbe Version.

## Ausnahme- und Fehlerfälle

Ohne Git (`git describe` schlägt fehl) → beide zeigen „unbekannt“.

## Akzeptanzkriterien

- **AC-01** Im Worktree nach einem neuen Commit: `task native:game:paket` baut die EXE neu; die Version in `LIZENZEN.txt` gleicht der eingebauten Version (`k3c-game.exe -version` oder Test der Task-Variablen).
- **AC-02** Im Hauptcheckout ohne Änderung kein unnötiger Neubau (`task native:game:build` meldet „up to date“).

## Offene Fragen

keine

## Notizen

Review NG13.6 (2026-11-12), `k3c-reviewer`. Idee: Version als `status`/`vars`-Vergleich statt Datei-Quelle, oder `git rev-parse --git-path logs/HEAD`.
