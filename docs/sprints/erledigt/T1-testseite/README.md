# T1 · PLAT · Testseite mit Szenarien und Mock-Spielern

- **Status:** erledigt
- **Domäne:** PLAT
- **Prio:** mittel
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-081
- **Start-Commit:** 59389cd
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (T1 Revision 1, mit B-081 Revision 2)

## Ausgangslage

Die Landingpage hat einen Abschnitt „Tests & Werkzeuge“ (`src/landing/pages.ts`, `section: 'test'`). Um Layout und Räume mit 3–4 Spielern zu prüfen,
braucht man heute echte Geräte. SP08 liefert ab SP08.3 die Start-Parameter `?autostart=1&fresh=1&save=NAME&mock=N` (B-082) für `game.html`.

## Ziel

Auf der Landingpage führt eine Kachel zu einer Testseite, die Test-Szenarien startet, zuerst eine Session mit 1, 2, 3 oder 4 Spielern, bei der die
übrigen Mock-Spieler sind. Am Ende sichtbar: Kachel „Testing“ → „4 Spieler“ → Spiel im 2×2-Raster mit vier Monarchen.

## Beteiligte und Zielgruppen

Entwickler und Agenten; 🧑 prüft am TV oder Rechner und gibt die Spec frei.

## Anforderungen

B-081 › Anforderungen. Sprint-eigen:

- Neue Shell-Nachricht `open` (`openPage(href)` in `src/core/shell.ts`): Eine Seite bittet die Landingpage, im Vollflächen-iframe eine andere Seite zu öffnen.
  Die Landingpage lässt nur relative Ziele auf `*.html` im selben Ordner zu (kein `//`, kein Schema, kein `..`), alles andere wird ignoriert.
  Regel 4 in `CLAUDE.md` bekommt den Zusatz „außer über `openPage()`“.
- Die Szenarien liegen als Daten in `src/tools/testScenarios.ts` (`id`, Titel, Spielerzahl); eine reine Funktion baut daraus die URL. Neue Szenarien
  sind ein Eintrag, kein Seitenumbau.
- Mock-Spieler sind lokale Slots desselben Geräts (Entscheidung 🧑 2026-10-01), eine Verbindung, sie stehen still.

## Nicht-Ziele

Verhalten der Mock-Spieler über „stehen“ hinaus, weitere Szenarien, Mock-Spieler als eigene Geräte; Änderungen an Spiel, Client oder Protokoll (SP08, B-082).

## Regeln und Einschränkungen

Domäne PLAT. Seiten-Regeln aus `CLAUDE.md`: `installPageChrome()`, Eintrag in `src/landing/pages.ts`, oben ca. 70 px frei, Vollbild nur über `toggleFullscreen()`,
B nicht belegen, View + Menu = Home. Bedienbar mit Controller (Fokus, A), Tastatur und Touch. Beginnt erst, wenn SP08.3 auf `main` liegt.
Prüfungen am TV nur mit Freigabe durch 🧑.

## Beispiele

Kachel „Testing“ → „3 Spieler“ → `game.html?autostart=1&fresh=1&save=test-k3x9&mock=2`: ein Raum mit drei Monarchen, zwei stehen, zwei Felder oben und eines breit unten.

## Ausnahme- und Fehlerfälle

Raum nicht erstellbar (`too_many_rooms`, Server weg) → die Fehlermeldung zeigt das Spiel wie in der Lobby (SP08), die Testseite bleibt über Home erreichbar.
Ziel von `open` unzulässig → ignoriert, kein Seitenwechsel.

## Akzeptanzkriterien

- **AC-01** `testing.html` ist von der Landingpage erreichbar (Eintrag in `pages.ts`, `task check` mit `tests/projectRules.test.ts` grün) (B-081/AC-01).
- **AC-02** Die Szenarien „1 Spieler“ bis „4 Spieler“ erzeugen die URL mit `mock` = Spielerzahl − 1 und einem neuen, gültigen `save`-Namen (Test der URL-Funktion) (B-081/AC-02, B-081/AC-03).
- **AC-03** `openPage()` öffnet nur zulässige Ziele (Test der Prüfung: `game.html?…` ja; `https://…`, `//…`, `../x.html`, `x.js` nein).
- **AC-04** Am Rechner oder TV: „4 Spieler“ zeigt ein 2×2-Raster mit vier Monarchen, drei stehen (Beobachtung durch 🧑) (B-081/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| T1.1 | `T1.1-testseite.md` | Umsetzung | autonom | fertig |
| T1.2 | `T1.2-review.md` | Review | autonom | fertig |
| T1.3 | `T1.3-abnahme.md` | Workshop | Mensch | fertig |

## Abnahme

Review 2026-10-01 (T1.2): `task check` grün (403 Tests); Diff `59389cd..main` gelesen; keine schweren Befunde, nichts behoben (ein Hinweis: `open` mit nicht-String-`href` würfe im Listener, harmlos, nur gleicher Ursprung).
AC-01 bis AC-03 geprüft mit Tests laut T1.1-Ergebnis (`isOpenable`-Tabelle, Szenarien-URLs, `projectRules`); im Browser bestätigt (Agent, Freigabe 🧑): Kachel → Testseite → „4 Spieler“ → 2×2-Raster → Home.
AC-04 abgenommen 2026-10-01 (T1.3, 🧑 im lokalen Browser, 2×2-Raster bei 4 Spielern; Layout bei 3 Spielern nach Wunsch von 🧑 geändert: zwei oben, einer breit unten, B-016 Revision 3). Neues Ticket: B-086 (Test-Spielstände sammeln sich in `saves/`).
