# SP09 · INF · Aufräumen: TS-Simulation und Node-Server löschen

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-032, B-049, B-078
- **Start-Commit:** bee8e44
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („SP09 freigegeben“), Revision 2

## Ausgangslage

Nach SP08 liegen TS-Simulation und Node-Server noch im Repo, werden aber nicht mehr gebraucht. Der Client importiert aber
noch Typen und Daten aus `src/world/` (`World`, `GameEvent`, `BIOMES`, `ECONOMY`, `SaveGame`, `LevelLayout`, `CycleInfo`),
und `task dev`, `task serve`, der k3c-dev-Dienst „Heimnetz“, das Release-Paket und die Pages-Auslieferung hängen am Node-Server.

## Ziel

Es gibt nur noch eine Engine. Am Ende sichtbar: Release `v0.2.0`, CI ohne Node-Server, `task dev` spricht mit dem Go-Server.

## Beteiligte und Zielgruppen

Entwickler; Spieler bekommen Release `v0.2.0`; 🧑 gibt die Spec frei und entscheidet über den Tag-Push.

## Anforderungen

B-032 › Anforderungen (Entscheidung 🧑 2026-10-01: Spiel-Kacheln auf Pages **erklären**, nicht ausblenden),
B-078 › Anforderungen. Sprint-eigen: `src/world/`, `src/online/room.ts`, `src/online/wsServer.ts`, `server/*.mjs` und
`vite.server.config.ts` sind gelöscht; der Client hält seine Typen und Daten in `src/model/`; CI, Release, README und
`CLAUDE.md` passen dazu.

## Nicht-Ziele

Neue Funktionen. Kommentare in `engine/` („Port von src/world/…“) bleiben als Herkunftsnachweis. `testdata/golden/` bleibt
(Go liest es); die TS-Erzeugung entfällt, die Dateien gelten damit als eingefroren.

## Regeln und Einschränkungen

- Release-Tag `v0.2.0` nach `docs/arbeitsweise.md` › Entscheidungen und Versionen.
- Domäne: Grenzfall „Alt-Engine löschen“ in `docs/arbeitsweise.md` › Domänen (B-049, Entscheidung 🧑 2026-10-01).
- Reihenfolge: erst den Client von `src/world/` lösen (SP09.1), dann umstellen und löschen (SP09.2); nach jeder Session ist `task check` grün.

## Beispiele

`git grep "world/" -- src` → keine Treffer, CI grün. `task dev` + `task start` → `http://<LAN-IP>:5173/game.html` verbindet sich mit dem Go-Server.

## Ausnahme- und Fehlerfälle

- Ein Verweis auf gelöschte Dateien bleibt → Build oder Test scheitert und wird behoben.
- Go-Server läuft nicht, `task dev` offen → Vite meldet den Proxy-Fehler im Terminal, der Browser zeigt „Server nicht erreichbar“.
- Pages (kein Server) → Spiel-Kacheln deaktiviert mit Hinweis, `game.html` direkt aufgerufen zeigt einen Hinweis statt eines Fehlers.

## Akzeptanzkriterien

- **AC-01** TS-Simulation und Node-Server sind gelöscht, die CI ist grün.
- **AC-02** README und `CLAUDE.md` beschreiben nur noch den Go-Server.
- **AC-03** GitHub Pages zeigt nur, was ohne Server geht (B-032/AC-01, B-032/AC-02).
- **AC-04** Release `v0.2.0` ist getaggt (Tag auf dem Commit der Abnahme); der Push des Tags bleibt bei 🧑.
- **AC-05** Der Client importiert nichts aus `src/world/`; seine Typen und Daten liegen in `src/model/`, ein Test hält das fest.
- **AC-06** `task dev` leitet `/api` und `/ws` an den Go-Server (Port 8080) weiter; `task start` und der k3c-dev-Dienst „Heimnetz“ starten den Go-Server (B-078/AC-01).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP09.1 | `SP09.1-client-model.md` | Umsetzung | autonom | fertig |
| SP09.2 | `SP09.2-umstellen-loeschen.md` | Umsetzung | autonom | fertig |
| SP09.3 | `SP09.3-pages-doku.md` | Umsetzung | autonom | fertig |
| SP09.4 | `SP09.4-review-release.md` | Review | autonom | fertig |

## Abnahme

Review 2026-10-01 (SP09.4): `task check`, `task check:go`, `task check:dev` grün, Diff `bee8e44..main` gelesen, keine schweren Befunde; `testdata/` unverändert.
AC-01 und AC-06: SP09.2 (Löschliste leer, Proxy-Probe `welcome`); AC-05: SP09.1 (`noSim.test.ts`); AC-02 und AC-03: SP09.3 (Tests in `serverCheck.test.ts`, Browser nicht geprüft); AC-04: lokaler Tag `v0.2.0`, Push offen bei 🧑.
Neue Tickets: keine. B-032, B-049, B-078 erledigt.
