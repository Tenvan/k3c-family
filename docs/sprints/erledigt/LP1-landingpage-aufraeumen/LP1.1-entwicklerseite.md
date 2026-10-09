# LP1.1 · Landingpage teilen, Entwicklerseite dev.html

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** live
- **Branch:** lp1/1-entwicklerseite
- **Abhängig von:** –
- **Tickets:** B-335
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Die Landingpage zeigt nur Weiterspielen, Neues Spiel, Online spielen, Lizenzen und eine kleine Kachel „Entwicklung“; die neue Seite `dev.html` sammelt alle Werkzeuge in den Abschnitten Entwicklung, Performance und Balancing.

## Kontext

- `src/landing/pages.ts`: `PAGES` (Kacheln mit `section: 'play' | 'test' | 'about'`, `primary`), `SECTIONS`; `src/landing/landing.ts` zeichnet je Abschnitt ein Raster (`.card`, `.card.primary`, CSS in `index.html`).
- `testing.html` + `src/tools/testing.ts` + `src/tools/testTiles.ts` (`nextFocus`, Controller-Abfrage): Vorbild für `dev.html`.
- `tests/projectRules.test.ts` prüft: jede `*.html` (außer `index.html`, `dm.html`) steht in `PAGES`, jede Kachel zeigt auf eine vorhandene Seite, jede Seite ruft `installPageChrome()` auf, kein `location.href =`/`assign`/`replace` außerhalb `src/core`/`src/landing`.
- `dm.html` läuft außerhalb der Shell (B-232) und wird mit `window.open('dm.html', '_blank')` geöffnet; schlägt das fehl (Pop-up-Sperre, Rückgabe `null`), zeigt die Seite einen Hinweis mit der Adresse.
- Beschlüsse in der Sprint-README › Anforderungen (Kachel-Zuordnung von 🧑).

## Erlaubte Dateien

- `src/landing/pages.ts`, `src/landing/pages.test.ts`, `src/landing/landing.ts`, `index.html` (nur CSS für die kleine Kachel)
- `dev.html`, `src/tools/dev.ts`, `src/tools/devTiles.ts`, `src/tools/devTiles.test.ts`
- `tests/projectRules.test.ts` (nur die Seiten-Prüfung, Grenzfall laut Sprint-README)
- `src/landing/codemap.md`, `src/tools/codemap.md`, `CLAUDE.md` (Abschnitt Struktur/Seiten, nur Hinweis auf die Entwicklerseite)
- Planungsdateien

## Nicht-Ziele

Texte übersetzen (B-215/PL2); Werkzeug-Seiten selbst ändern; Performance-Modi (B-334); `testing.html` umbauen.

## Schritte

1. `src/tools/devTiles.ts`: Kacheln der Entwicklerseite als Daten (`DEV_SECTIONS` mit Entwicklung, Performance, Balancing; Kachel mit `title`, `description`, `icon`, `href`, optional `external: true` für `dm.html`), Werte aus den bisherigen `PAGES`-Einträgen übernehmen. Test `devTiles.test.ts`: alle bisherigen Werkzeug-Seiten plus `dm.html` enthalten, jede genau einmal, Balancing leer.
2. `src/landing/pages.ts`: `PAGES` nur noch Weiterspielen, Neues Spiel, Online spielen, Lizenzen und die kleine Kachel „Entwicklung“ (`dev.html`, Feld `small: true`); Abschnitt „Tests & Werkzeuge“ entfällt. `pages.test.ts`: genau diese Kacheln, genau eine kleine.
3. `src/landing/landing.ts` + CSS in `index.html`: kleine Kachel (z. B. `.card.small`, eine Spalte, kleinere Schrift, gedämpft).
4. `dev.html` + `src/tools/dev.ts` nach dem Muster von `testing.html`: `installPageChrome()`, Abschnitte mit Kacheln, Balancing mit Hinweis „noch keine Aufrufe“, `openPage()` für Seiten, `window.open` für `dm.html` mit Hinweis bei Sperre, Controller-Fokus mit `nextFocus`.
5. `tests/projectRules.test.ts`: Seiten-Prüfung auf „in `PAGES` oder in den Kacheln der Entwicklerseite“ erweitern (beide Richtungen).
6. Codemaps und `CLAUDE.md` (Seiten-Regel Punkt 2: „in `src/landing/pages.ts` oder `src/tools/devTiles.ts` eintragen“) nachziehen.
7. `task check`.

## Fertig, wenn

- [x] AC-01: `pages.test.ts` prüft die vier Spieler-Kacheln und genau eine kleine Kachel `dev.html`.
- [x] AC-02: `devTiles.test.ts` prüft Abschnitte und Vollständigkeit (alle früheren Werkzeug-Kacheln plus `dm.html`).
- [x] AC-03: `task check` grün; `projectRules.test.ts` deckt `dev.html` ab.

## Prüfen

```bash
task check
```

## Ergebnis

2026-10-07, Agent (Claude Opus 5.5), Branch `sprint/lp1`.

- **AC-01 geprüft:** `src/landing/pages.ts` enthält nur Weiterspielen, Neues Spiel, Online spielen, Lizenzen & Danksagung und die kleine Kachel „Entwicklung“ (`dev.html`, `small: true`, CSS `.card.small` in `index.html`); Test in `pages.test.ts`. Browser-Pane: Landingpage zeigt genau diese fünf Kacheln.
- **AC-02 geprüft:** `src/tools/devTiles.ts` (`DEV_SECTIONS`: Entwicklung mit 8 Kacheln inkl. Dungeon Master, Performance mit Monitor, Balancing leer mit „Noch keine Aufrufe.“), Seite `dev.html` + `src/tools/dev.ts`; Test `devTiles.test.ts` (4 Tests). Browser-Pane: „Entwicklung“ öffnet `dev.html` im Rahmen, drei Abschnitte, 9 Kacheln; „Monitor“ öffnet über `openPage()`, „Dungeon Master“ ruft `window.open(…/dm.html, '_blank')`; keine Konsolenfehler.
- **AC-03 geprüft:** `task check` grün; `tests/projectRules.test.ts` verlangt jede Seite in `PAGES` oder `DEV_TILES` und prüft Kachel-Ziele gegen alle `*.html` (wegen `dm.html`).
- **Abweichung Erlaubte Dateien:** zusätzlich `src/landing/serverCheck.test.ts` (Erwartung der Nicht-Spiel-Kacheln: jetzt `lizenzen.html`, `dev.html`) und `src/tools/testTiles.test.ts` (Level-Betrachter steht jetzt auf der Entwicklerseite) angepasst; beides Tests derselben Domäne, die sonst an der gewollten Änderung scheitern.
- Codemaps (`src/landing`, `src/tools`) und `CLAUDE.md` (Seiten-Regel Punkt 2) nachgezogen. Keine neuen Tickets.
