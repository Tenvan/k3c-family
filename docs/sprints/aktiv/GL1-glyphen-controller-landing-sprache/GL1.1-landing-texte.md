# GL1.1 · Landingpage-Texte über t() in de und en

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** gl1/1-landing-texte
- **Abhängig von:** –
- **Tickets:** B-369
- **Kriterien:** AC-02

## Ziel

Alle sichtbaren Texte der Landingpage (Kacheln, Abschnitte, Statuszeile, Vollbild-Knopf und -Hinweis, Fußleiste, Server-Hinweis) kommen über `t()` aus `src/core/texts.de.ts` und `texts.en.ts`; ein Regeltest verhindert neue deutsche Literale in `src/landing/*.ts` und `index.html`.

## Kontext

- `t()` und `currentLanguage()` liegen in `src/core/texts.ts` (Tabellen `texts.de.ts`, `texts.en.ts`; fehlt ein englischer Text, gilt der deutsche). Die Sprache kommt aus den Einstellungen des Geräts, die Landingpage übernimmt sie beim Laden (Neuladen genügt, Sprint-README › Offene Fragen).
- Deutsche Literale heute: `src/landing/pages.ts` (`PAGES` › `title`, `description`; `SECTIONS`), `src/landing/landing.ts` (`frame.title`, Vollbild-Knopf und -Hinweis, Controller-Status), `src/landing/serverCheck.ts` (`NO_SERVER_HINT`, Text in `showNoServer`, auch von `src/main.ts` auf der Spielseite genutzt), `index.html` (Tagline, Controller-Status, „Vollbild“, Fußleiste „/ Stick auswählen“, „öffnen“, „Vollbild“, „auf jeder Seite: zurück hierher“).
- `landing.ts` merkt sich die gewählte Kachel über `dataset.title` in `sessionStorage`; mit übersetzten Titeln wird dafür eine sprachunabhängige `id` gebraucht.
- Vorbild für den Regeltest: `src/core/platformTextRule.test.ts` (B-215, prüft `touchInput.ts` und `shell.ts`); in Markup zählen nur Umlaute und ß. `index.html` braucht zusätzlich eine Prüfung des Text-Inhalts zwischen Tags (ohne `<style>`, `<script>`, Kommentare); „Family Three Crowns“ ist erlaubt (B-369 › Nicht-Ziele).
- Fußleisten-Glyphen (A, Y, View, Menu, ✚) bleiben Xbox-Beschriftung und unübersetzt (B-339 verworfen).

## Erlaubte Dateien

- `src/landing/`
- `index.html`
- `src/core/texts.de.ts`, `src/core/texts.en.ts`
- `src/core/platformTextRule.test.ts`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Sprache ohne Neuladen übernehmen; Glyphen je Controller; Werkzeug-Seiten, Shell und Touch-Overlay (schon übersetzt); Browser- oder Geräteprüfungen (GL1.3).

## Schritte

1. `Status: in Arbeit`, committen, pushen.
2. Regeltest erweitern: `src/landing/*.ts` (ohne Tests) und `index.html` in `platformTextRule.test.ts`; Test rot.
3. Schlüssel `landing.*` in `texts.de.ts` und `texts.en.ts` anlegen.
4. `pages.ts`: `PageEntry` bekommt eine `id`, Titel und Beschreibung kommen über `t()`; `SECTIONS` über `t()`. `landing.ts` merkt sich die Kachel über die `id`. Tests in `src/landing/` auf `id` umstellen.
5. `landing.ts`, `serverCheck.ts`, `index.html` (Texte per `data-t` beim Laden setzen) umstellen; Test grün.
6. `task check` grün, Ergebnis eintragen, `Status: fertig`, committen, pushen.

## Fertig, wenn

- [ ] AC-02: `platformTextRule.test.ts` prüft `src/landing/*.ts` und `index.html` und ist grün; jeder neue Schlüssel steht in `texts.de.ts` und `texts.en.ts`.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
