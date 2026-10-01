# B-081 · Eine Testseite startet Test-Szenarien, zuerst 1–4 Spieler mit Mock-Spielern

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** T1
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Die Landingpage (`src/landing/pages.ts`) hat Kacheln für Spiel und einzelne Testseiten (`section: 'test'`). Um Layout,
Lobby und Mehrspieler-Verhalten zu prüfen, braucht man heute echte Geräte und Controller. Ab SP08 läuft alles über den Go-Server
(Räume, Slots), eine Prüfung mit 3–4 Spielern ist ohne Mitspieler kaum möglich.

## Ziel

Auf der Landingpage führt eine Kachel „Testing“ zu einer eigenen Seite `testing.html` mit speziellen Aufrufen für Test-Szenarien.
Erstes Szenario: Eine Session mit 1, 2, 3 oder 4 Spielern starten, bei der die übrigen Spieler **Mock-Spieler** sind. Nutzen: Layout
(2×2, Streifen), Raumgrenzen und Lobby lassen sich allein am Rechner oder an der Xbox prüfen.

## Beteiligte und Zielgruppen

Entwickler und Agenten; 🧑 prüft am TV. Spieler sehen die Seite nicht im Hauptbereich (Abschnitt „Test“).

## Anforderungen

- Neue Seite `testing.html` im Stil der Landingpage, mit je einer Schaltfläche „1 Spieler“ bis „4 Spieler“ (Controller, Tastatur und Touch bedienbar).
- Ein Klick öffnet `game.html` mit den Start-Parametern aus B-082 (`?autostart=1&fresh=1&save=test-<kennung>&mock=N−1`): ein neuer Raum,
  der echte Spieler ist Slot 0.
- Mock-Spieler sind **lokale Slots desselben Geräts** (Entscheidung 🧑 2026-10-01) mit eigenem Monarchen, eine einzige Browser-Verbindung.
  Sie erscheinen sofort und stehen still. In der ersten Version nur das; Verhalten (Laufen, Bauen, Zufallseingaben) kommt später.
- Der Wechsel von der Testseite ins Spiel läuft über die Shell (`openPage()`, neue Shell-Nachricht), nicht über einen Link oder `location`
  (Regel 4 in `CLAUDE.md`).
- Szenarien sind Daten (eine Liste in einer Datei), damit neue ohne Seitenumbau dazukommen.
- Die Seite hat `installPageChrome()` und steht in `src/landing/pages.ts` (Abschnitt `test`); Seiten-Regeln aus `CLAUDE.md`.

## Nicht-Ziele

Verhalten der Mock-Spieler über „steht still“ hinaus; weitere Szenarien (Wiederverbinden, Raum voll, Stufenwechsel); Dev-Aktionen am Server (B-080).

## Regeln und Einschränkungen

Domäne PLAT (`*.html`, `src/landing/`, `src/tools/`, `src/core/shell.ts`); Spiel und Client (SP08, B-082) werden nur benutzt, nicht geändert.
Nach SP08.3 (Lobby, Start-Parameter). Test-Räume belegen Raumplätze auf dem Server (höchstens 4 Räume, B-016/Protokoll); beim Beenden verlassen.

## Beispiele

Kachel „Testing“ → Seite → „4 Spieler“ → Raum mit 4 Monarchen, ich steuere einen, die anderen stehen; das Layout zeigt ein 2×2-Raster.

## Ausnahme- und Fehlerfälle

Raum nicht erstellbar (`too_many_rooms`, Server weg) → Hinweis auf der Testseite statt leerem Bildschirm. Gleiches Szenario zweimal → neuer Spielstandname.

## Akzeptanzkriterien

- **AC-01** `testing.html` ist von der Landingpage erreichbar (Eintrag in `pages.ts`, `tests/projectRules.test.ts` grün).
- **AC-02** Je „1 Spieler“ bis „4 Spieler“ baut die Testseite die richtige URL (`mock` = Spielerzahl − 1, neuer eindeutiger `save`-Name) und öffnet sie über die Shell (Test der URL-Funktion).
- **AC-03** Mock-Spieler stehen still und verhindern nichts: Die Szenen-URL nutzt nur die Parameter aus B-082; die echte Eingabe steuert nur Slot 0 (Beobachtung in AC-04).
- **AC-04** Am TV oder Rechner: 4 Spieler zeigen das 2×2-Raster (Beobachtung durch 🧑).

## Offene Fragen

keine (Entscheidung 🧑 2026-10-01: Mocks als lokale Slots, eine Browser-Verbindung; Mocks als eigene Geräte wären ein späteres Szenario).

## Notizen

Entstanden aus dem Wunsch nach einer Testseite mit Szenarien. Eingeplant als einschiebbarer PLAT-Sprint T1 nach SP08.3 (braucht B-082).
