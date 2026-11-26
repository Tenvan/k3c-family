# B-1007 · task ops:aufraeumen räumt die EXE entfernter Programme aus bin/ ab

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WN3
- **Projekt:** WZN
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

In `bin/` (nicht eingecheckt, `.gitignore` `/bin/`) liegen noch EXE von Programmen, die es nicht mehr gibt (B-850, B-887): `k3c-server.exe`, `k3c-load.exe`, `k3c-tui.exe`, `k3c-starter.exe`, `k3c-workbench-web.exe` und die Sicherung `k3c-workbench-web.exe~` (Stand 2026-11-15, zusammen rund 110 MiB). `task ops:aufraeumen` (`scripts/Taskfile.yml`, `tools/cleanup`) räumt bisher nur lokale Branches und Worktrees ab (B-302).

## Ziel

`bin/` enthält nur EXE von Programmen, die das Repo noch baut; alte EXE verschwinden mit einem Befehl statt von Hand.

## Beteiligte und Zielgruppen

🧑 und Agenten am Entwicklungsrechner; 🧑 gibt das Löschen frei.

## Anforderungen

- `tools/cleanup` kennt eine feste Liste entfernter Programme (`k3c-server`, `k3c-load`, `k3c-tui`, `k3c-starter`, `k3c-workbench-web`) und löscht in `bin/` nur deren Dateien (`<name>.exe`, `<name>.exe~`, `<name>` ohne Endung).
- `--dry` zeigt je Datei Name und Größe und löscht nichts.
- Alles andere in `bin/` bleibt unberührt (`k3c-workbench.exe`, `k3c-sim.exe`, `soundatlas.exe`, `aufraeumen.exe`).
- Eine gesperrte Datei (läuft noch) wird mit Grund gemeldet, der Lauf endet nicht mit Fehler.
- Log-Zeilen mit Emoji (🧹 aufgeräumt).

## Nicht-Ziele

`dist/`, `native/k3c-game/bin/` und Worktrees aufräumen; EXE noch vorhandener Programme löschen; Platzhalter-Erkennung „unbekannte EXE“.

## Regeln und Einschränkungen

Kein Löschen außerhalb von `bin/` der Repo-Wurzel; Pfade nur aus der festen Liste (kein Glob aus Eingaben). Pfad-Code: Reviewer-Agent vor dem Commit. Das tatsächliche Löschen auf Ralfs Rechner läuft erst nach seinem Ja (Lauf ohne `--dry`).

- **Bereich:** `tools/cleanup/**`, `scripts/Taskfile.yml`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

`task ops:aufraeumen -- --dry` → Liste „bin/k3c-server.exe 15,0 MiB … würde gelöscht“; ohne `--dry` sind die fünf Programme weg, `bin/k3c-workbench.exe` bleibt.

## Ausnahme- und Fehlerfälle

`bin/` fehlt → nichts zu tun, kein Fehler. Datei gesperrt → Meldung mit Grund, Rest läuft weiter. Name aus der Liste ist ein Ordner → nicht anfassen.

## Akzeptanzkriterien

- **AC-01** Test in `tools/cleanup`: In einem Temp-Ordner mit den fünf alten Namen, `.exe~` und `k3c-workbench.exe` löscht der Lauf genau die alten und lässt `k3c-workbench.exe` stehen.
- **AC-02** Test: Mit `--dry` bleibt jede Datei, die Ausgabe nennt jede alte Datei.
- **AC-03** `go test ./tools/cleanup/...` grün; Beschreibung des Tasks `ops:aufraeumen` nennt `bin/`.
- **AC-04** Nach Ralfs Ja: `task ops:aufraeumen` auf seinem Rechner, `ls bin/` zeigt keine der fünf alten EXE (Nachweis im Ergebnis).

## Offene Fragen

- Feste Liste oder alles löschen, was kein Taskfile mehr baut? → Default: feste Liste, weil sie nichts Unerwartetes trifft (vorläufig, 🧑 bestätigt).
- Darf der Agent den Lauf ohne `--dry` in der Session selbst starten? → Default: nein, nur `--dry`; das Löschen startet 🧑 oder gibt es im Chat frei (vorläufig, 🧑 bestätigt).

## Notizen

Gefunden beim Planen 2026-11-15 (`ls bin/`). Klasse A wäre möglich (Schuld, INF), bleibt aber `Entwurf`, weil gelöscht wird.
