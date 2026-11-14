# WZ31 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Workbench und Studio als Web-Client: Server starten, im Browser arbeiten
- **Projekt:** WZG

## Ziel

Ralf startet Workbench und Studio je mit einem Task als Server und arbeitet im Browser. Am Ende sichtbar: `task k3c-workbench:web` öffnet `http://127.0.0.1:8181/`, `task k3c-studio:web` öffnet `http://127.0.0.1:8182/studio/`.

## Ergebnis

2026-10-31: AC-01 bis AC-05 von Ralf (🧑) abgenommen (Review und Alltagsnutzung beider Web-Starts), im Chat bestätigt; Claude hat die Prüfung nicht selbst gesehen. Nachweise der Umsetzung: WZ31.1 › Ergebnis. Keine Befunde genannt, keine neuen Tickets. Sprint erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Workbench läuft als Server, Oberfläche im Browser (`B-614/AC-01`). |
| AC-02 | – | Studio läuft als Server, Oberfläche im Browser (`B-614/AC-02`). |
| AC-03 | – | Feste EXE-Pfade, Frontend-Bau nur bei Änderungen (`B-614/AC-03`). |
| AC-04 | – | `task --list` und `CLAUDE.md` nennen die Tasks mit Ports (`B-614/AC-04`). |
| AC-05 | – | EXE-Builds, `window`, `dev` unverändert (`B-614/AC-05`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-614 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 – | Workbench und Studio starten als Server, gearbeitet wird im Browser |
