# PL2.3 · Restliche Werkzeug-Seiten (Dev, Testen, Gamepad, Figuren, Aufstellung, Lizenzen)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** pl2/3-restliche-werkzeug-seiten
- **Abhängig von:** PL2.2
- **Tickets:** B-322
- **Kriterien:** AC-01

## Ziel

Alle übrigen Werkzeug-Seiten folgen der gewählten Sprache; die Liste `OFFEN` in `src/tools/textRule.test.ts` ist leer und wird entfernt.

## Kontext

- Werkzeug aus PL2.1: `src/tools/texts.ts` (`t()`, `applyTexts()`), Tabellen `src/tools/texts.de.ts`/`texts.en.ts`, Regeltest `src/tools/textRule.test.ts` mit `OFFEN` und `DATEN`. Vorbild: `src/tools/leveltest.ts` mit `leveltest.html`.
- Dateien dieser Session: `dev.ts`, `devTiles.ts`, `dev.html`; `testing.ts`, `testTiles.ts`, `testScenarios.ts`, `testing.html`; `gamepadTest.ts`, `audioProbe.ts`, `gamepad-test.html`; `selection.ts`; `spriteReference.ts`; `figuren.ts`, `figuren.html`; `aufstellung.ts`, `aufstellung.html`; `lizenzen.ts`, `lizenzen.html` (Credits-Seite).
- `audioProbe.ts` liefert Zeilen für die Berichte des Gamepad-Tests (`reports/*.json`); die Beschriftungen werden übersetzt, die Auswertung der Berichte darf daran nicht hängen.
- Kachel-Titel in `devTiles.ts`/`testTiles.ts` werden zur Laufzeit über `t()` gelesen, nicht beim Laden des Moduls.
- Lizenztexte, Urheber und Quellen sind Daten und bleiben unübersetzt.

## Erlaubte Dateien

- `src/tools/` (alle Module und Tests)
- `dev.html`, `testing.html`, `gamepad-test.html`, `figuren.html`, `aufstellung.html`, `lizenzen.html`
- Planungs-Dateien des Sprints

## Nicht-Ziele

Lizenz- und Urheber-Daten übersetzen; Log-Meldungen; Seiten außerhalb von `src/tools/` (Landingpage, Spiel).

## Schritte

1. Entwickler- und Testseite mit ihren Kacheln umstellen (`dev*`, `test*`, `selection.ts`).
2. Gamepad-Test, Figuren, Aufstellung, Sprite-Referenz und Lizenzen umstellen.
3. `OFFEN` aus `textRule.test.ts` entfernen; `DATEN` begründet jeden Eintrag.
4. `task check`.

## Fertig, wenn

- [ ] AC-01: `textRule.test.ts` grün ohne Liste `OFFEN`; nur reine Daten-Module stehen in `DATEN`.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
