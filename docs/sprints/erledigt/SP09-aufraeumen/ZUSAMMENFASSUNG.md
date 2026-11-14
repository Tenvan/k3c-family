# SP09 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Aufräumen: TS-Simulation und Node-Server löschen
- **Projekt:** –

## Ziel

Es gibt nur noch eine Engine. Am Ende sichtbar: Release `v0.2.0`, CI ohne Node-Server, `task dev` spricht mit dem Go-Server.

## Ergebnis

Review 2026-10-01 (SP09.4): `task check`, `task check:go`, `task check:dev` grün, Diff `bee8e44..main` gelesen, keine schweren Befunde; `testdata/` unverändert. AC-01 und AC-06: SP09.2 (Löschliste leer, Proxy-Probe `welcome`); AC-05: SP09.1 (`noSim.test.ts`); AC-02 und AC-03: SP09.3 (Tests in `serverCheck.test.ts`, Browser nicht geprüft); AC-04: lokaler Tag `v0.2.0`, Push offen bei 🧑. Neue Tickets: keine. B-032, B-049, B-078 erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | TS-Simulation und Node-Server sind gelöscht, die CI ist grün. |
| AC-02 | geprüft | README und `CLAUDE.md` beschreiben nur noch den Go-Server. |
| AC-03 | – | GitHub Pages zeigt nur, was ohne Server geht (B-032/AC-01, B-032/AC-02). |
| AC-04 | – | Release `v0.2.0` ist getaggt (Tag auf dem Commit der Abnahme); der Push des Tags bleibt be … |
| AC-05 | – | Der Client importiert nichts aus `src/world/`; seine Typen und Daten liegen in `src/model/ … |
| AC-06 | geprüft | `task dev` leitet `/api` und `/ws` an den Go-Server (Port 8080) weiter; `task start` und d … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-032 | erledigt | AC-03 – | GitHub Pages zeigt nur, was ohne Server geht |
| B-049 | erledigt | – | SP09 bleibt in einer Domäne oder hat einen erlaubten Grenzfall |
| B-078 | erledigt | AC-06 geprüft | `task dev` leitet `/ws` an den Go-Server weiter |
