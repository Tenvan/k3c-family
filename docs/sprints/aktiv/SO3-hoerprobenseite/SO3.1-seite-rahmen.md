# SO3.1 · Seite `soundtest.html` mit Seitenrahmen und Landingpage-Eintrag

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** so3/1-seite-rahmen
- **Abhängig von:** –
- **Tickets:** B-169
- **Kriterien:** AC-01, AC-05

## Ziel

Die Seite `soundtest.html` existiert mit Seitenrahmen (Home-Button, Zurück-Falle), steht auf der Landingpage und hält alle Seiten-Regeln aus `CLAUDE.md` ein; der Inhalt ist zunächst ein leeres Gerüst mit Überschrift und Hinweis „Taste drücken zum Entsperren“.

## Kontext

- Seiten-Regeln: `CLAUDE.md` › Regel: Seiten & Navigation (`installPageChrome()` aus `src/core/shell.ts`, oben ca. 70 px frei, Eintrag in `src/landing/pages.ts`, Vollbild nur `toggleFullscreen()` aus `src/core/fullscreen.ts`, Seitenwechsel nur `openPage()`/`goHome()`, B frei, View + Menu reserviert). `tests/projectRules.test.ts` prüft das automatisch; jede `*.html` im Root wird gebaut.
- Vorbild: `grafiken.html` mit `src/tools/grafiken.ts` (108 Zeilen; Auswahlseite, Pad-Scroll über `installPadScroll` aus `src/tools/spriteReference.ts`) und `lizenzen.html`/`src/tools/lizenzen.ts` (Gerüst).
- Kachel in `src/landing/pages.ts` im Abschnitt `test` (Tests & Werkzeuge); `src/landing/serverCheck.test.ts` listet erwartete Kacheln, ggf. ergänzen.

## Erlaubte Dateien

- `soundtest.html` (neu), `src/tools/soundtest.ts` (neu)
- `src/landing/pages.ts`, `src/landing/serverCheck.test.ts` (nur neue Kachel)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Kandidatenliste, Abspielen, Crossfade (SO3.2), Audio-Kern (SO1), Kandidaten selbst (SO2, SO4).

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. `soundtest.html` und `src/tools/soundtest.ts` nach Vorbild, `installPageChrome()`, Platz oben frei.
3. Kachel in `src/landing/pages.ts`.
4. Nachweis im Browser-Pane: Seite über die Landingpage öffnen, Home-Button sichtbar, `goHome()` führt zurück (Screenshot im PR).
5. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: `soundtest.html` ruft `installPageChrome()` auf und steht in `src/landing/pages.ts`; `tests/projectRules.test.ts` grün.
- [x] AC-05: Vollbild nur über `toggleFullscreen()`, Seitenwechsel nur über `openPage()`/`goHome()` (Test grün).
- [x] `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

`soundtest.html` und `src/tools/soundtest.ts` angelegt (Gerüst mit Hinweis „Taste drücken zum Entsperren“, entsperrt per Taste, Klick/Touch oder Controller-Taste), Kachel „Hörprobe“ in `src/landing/pages.ts` (Abschnitt `test`), `serverCheck.test.ts` ergänzt.

- AC-01: `tests/projectRules.test.ts` grün (Seite eingetragen, `installPageChrome()` im Script).
- AC-05: Seite nutzt weder `requestFullscreen()` noch `location`; Regeltests grün.
- `task check` grün (50 Dateien, 905 Tests).
- Schritt 4 (Browser-Pane-Screenshot) bewusst nicht ausgeführt, Auftrag ohne Browserprüfung; Sicht am TV bleibt SO3.3.
