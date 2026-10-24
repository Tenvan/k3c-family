# WZ31 · DEV · Workbench und Studio als Web-Client: Server starten, im Browser arbeiten

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-614
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-10-24

## Ausgangslage

Workbench und Studio starten heute als Fenster (EXE, `window`) oder als `dev` mit Vite. Ein Start „nur Server, Oberfläche im Browser“ ohne Vite fehlt (B-614 › Ausgangslage).

## Ziel

Ralf startet Workbench und Studio je mit einem Task als Server und arbeitet im Browser. Am Ende sichtbar: `task k3c-workbench:web` öffnet `http://127.0.0.1:8181/`, `task k3c-studio:web` öffnet `http://127.0.0.1:8182/studio/`.

## Beteiligte und Zielgruppen

🧑 Ralf (Nutzer, Freigabe, Abnahme); Claude-Sessions über den MCP-Server.

## Anforderungen

B-614 › Anforderungen.

## Nicht-Ziele

B-614 › Nicht-Ziele (EXE-Builds unverändert, kein Hot Reload, kein LAN).

## Regeln und Einschränkungen

B-614 › Regeln und Einschränkungen. Keine Dateien der parallelen Sprints WZ28, WZ30, BD9 anfassen.

## Beispiele

B-614 › Beispiele.

## Ausnahme- und Fehlerfälle

B-614 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Workbench läuft als Server, Oberfläche im Browser (`B-614/AC-01`).
- **AC-02** Studio läuft als Server, Oberfläche im Browser (`B-614/AC-02`).
- **AC-03** Feste EXE-Pfade, Frontend-Bau nur bei Änderungen (`B-614/AC-03`).
- **AC-04** `task --list` und `CLAUDE.md` nennen die Tasks mit Ports (`B-614/AC-04`).
- **AC-05** EXE-Builds, `window`, `dev` unverändert (`B-614/AC-05`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ31.1 | `WZ31.1-web-tasks.md` | Umsetzung | autonom | fertig |
| WZ31.2 | `WZ31.2-review.md` | Review | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
