# LB1.2 · Review und Abnahme des Sprints LB1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/lb1
- **Abhängig von:** LB1.1
- **Tickets:** B-037
- **Kriterien:** alle

## Ziel

Der Sprint LB1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, abgeschlossen und der PR des Sprints ist geöffnet.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/scenes/LobbyScene.ts`, `src/scenes/lobbyLogic.ts` mit Tests, `src/core/saveStore.ts` und `src/core/texts.*.ts`. Besonders prüfen: Taste B ist nicht belegt; die Lobby bleibt ohne `api/saves` (Server weg, GitHub Pages) bedienbar; ein Spielstand-Eintrag sendet `create` mit `fresh: false` und nie `fresh: true` auf einen vorhandenen Namen; Namen aus der Server-Antwort werden nur angezeigt und als `save` gesendet (Format `^[a-z0-9-]{1,32}$` prüft der Server); `src/scenes` rechnet keine Spiel-Logik (`noSim.test.ts`). Nach dem Review ist B-037 erledigt, wenn alle vier Ticket-Kriterien belegt sind.

## Erlaubte Dateien

- `src/scenes/LobbyScene.ts`, `src/scenes/lobbyLogic.ts`, `src/core/saveStore.ts`, `src/core/texts.de.ts`, `src/core/texts.en.ts` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Umbau von Produktionscode; Anlegen-Dialog (K5).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/lb1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis für AC-01 (B-037/AC-01 bis AC-04) im Ergebnis von LB1.1 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag (`Version: v… vorgeschlagen (Grund)`). B-037 auf erledigt setzen und nach `docs/backlog/archiv/` verschieben, Index `docs/backlog/README.md` anpassen.
5. Sprint nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` und Projekt `docs/projekte/BED-*.md` anpassen, committen, `git merge origin/develop`, pushen und den einen PR des Sprints öffnen (Branch `sprint/lb1`, Ziel `develop`).

## Fertig, wenn

- [x] AC-01 hat einen Nachweis im Ergebnis von LB1.1 oder ist mit Grund und Ticket verschoben.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt.
- [x] `task check` grün; PR des Sprints ist geöffnet.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Review des Diffs `origin/develop...sprint/lb1` durch einen eigenen Agenten (code-reviewer, Sonnet): keine schweren Befunde. Geprüft: Taste B nicht belegt; `listSaves` fängt Netzfehler, Timeout, `!ok` und kaputtes JSON ab, die Lobby bleibt bedienbar; Spielstand-Einträge senden nur `fresh: false`; Namen nur als Text und als `save`; keine Spiel-Logik und kein `Math.random()` in `src/scenes`; Dateien und Funktionen im Budget.
- Befund mittel/leicht als Ticket B-382: Neuversuch greift, wenn ein gewählter Spielstand zufällig `params.save` heißt und inzwischen gelöscht ist; `NaN` aus ungültigem `savedAt` in der Sortierung.
- Nachweis AC-01 (B-037/AC-01 bis AC-04) im Ergebnis von LB1.1 geprüft. B-037 erledigt und archiviert, Sprint nach `erledigt/`.
- `task check` grün; Planung von Hand, weil k3c-dev die Repo-Wurzel bediente (B-275).
