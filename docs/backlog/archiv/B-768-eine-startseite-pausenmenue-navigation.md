# B-768 · Eine Startseite, Navigation im Pausenmenü, Vollbild halten

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** BD11
- **Projekt:** BED
- **Erstellt:** 2026-11-09
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-09 🧑 Chat (Konzept und Entscheidungen bestätigt)

## Ausgangslage

Heute ist `index.html` eine Landingpage mit Kachelraster, die jede andere Seite in einem Vollflächen-iframe öffnet (`src/landing/`, `src/core/shell.ts`); `game.html` ist das Spiel mit Lobby und Pausenmenü; Werkzeug-, Test- und Stress-Seiten sind eigene `*.html`. Szenarien und Tests werden über URL-Parameter gesteuert (`?dev`, `?autostart`, `?fresh`, `?save`, `?seed`, `?world`, `?players`, `?mock`, `?depth`, `?room`, `?perf`, `?stress`, `?scenario`, `?renderer`, `?gfx`, `?touch` …), die rund 24 Dateien jeweils selbst aus `location.search` lesen. Das ist fragil: Jeder Start braucht einen Seitenwechsel, ein alter Client im Zwischenspeicher ignoriert neue Parameter, und der Pi-Test zeigte, dass Szenen „nur als normaler Start“ liefen (B-767). Kachelseite für Pausenmenü (BD11.1) und Start der Lobby (BD11.2) stehen schon.

## Ziel

Ein Startpunkt: Wer `ip:8080` öffnet, ist direkt im Spiel und sieht das Spielmenü. Alle Features, Test-Szenarien und Werkzeuge laufen aus diesem Menü, ohne Seitenwechsel und ohne URL-Syntax. Die alte Landingpage entfällt komplett. Der Client hält Vollbild.

## Beteiligte und Zielgruppen

Familie am TV mit Xbox-Controller (Spielen, Optionen), Entwickler (Werkzeuge, Tests, Stress). 🧑 nimmt am Gerät ab (Xbox, Pi).

## Anforderungen

- **Das Spiel ist die Seite:** `/` liefert das Spiel; `index.html` ist das Spiel, die Landingpage, `src/landing/` und der Shell-Rahmen (`shell.ts`: Iframe-Wechsel, Home-Kombi über Nachrichten) entfallen; `game.html` leitet auf `/` um oder entfällt.
- **Menü-Ebenen:** (1) Startmenü (Fortsetzen, Neues Spiel, Raum beitreten, Optionen, Seiten; im Spiel dasselbe als Pause mit Weiter und Spiel verlassen), (2) Seiten mit den Kategorien Info und Hilfe, Entwicklung, Performance, Balancing, (3) Kategorie mit einer Kachel je Werkzeug oder Szenario, (4) das Werkzeug im Menü.
- **Werkzeuge im Menü eingebettet:** Die bestehenden HTML-Seiten (Testen, Gamepad-Test, Grafiken, Pipeline, Hörproben, Monitor, Stress Live-Grafik, Lizenzen) öffnen sich als vollflächiges Panel im selben Dokument unter einer Leiste mit Zurück (Menu/Esc); das Dokument navigiert nie. Ihr `installPageChrome()` schließt das Panel statt zur Landingpage zu gehen; Vollbild-Anfragen leitet das Spiel weiter.
- **Szenarien ohne URL:** Eine zentrale `RunConfig` (Spielstand, Neu, Seed, Welt, Tiefe, Spieler, Mock, Dev-Overlay, Perf-/Stress-Szenario, Renderer, Touch) ersetzt das verstreute Lesen von `location.search`. Quellen in dieser Rangfolge: Auswahl im Menü, URL beim Start (nur als Deep-Link für Tests, CI und Bots, einmal gelesen), Standardwerte. Menü-Einträge „Stress echt“, „Spiel mit Debug“ und Kategorien starten das Spiel mit einer `RunConfig` im selben Dokument (Szenen neu starten, kein Neuladen).
- **Dev-Modus:** Die Kategorien Entwicklung, Performance und Balancing sind vorerst immer sichtbar (fest an); später nur bei Dev-Modus des Servers (`/api/health`). Das Dev-Overlay und die Technik-Anzeige bleiben an.
- **Dungeon Master (`dm.html`)** bleibt extern: Die Kachel öffnet die Seite wie heute in einem neuen Fenster.
- **Vollbild halten:** erste Eingabe fordert Vollbild an (`toggleFullscreen()`), nach einem Verlust die nächste Eingabe erneut; Hinweis bei Ablehnung. Ohne Seitenwechsel geht es nicht mehr verloren.
- **Frischer Client:** Die Versionsprüfung (B-767) lädt das Spiel selbst neu, wenn der Server eine andere Version meldet, nur im Startmenü, nie im Spiel.
- **Pad:** Pfeile/Stick bewegen, A bestätigt, Menu/Esc zurück, B bleibt frei, View+Menu reserviert; die Panels bedient `padNav` oder ihre eigene Steuerung.

