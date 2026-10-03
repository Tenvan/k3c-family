# F5 · INF · Doku-Drift, Version und Landing-Kacheln

- **Status:** aktiv
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-141, B-079
- **Start-Commit:** 605f467
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-141, B-079

## Ausgangslage

Die vier `docs/rules/ist-*.md` sind überholt, der Satz zu Entscheidung 003 in `CLAUDE.md` nennt „Umsetzung offen“. Die Version steht nur im token-geschützten `/api/status`; die Landingpage zeigt keine Version. Die Kacheln in `src/landing/pages.ts` benutzen Parameter (`?continue=1`, `?seed=…`, `?depth=…`, `?online=…`), die die Lobby nicht mehr kennt (`parseStartParams` in `src/scenes/lobbyLogic.ts`: `autostart`, `fresh`, `save`, `mock`, `room`).

## Ziel

Doku und `CLAUDE.md` zeigen den heutigen Stand, die Landingpage nennt Server- und Client-Version, und jede Kachel führt in die Lobby statt in tote Parameter. Am Ende sichtbar: Fußzeile „Server <Version> · Client <Version>“ auf der Landingpage am TV, keine toten Kacheln.

## Beteiligte und Zielgruppen

Entwickler und Agenten (lesen `CLAUDE.md`), Betreiber des Pi, Spieler an der Xbox; 🧑 entscheidet, welche Dev-Kacheln bleiben (B-079).

## Anforderungen

B-141 und B-079 › Anforderungen.

## Nicht-Ziele

Release-Prozess und Tag-Konvention (B-170), Lobby selbst (B-037), Pi-Betrieb (F4, SP11).

## Regeln und Einschränkungen

Domäne INF. **Domänen-Ausnahme (Freigabe dieser Spec erlaubt sie, wie SP11):** F5.2 darf `engine/net/handler.go` (Feld `version` in `/api/health`), `Dockerfile` (Build-Argument), `index.html`, `src/landing/`, `src/env.d.ts` und die zugehörigen Tests ändern; F5.3 darf `src/landing/pages.ts` und einen Test dazu ändern. Seitenregeln aus `CLAUDE.md` bleiben (`installPageChrome()`, Eintrag in `src/landing/pages.ts`, kein `location` außer über `openPage()`/`goHome()`). Keine neue Abhängigkeit. Von diesem Rechner aus kein Build des Docker-Images.

## Beispiele

Server v0.3.0, Xbox hat noch den Client v0.2.0 im Cache → Fußzeile zeigt beide Versionen, die abweichende hervorgehoben. Kachel „Neues Spiel“ → Lobby (`game.html`) ohne unbekannte Parameter.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar → Fußzeile „Server – · Client <Version>“, keine Fehlermeldung. Entwicklungsbuild ohne Tag → Version `dev`.

## Akzeptanzkriterien

- **AC-01** `docs/rules/archiv/` enthält die vier `ist-*.md`; `docs/rules/` selbst keine; kein Verweis zeigt auf eine fehlende Datei (B-141/AC-01).
- **AC-02** Der Satz zur Spielstruktur in `CLAUDE.md` nennt keine „Umsetzung offen“ mehr (B-141/AC-02).
- **AC-03** `GET /api/health` liefert `ok: true` und `version` (B-141/AC-03).
- **AC-04** Die Fußzeile der Landingpage zeigt Server- und Client-Version und markiert Abweichung (B-141/AC-04).
- **AC-05** `.html` wird mit `no-cache`, JS und Assets mit `immutable` ausgeliefert, per Test belegt (B-141/AC-05).
- **AC-06** Jede Kachel mit Ziel `game.html` benutzt nur Parameter, die die Lobby kennt (B-079/AC-01).

## Offene Fragen

Entschieden am 2026-10-03: Es bleiben nur Kacheln mit Parametern, die die Lobby kennt (Neues Spiel, Online); Kacheln mit unbekannten Parametern (Höhle/Mine direkt, Zufallsseed) entfallen. Keine offene Frage.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| F5.1 | `F5.1-doku-drift.md` | Umsetzung | autonom | fertig |
| F5.2 | `F5.2-version-cache.md` | Umsetzung | autonom | fertig |
| F5.3 | `F5.3-landing-kacheln.md` | Umsetzung | autonom | in Arbeit |
| F5.4 | `F5.4-review.md` | Review | autonom | offen |

## Abnahme

–
