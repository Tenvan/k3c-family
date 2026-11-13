# B-835 · cmd/k3c-server/k3c-server.exe liegt als 14-MB-Binary im Repo

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ48
- **Projekt:** PAP
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gefunden im Review CU1.6: `cmd/k3c-server/k3c-server.exe` (14,6 MB) ist seit #427 (WZ25, 88cd41257) eingecheckt, obwohl `.gitignore` `*.exe` ausschließt; die Datei enthält alte Pfad-Strings und taucht in der Suche aus CU1/AC-01 auf.

## Ziel

Kein Binary unter `cmd/` im Repo.

## Beteiligte und Zielgruppen

Claude.

## Anforderungen

`git rm --cached cmd/k3c-server/k3c-server.exe`; prüfen, dass kein Task oder Skript die Datei an diesem Pfad erwartet (Builds legen nach `bin/`).

## Nicht-Ziele

Git-Historie umschreiben.

## Regeln und Einschränkungen

Klasse A.

- **Bereich:** `cmd/k3c-server/k3c-server.exe`
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

–
