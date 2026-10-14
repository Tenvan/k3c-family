# SV1.4 · Knopf „Voll ausgebaut starten“ im Level-Betrachter

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** live
- **Branch:** sv1/4-knopf-level-betrachter
- **Abhängig von:** SV1.3
- **Tickets:** B-315
- **Kriterien:** AC-01, AC-06

## Ziel

`leveltest.html` hat einen Knopf (und eine Taste), der über die Dev-API aus SV1.3 den voll ausgebauten Spielstand zum gewählten Seed erzeugt und ihn im Spiel startet.

## Kontext

- Domänen-Ausnahme (Beschluss 🧑 2026-10-06): Diese Session ändert PLAT-Dateien (`leveltest.html`, `src/tools/`) im SRV-Sprint SV1.
- `src/tools/leveltest.ts` (196 Zeilen): Seed-Feld, Biom-Auswahl, `startButton` öffnet über `startTarget(seed, biome)` und `openPage(target.url)` das Spiel; Tastatur-Handler (`keydown` auf `document`, nicht im Eingabefeld) und Controller-Handler (`PAD = { A, LB, RB, UP, DOWN }`). `src/tools/levelApi.ts` holt `/api/level` mit injizierbarem `fetchFn` (Muster für den neuen Aufruf, Test `levelApi.test.ts`).
- Start eines vorhandenen Spielstands: `game.html?autostart=1&save=<name>` ohne `fresh` (Parser `parseStartParams` in `src/scenes/lobbyLogic.ts`, Beispiele in `src/tools/testScenarios.ts`). Ob die Startstufe aus dem gewählten Biom übergeben werden kann, prüfen (ungeprüft).
- Seiten-Regeln aus `CLAUDE.md`: Wechsel nur über `openPage()`, Taste **B** nicht belegen, View + Menu reserviert. Ohne Dev-Mode (403) zeigt die Seite einen Hinweis statt zu starten.
- Den Spielstandnamen und den API-Pfad liefert das Ergebnis von SV1.3.

## Erlaubte Dateien

- `leveltest.html`, `src/tools/leveltest.ts`, `src/tools/levelApi.ts`, `src/tools/levelApi.test.ts`
- `src/tools/texts.de.ts`, `src/tools/texts.en.ts` (Texte für Knopf und Hinweis; ergänzt mit Zustimmung 🧑 2026-10-14, Chat)
- Planungs-Dateien (`docs/sprints/`, `docs/backlog/`)

## Nicht-Ziele

Änderungen an Server, Spiel-Szenen oder Renderer; Darstellung der Ausbaustufen (B-208).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Ergebnis von SV1.3 lesen (API-Pfad, Spielstandname).
2. Test zuerst in `src/tools/levelApi.test.ts`: Aufruf schickt den Seed an die Dev-API und liefert den Spielstandnamen; 403 ergibt einen erkennbaren Fehler.
3. Funktion in `levelApi.ts` umsetzen; Ziel-URL zum Laden des Stands als reine Funktion mit Test.
4. Knopf in `leveltest.html` und Anbindung in `leveltest.ts`: Klick, Tastatur-Taste (z. B. `V`, nicht im Eingabefeld) und Controller (nicht B); Hinweis bei 403.
5. `task check` grün.

## Fertig, wenn

- [x] AC-01: Vitest belegt Aufruf der Dev-API und Ziel-URL des Spielstands.
- [x] AC-01: Knopf und Taste sind eingebaut; die Beobachtung am PC (B-315/AC-03) macht 🧑 bei der Abnahme des Sprints.
- [x] AC-06: `task check` grün (inklusive `tests/projectRules.test.ts`).

## Prüfen

```bash
task check
```

Manuelle Prüfung (Klick am PC, B-315/AC-03) nur durch 🧑 bei der Sprint-Abnahme.

## Ergebnis

- **AC-01 (Vitest)** umgesetzt, geprüft: `src/tools/levelApi.test.ts` › `createFullSave` (POST an `/api/dev/fullsave` mit `{"seed"}`, liefert `voll-…`; 403 → Hinweis `level.fullForbidden`; Seed außerhalb `[a-z0-9-]{1,27}` fragt gar nicht erst, weil `voll-<seed>` sonst nicht ins Namensformat der Lobby passt; Server weg oder Antwort ohne Spielstand → Hinweis) und `fullSaveTarget` (`game.html?autostart=1&save=voll-abc`, ohne `fresh`).
- **AC-01 (Knopf und Taste)** umgesetzt: In `leveltest.html` gibt es den Knopf „Voll ausgebaut starten (V)“ und darunter einen Hinweis (`#fullNote`), angebunden in `src/tools/leveltest.ts` über `startFull()`. Taste `V` wirkt außerhalb der Eingabefelder, am Controller über Steuerkreuz und A wie die anderen Knöpfe. B, View und Menu bleiben unbelegt. Der Wechsel läuft über `openPage()`. Bei 403 steht der Hinweis zum Dev-Mode da, gestartet wird nichts. Die Beobachtung am PC (B-315/AC-03) macht 🧑 bei der Sprint-Abnahme.
- **Startstufe:** Der Autostart beginnt immer in Tiefe 0 (B-095), dort steht Hub-Stufe 5. Die Biom-Auswahl wirkt beim vollen Stand nicht, der Hinweistext sagt das.
- **Erlaubte Dateien:** um `src/tools/texts.de.ts` und `texts.en.ts` erweitert, Zustimmung 🧑 2026-10-14 im Chat. Neue Schlüssel `level.full*`, `level.keys` nennt V.
- **AC-06** geprüft: `task check` grün (k3c-dev `check_run`, mit `tests/projectRules.test.ts`).
