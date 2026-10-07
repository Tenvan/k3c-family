# LP1.1 · Landingpage teilen, Entwicklerseite dev.html

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
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

- [ ] AC-01: `pages.test.ts` prüft die vier Spieler-Kacheln und genau eine kleine Kachel `dev.html`.
- [ ] AC-02: `devTiles.test.ts` prüft Abschnitte und Vollständigkeit (alle früheren Werkzeug-Kacheln plus `dm.html`).
- [ ] AC-03: `task check` grün; `projectRules.test.ts` deckt `dev.html` ab.

## Prüfen

```bash
task check
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
