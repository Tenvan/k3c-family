# SP09 · INF · Aufräumen: TS-Simulation und Node-Server löschen

- **Status:** geplant
- **Domäne:** INF
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-032
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach SP08 liegen TS-Simulation und Node-Server noch im Repo, werden aber nicht mehr gebraucht.

## Ziel

Es gibt nur noch eine Engine. Am Ende sichtbar: Release `v0.2.0`, CI ohne Node-Server.

## Beteiligte und Zielgruppen

Entwickler; Spieler bekommen Release `v0.2.0`.

## Anforderungen

B-032 › Anforderungen. Sprint-eigen: `src/world/`, `src/online/room.ts`, `src/online/wsServer.ts`, `server/*.mjs` und `vite.server.config.ts` sind gelöscht; CI, README und `CLAUDE.md` passen dazu.

## Nicht-Ziele

Neue Funktionen.

## Regeln und Einschränkungen

Release-Tag `v0.2.0` nach `docs/arbeitsweise.md` › Entscheidungen und Versionen.

## Beispiele

`grep -rn "src/world" src` → keine Treffer, CI grün.

## Ausnahme- und Fehlerfälle

Ein Verweis auf gelöschte Dateien bleibt → Build oder Test scheitert und wird behoben.

## Akzeptanzkriterien

- **AC-01** TS-Simulation und Node-Server sind gelöscht, die CI ist grün.
- **AC-02** README und `CLAUDE.md` beschreiben nur noch den Go-Server.
- **AC-03** GitHub Pages zeigt nur, was ohne Server geht (B-032/AC-01, B-032/AC-02).
- **AC-04** Release `v0.2.0` ist getaggt.

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP09.1 `src/world/`, `src/online/room.ts`, `src/online/wsServer.ts`, `server/*.mjs`, `vite.server.config.ts` löschen; CI, README, CLAUDE.md, GitHub Pages (B-032) anpassen (AC-01, AC-02, AC-03).
- SP09.2 🔍 Review, Release `v0.2.0` (AC-04, alle).

## Abnahme

–
