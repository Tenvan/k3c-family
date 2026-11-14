# T1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Testseite mit Szenarien und Mock-Spielern
- **Projekt:** –

## Ziel

Auf der Landingpage führt eine Kachel zu einer Testseite, die Test-Szenarien startet, zuerst eine Session mit 1, 2, 3 oder 4 Spielern, bei der die übrigen Mock-Spieler sind. Am Ende sichtbar: Kachel „Testing“ → „4 Spieler“ → Spiel im 2×2-Raster mit vier Monarchen.

## Ergebnis

Review 2026-10-01 (T1.2): `task check` grün (403 Tests); Diff `59389cd..main` gelesen; keine schweren Befunde, nichts behoben (ein Hinweis: `open` mit nicht-String-`href` würfe im Listener, harmlos, nur gleicher Ursprung). AC-01 bis AC-03 geprüft mit Tests laut T1.1-Ergebnis (`isOpenable`-Tabelle, Szenarien-URLs, `projectRules`); im Browser bestätigt (Agent, Freigabe 🧑): Kachel → Testseite → „4 Spieler“ → 2×2-Raster → Home. AC-04 abgenommen 2026-10-01 (T1.3, 🧑 im lokalen Browser, 2×2-Raster bei 4 Spielern; Layout bei 3 Spielern nach Wunsch von 🧑 geändert: zwei oben, einer breit unten, B-016 Re …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `testing.html` ist von der Landingpage erreichbar (Eintrag in `pages.ts`, `task check` mit … |
| AC-02 | geprüft | Die Szenarien „1 Spieler“ bis „4 Spieler“ erzeugen die URL mit `mock` = Spielerzahl − 1 un … |
| AC-03 | geprüft | `openPage()` öffnet nur zulässige Ziele (Test der Prüfung: `game.html?…` ja; `https://…`, … |
| AC-04 | – | Am Rechner oder TV: „4 Spieler“ zeigt ein 2×2-Raster mit vier Monarchen, drei stehen (Beob … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-081 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-04 – | Eine Testseite startet Test-Szenarien, zuerst 1–4 Spieler mit Mock-Spielern |
