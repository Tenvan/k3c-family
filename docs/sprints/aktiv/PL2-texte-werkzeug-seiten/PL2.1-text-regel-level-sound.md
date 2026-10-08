# PL2.1 · Text-Regel für `src/tools/`, Level-Betrachter und Hörprobe

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** pl2/1-text-regel-level-sound
- **Abhängig von:** –
- **Tickets:** B-322
- **Kriterien:** AC-01

## Ziel

Werkzeug-Seiten haben eigene Textdateien mit `t()` und Rückfall auf Deutsch, dazu einen Regeltest gegen deutsche Literale. Level-Betrachter (`leveltest.html`) und Hörprobe (`soundtest.html`) sind vollständig umgestellt, auch ihre statischen HTML-Texte.

## Kontext

- Spieltexte liegen in `src/core/texts.ts` (`t()`, `currentLanguage()`, Rückfall Deutsch) mit `texts.de.ts`/`texts.en.ts` (S5.3, B-172). `texts.de.ts` hat schon 182 Zeilen; die Werkzeug-Texte (~150 Schlüssel) kämen über 400 Zeilen. Deshalb bekommen die Werkzeug-Seiten **eigene** Tabellen `src/tools/texts.de.ts` und `src/tools/texts.en.ts` (gleicher Aufbau, `en` mit Typ `Partial<Record<ToolTextKey, string>>`) und ein `t()` in `src/tools/texts.ts`, das die Sprache über `currentLanguage()` aus `src/core/texts.ts` holt. `src/core/` bleibt unverändert.
- Statische Texte in `*.html` bekommen ein Attribut `data-t="schlüssel"` (bei `aria-label`/`placeholder`: `data-t-aria`, `data-t-placeholder`); `applyTexts(root = document)` aus `src/tools/texts.ts` setzt sie beim Start der Seite. Ohne Aufruf bleibt der deutsche Text im HTML stehen (Rückfall).
- Vorbild für den Regeltest: `src/scenes/textRule.test.ts` (liest Literale, prüft Umlaute oder zwei Wörter). Der neue Test `src/tools/textRule.test.ts` prüft alle Module unter `src/tools/` außer Tests, mit einer Liste `OFFEN` noch nicht umgestellter Dateien, die PL2.2 und PL2.3 leeren. Reine Daten (Grafik-Packs mit Urheber und Notizen in `grafikPacks.ts`) sind keine Oberflächen-Texte und stehen mit Grund in einer eigenen Liste `DATEN`.
- Diese Session: `leveltest.ts`, `levelView.ts`, `levelApi.ts`, `leveltest.html`, `soundtest.ts`, `soundtestLogic.ts`, `audioProbe.ts`, `soundtest.html`.
- Schlüssel nach Seite gruppiert: `level.*`, `sound.*`; Platzhalter `{name}` wie in `src/core/texts.ts`.

## Erlaubte Dateien

- `src/tools/texts.ts`, `src/tools/texts.de.ts`, `src/tools/texts.en.ts`, `src/tools/texts.test.ts`, `src/tools/textRule.test.ts` (neu)
- `src/tools/leveltest.ts`, `src/tools/levelView.ts`, `src/tools/levelApi.ts` und ihre Tests
- `src/tools/soundtest.ts`, `src/tools/soundtestLogic.ts`, `src/tools/audioProbe.ts` und ihre Tests
- `leveltest.html`, `soundtest.html`
- Planungs-Dateien des Sprints

## Nicht-Ziele

Andere Werkzeug-Seiten (PL2.2, PL2.3); Änderungen an `src/core/texts*.ts`; Sprachwahl auf den Werkzeug-Seiten selbst; Log-Meldungen (bleiben deutsch).

## Schritte

1. `src/tools/texts.ts` mit `t(key, params?)` und `applyTexts(root?)`, dazu `texts.de.ts`/`texts.en.ts`; Test `texts.test.ts` (Englisch, Rückfall bei fehlendem Text, Platzhalter, `applyTexts`).
2. `src/tools/textRule.test.ts` mit Listen `OFFEN` und `DATEN`; zu Beginn stehen alle noch nicht umgestellten Dateien in `OFFEN`.
3. Level-Betrachter umstellen (Module und `leveltest.html`), Dateien aus `OFFEN` streichen.
4. Hörprobe umstellen (Module und `soundtest.html`), Dateien aus `OFFEN` streichen.
5. `task check`.

## Fertig, wenn

- [ ] AC-01: `textRule.test.ts` grün; Level-Betrachter- und Hörprobe-Module stehen nicht in `OFFEN`.
- [ ] AC-01: `texts.test.ts` zeigt Englisch, Rückfall auf Deutsch und `applyTexts` für `data-t`.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-01 umgesetzt (Teil Level-Betrachter, Hörprobe), geprüft:** `task check` grün (`check_run`, 1655 Tests). `src/tools/textRule.test.ts` prüft alle Module unter `src/tools/` außer `OFFEN` und `DATEN`; `src/tools/texts.test.ts` belegt Englisch, Rückfall auf Deutsch und `applyTexts` (`data-t`, `data-t-aria`, `data-t-placeholder`).
- Gegenprobe vom Agenten im Browser-Pane (Vite, ohne Go-Server): `soundtest.html` und `leveltest.html` mit `language: en` vollständig englisch, Tab-Titel eingeschlossen; ohne Einstellung deutsch. Ersetzt nicht die Abnahme in PL2.5.
- **Abweichungen:** `audioProbe.ts` gehört zum Gamepad-Test und bleibt für PL2.3 in `OFFEN`. Der Regeltest entfernt HTML-Tags vor der Prüfung und findet dadurch die deutschen Tabellenköpfe in `credits.ts`, die vorher nicht auffielen; die Datei steht in `OFFEN` für PL2.2. `soundtestLogic.ts` steht in `DATEN`, weil Zustände und Ereignisse die Gruppen-Namen aus `public/audio/kandidaten.json` sind. Biom-Namen im Level-Betrachter kommen jetzt über `nameOf('biome', …)` aus den Spieltexten.
- Neue Tickets: keine.
