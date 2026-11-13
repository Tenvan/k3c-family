# B-835 · cmd/k3c-server/k3c-server.exe liegt als 14-MB-Binary im Repo

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ48
- **Projekt:** PAP
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13

## Ausgangslage

Gefunden im Review CU1.6: `cmd/k3c-server/k3c-server.exe` (14,6 MB) ist seit #427 (WZ25, 88cd41257) eingecheckt, obwohl `.gitignore` `*.exe` ausschließt; die Datei enthält alte Pfad-Strings und taucht in der Suche aus CU1/AC-01 auf.

## Ziel

Kein Binary unter `cmd/` im Repo.

## Beteiligte und Zielgruppen

Claude (Umsetzung); 🧑 liest das Ergebnis.

## Anforderungen

`git rm --cached cmd/k3c-server/k3c-server.exe`; prüfen, dass kein Task oder Skript die Datei an diesem Pfad erwartet (Builds legen nach `bin/`).

## Nicht-Ziele

Git-Historie umschreiben.

## Regeln und Einschränkungen

Klasse A (Schuld, Domäne INF, ohne Spielregeln, Protokoll, Spielstand, neue Abhängigkeit).

- **Bereich:** `cmd/k3c-server/k3c-server.exe`, `tools/k3c-starter/k3c-starter.exe`, `.gitignore`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`git ls-files '*.exe'` → leer.

## Ausnahme- und Fehlerfälle

Erwartet ein Werkzeug die Datei doch, Pfad auf `bin/` umstellen.

## Akzeptanzkriterien

- **AC-01** `git ls-files '*.exe'` ist leer, `task check:diff` grün.

## Offene Fragen

keine

## Notizen

WZ48.3 (2026-11-13): `git ls-files '*.exe'` fand zwei Dateien, neben `cmd/k3c-server/k3c-server.exe` auch `tools/k3c-starter/k3c-starter.exe`; beide entfernt (AC-01 verlangt eine leere Liste). Kein Task, Workflow, Dockerfile oder Code erwartet sie an diesem Pfad (Builds legen nach `bin/`). `.gitignore` schließt `*.exe` schon aus (`git check-ignore` greift für beide Pfade); hinein kamen sie vermutlich per `git add -f` (ungeprüft).
