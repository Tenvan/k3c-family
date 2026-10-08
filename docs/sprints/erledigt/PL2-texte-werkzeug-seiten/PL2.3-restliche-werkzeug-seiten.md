# PL2.3 · Restliche Werkzeug-Seiten (Dev, Testen, Gamepad, Figuren, Aufstellung, Lizenzen)

- **Status:** fertig
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

- [x] AC-01: `textRule.test.ts` grün ohne Liste `OFFEN`; nur reine Daten-Module stehen in `DATEN`.
- [x] `task check` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `task check` grün; `textRule.test.ts` ohne Liste `OFFEN`, `DATEN` nennt nur `grafikPacks.ts` und `soundtestLogic.ts` (reine Daten).
- Umgestellt: `dev`, `testing` mit Kacheln (`devTiles.ts`, `testTiles.ts`, `testScenarios.ts` lesen Titel per Getter zur Laufzeit), `gamepadTest`, `audioProbe`, `selection`, `spriteReference`, `figuren`, `aufstellung`, `lizenzen` samt HTML.
- Neu: `data-t-html` in `applyTexts()` für Texte mit eigenem HTML (`<kbd>`, Links); Lizenz-, Urheber- und Quellen-Daten bleiben unübersetzt.
- Abweichung: Fehlertext `kein vibrationActuator` im Bericht heißt jetzt `vibrationActuator-missing` (kein Leerzeichen, Regeltest). Berichtswerte `gespielt`/`blockiert`/`fehler: …` und die Wege `klick`/`taste` bleiben im Bericht deutsch, nur die Anzeige wird übersetzt.
- Manuell geprüft: nichts (Sprachwechsel am Gerät: PL2.5).
- Neue Tickets: keine.