## Nicht-Ziele

Keine Neuschreibung der Werkzeugseiten in Phaser, kein neues Menü-Design (Entwurf A), keine Änderung an Protokoll oder Spielständen, keine neue Cache-Strategie am Server (Service-Worker).

## Regeln und Einschränkungen

Texte aus `src/core/texts.*.ts`; Kachelgrößen und Schrift nach `PRODUCT.md` (aus Sofa-Abstand lesbar, nur Pad nötig); jede Mechanik mit zwei Spielern; B nie belegen; Regeln in `CLAUDE.md` zu Seiten und Navigation werden mit dem Umbau neu gefasst (Shell, `openPage`, `goHome` entfallen oder bedienen das Menü), `tests/projectRules.test.ts` angleichen.

## Beispiele

Xbox: Adresse öffnen → Startmenü im Vollbild → A auf Fortsetzen → Spiel. Menü → Seiten → Performance → „Stress echt: Wald 2 Spieler“ → Spiel startet mit 2 Spielern und Gegnern, Technik-Anzeige an → Menu → zurück ins Menü. Menü → Seiten → Entwicklung → Monitor → Panel → Menu → Menü. CI-Test öffnet `/?autostart=1&save=ci` und landet ohne Menü im Spiel.

## Ausnahme- und Fehlerfälle

Lehnt der Browser Vollbild ab (keine Nutzergeste), zeigt ein Hinweis die Eingabe; kein Dauerversuch. Ohne Server bleibt das Startmenü mit ausgegrauten Spiel-Kacheln, Werkzeuge ohne Server-Bedarf bleiben bedienbar. Antwortet der Server auf `spawn` mit `forbidden`, endet ein Stresslauf mit Hinweis (B-767). Ein Panel, das nicht lädt, zeigt Fehlertext und Zurück. Ein Spiel im Gange wird beim Öffnen eines Werkzeugs nicht beendet (Pause), außer das Szenario startet ein neues.

## Akzeptanzkriterien

- **AC-01** Test: `GET /` liefert das Spiel (Startmenü als Standardszene); es gibt keine Landingpage-Dateien und keine Verweise darauf (`projectRules`).
- **AC-02** Test: Startmenü, Seiten und Kategorien sind per Pad, Tastatur und Tippen bedienbar; jede bisherige Seite und jedes Stress-Szenario hat genau eine Kachel (Daten aus `DEV_SECTIONS` und `PAGES` abgeleitet).
- **AC-03** Test: Ein Werkzeug öffnet als Panel im selben Dokument, Menu/Esc schließt es und der Fokus kehrt zur Kachel zurück; `location` ändert sich dabei nicht.
- **AC-04** Test: `RunConfig` wird aus Menü, URL und Standardwerten in dieser Rangfolge gebildet; kein Modul außer `RunConfig` liest `location.search` (Regel-Test).
- **AC-05** Test: Vollbild wird bei der ersten Eingabe angefordert und nach Verlust erneut, mit Hinweis bei Ablehnung (Browser gemockt).
- **AC-06** Test: Eine Stress-Kachel startet ohne Neuladen ein Spiel mit der geforderten `RunConfig` (Spieler, Welt, Seed, Szenario).
- **AC-07** Abnahme (🧑): Xbox/Edge und Pi: nur Pad, `ip:8080` öffnen, Vollbild bleibt, alle Kategorien und Stress-Szenen laufen mit Gegnern.

## Offene Fragen

keine (entschieden 2026-11-09 🧑: Dungeon Master bleibt extern, Dev-Modus vorerst fest an, Landingpage komplett weg, URL-Steuerung nach Ermessen von Claude durch `RunConfig` ersetzt).

## Notizen

Baut auf B-767 (padNav, frischer Client) auf; die Kachelseiten aus BD11.1 und BD11.2 (`pauseHub.ts`, `hubView.ts`) bleiben die Grundlage. Die Landingpage bleibt bis BD11.6 in Betrieb. Bots und Tools, die URLs nutzen (`botInput`, Lasttest, `sim_test`), bleiben über den Deep-Link-Pfad lauffähig.
