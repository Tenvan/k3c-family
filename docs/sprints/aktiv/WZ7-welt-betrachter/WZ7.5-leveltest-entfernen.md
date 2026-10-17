# WZ7.5 · leveltest.html entfernen

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz7/5-leveltest-entfernen
- **Abhängig von:** WZ7.4
- **Tickets:** B-460
- **Kriterien:** AC-07

## Ziel

`leveltest.html` samt Skript, Logik, Tests, Kacheln und Texten ist entfernt; Codemaps und Sprint-Übersicht sind nachgezogen (AC-07).

## Kontext

Sprint WZ7 (`docs/sprints/geplant/WZ7-welt-betrachter/README.md`), Ticket `docs/backlog/B-460-welt-betrachter-live-vorschau.md` (Spec freigegeben 2026-10-17). Maßgeblich sind die Anforderungen, Beispiele und Ausnahmefälle dort.

- Grenzfall wie WZ6.6/B-299: das Entfernen fällt in DEV, obwohl es auch Dateien unter `src/` betrifft.
- Zu entfernen: `leveltest.html`, `src/tools/leveltest.ts`, `levelView.ts`/`levelApi.ts` mit Tests (nach dem Umzug in WZ7.2), Kacheln in `src/tools/devTiles.ts` und `src/tools/testTiles.ts`, Texte `level.*`, Einträge in Codemaps (`src/tools/codemap.md`, `src/model/codemap.md`), `docs/sprints/README.md`.
- Verweise in `docs/backlog`, `docs/sprints/erledigt`, den Sprint-Ordnern WZ6/WZ7 und `.codemap/state.json` bleiben (Historie bzw. erzeugte Datei).

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `leveltest.html`, `src/tools/leveltest.ts`, `src/tools/levelView.ts`, `src/tools/levelApi.ts` und deren Tests (löschen)
- `src/tools/devTiles.ts`, `src/tools/testTiles.ts`, Textdateien mit `level.*`, zugehörige Tests (anpassen)
- `src/tools/codemap.md`, `src/model/codemap.md`, `docs/sprints/README.md`
- diese Session-Datei

## Nicht-Ziele

Kein Löschen von Historie (`docs/backlog`, `docs/sprints/erledigt`, Sprint-Ordner WZ6/WZ7, `.codemap/state.json`). Keine Änderung am Welt-Betrachter im Studio. Keine Änderung unter `src/scenes/`, `src/online/`.

## Schritte

1. `git grep -n leveltest` sichten, Liste der Treffer festhalten.
2. Dateien löschen, Kacheln und Texte entfernen, Tests anpassen (`tests/projectRules.test.ts` verlangt Seiten-Einträge).
3. Codemaps und `docs/sprints/README.md` nachziehen.
4. `git grep leveltest` mit den Ausnahmen aus AC-07 leer.
5. Prüfungen.

## Fertig, wenn

- [ ] AC-07: `git grep -n leveltest` ist leer bis auf `.codemap/state.json`, `docs/backlog`, `docs/sprints/erledigt` und die Sprint-Ordner WZ6/WZ7.
- [ ] AC-07: Dev-Seite zeigt keine Kachel „Level“, `testing.html` keinen Abschnitt „Level“ (Test).
- [ ] CI: `check_run` `task:check` und `task:build` grün.

## Prüfen

`task check`, `task build`; über k3c-dev `check_run` (mit `checkout`).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
