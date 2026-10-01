# B-081 · Eine Testseite startet Test-Szenarien, zuerst 1–4 Spieler mit Mock-Spielern

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
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
- Ein Klick erstellt einen Raum (eigener Spielstandname, z. B. `test-<n>`, `fresh: true`) und startet das Spiel mit dem echten Spieler als Slot 0.
- Mock-Spieler erscheinen sofort als eigene Monarchen im Raum und stehen still. In der ersten Version nur das; Verhalten (Laufen, Bauen,
  Zufallseingaben) kommt später.
- Szenarien sind Daten (eine Liste in einer Datei), damit neue ohne Seitenumbau dazukommen.
- Die Seite hat `installPageChrome()` und steht in `src/landing/pages.ts` (Abschnitt `test`); Seiten-Regeln aus `CLAUDE.md`.

## Nicht-Ziele

Verhalten der Mock-Spieler über „steht still“ hinaus; weitere Szenarien (Wiederverbinden, Raum voll, Stufenwechsel); Dev-Aktionen am Server (B-080).

## Regeln und Einschränkungen

Domäne PLAT (`*.html`, `src/landing/`, `src/tools/`); der Client aus SP08 (`src/online/client*.ts`) wird nur benutzt, nicht geändert.
Nach SP08.3 (Lobby, Client). Test-Räume belegen Raumplätze auf dem Server (höchstens 4 Räume, B-016/Protokoll); beim Beenden verlassen.

## Beispiele

Kachel „Testing“ → Seite → „4 Spieler“ → Raum mit 4 Monarchen, ich steuere einen, die anderen stehen; das Layout zeigt ein 2×2-Raster.

## Ausnahme- und Fehlerfälle

Raum nicht erstellbar (`too_many_rooms`, Server weg) → Hinweis auf der Testseite statt leerem Bildschirm. Gleiches Szenario zweimal → neuer Spielstandname.

## Akzeptanzkriterien

- **AC-01** `testing.html` ist von der Landingpage erreichbar (Eintrag in `pages.ts`, `tests/projectRules.test.ts` grün).
- **AC-02** Je „1 Spieler“ bis „4 Spieler“ entsteht ein Raum mit genau dieser Zahl besetzter Monarchen (Test mit Fake-Server bzw. Client-Test).
- **AC-03** Mock-Spieler stehen still und verhindern nichts (die echte Eingabe steuert nur Slot 0).
- **AC-04** Am TV oder Rechner: 4 Spieler zeigen das 2×2-Raster (Beobachtung durch 🧑).

## Offene Fragen

Woher kommen die Mock-Spieler? Vorschlag: der Browser öffnet je Mock eine eigene Verbindung mit eigener Geräte-ID (ein „Mock-Gerät“,
nutzt den Client aus SP08, kein Server-Umbau). Alternative: ein Server-Aufruf, der Bots in den Raum setzt (SRV, mit B-080/M6). (🧑)
Sollen Mock-Spieler als lokale Slots desselben Geräts erscheinen (testet das 2×2-Raster direkt) oder als andere Geräte (testet den Mitspieler oben ⅓)? (🧑)

## Notizen

Entstanden aus dem Wunsch nach einer Testseite mit Szenarien. Einordnung: einschiebbarer PLAT-Sprint nach SP08.3, wenn 🧑 die Fragen entschieden hat.
