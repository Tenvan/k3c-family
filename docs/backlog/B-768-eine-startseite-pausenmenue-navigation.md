# B-768 · Eine Startseite, Navigation im Pausenmenü, Vollbild halten

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BD11
- **Projekt:** BED
- **Erstellt:** 2026-11-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Heute gibt es eine Landingpage mit Kacheln (Spiel, Lobby, Werkzeug-, Test- und Info-Seiten), die weitere Seiten im Vollflächen-iframe öffnet (`src/landing/`, `src/core/shell.ts`), dazu `game.html` mit Lobby (neue Spiele, Fortsetzen) und Pausenmenü. Auf der Xbox (Edge) kostet jeder Wechsel zwischen Seiten und Menüs Bedienung per Pad, und der Vollbildmodus geht dabei leicht verloren. Beim Xbox/Pi-Test (RX1) ist die Navigation über mehrere Einstiege umständlich.

## Ziel

Es gibt nur noch einen Einstieg: eine Startseite, in der man zwischen den Bereichen mit links/rechts (Pad, Tastatur) oder mit Schaltflächen auf der Seite wechselt. Neue Spiele und Fortsetzen gibt es nur noch im Pausenmenü des Spiels. Die bisherige Landingpage und alle anderen Seiten erreicht man nur über das Pausenmenü. Der Client versucht, dauerhaft im Vollbild zu bleiben.

## Beteiligte und Zielgruppen

Spieler am TV mit Xbox-Controller (Edge), Entwickler am PC. Claude setzt um, 🧑 nimmt am Gerät ab.

## Anforderungen

- **Eine Startseite:** `index.html` zeigt eine Seite, auf der zwischen den Bereichen (Spiel, Seiten, Werkzeuge) per links/rechts, Pad-D-Pad/Stick, Tastatur und sichtbaren Schaltflächen gewechselt wird; die bisherigen Kacheln gehen darin auf.
- **Spiel startet direkt im Pausenmenü:** `game.html` öffnet ohne Spielstand das Pausenmenü; Neues Spiel und Fortsetzen (heute Lobby) sind dort Einträge; die Lobby als eigener Einstieg entfällt.
- **Seitenwechsel nur über das Pausenmenü:** Einträge für Startseite und die übrigen Seiten (Credits, Lizenzen, Entwicklerseite …) stehen im Pausenmenü; `openPage()` und `goHome()` bleiben der einzige Weg.
- **Vollbild halten:** Der Client fordert beim ersten Nutzereingriff Vollbild an (`toggleFullscreen()` aus `src/core/fullscreen.ts`) und versucht es nach einem Verlust bei der nächsten Eingabe erneut; ein Hinweis zeigt, wenn der Browser es ablehnt.
- **Pad:** D-Pad/Stick links/rechts wechselt den Bereich, A bestätigt; B bleibt frei, View+Menu reserviert (Zurück-Kombi).

## Nicht-Ziele

Kein neues Menü-Design, keine neuen Spielmodi, keine Änderung an Spielständen oder Protokoll. Die Entwickler-Werkzeugseiten bleiben unverändert und erreichbar (nur anderer Weg).

## Regeln und Einschränkungen

Die Shell-Regeln gelten: Landingpage bleibt dauerhaft offen (Vollbild über Seitenwechsel), Seiten nur über `openPage()`, Vollbild nur über `toggleFullscreen()`; `installPageChrome()` auf jeder Seite; B nicht belegen; Texte aus `src/core/texts.*.ts`; Mitten im Spiel nie neu laden.

## Beispiele

Xbox: App starten → Startseite → mit D-Pad rechts zum Spiel → A → Pausenmenü mit Neues Spiel / Fortsetzen → Menü → Seiten → Credits → Zurück ins Pausenmenü; Vollbild bleibt während der ganzen Folge.

## Ausnahme- und Fehlerfälle

Lehnt der Browser Vollbild ab (keine Nutzergeste), zeigt ein Hinweis die Eingabe an, die es auslöst; kein Dauerversuch. Ohne Server bleibt die Startseite mit deaktiviertem Spiel (wie heute). Ein Spiel im Gange wird beim Seitenwechsel nicht beendet ohne Rückfrage.

## Akzeptanzkriterien

- **AC-01** Test: Startseite wechselt Bereiche mit links/rechts (Pad, Tastatur) und Schaltflächen; Zielwahl und Fokus.
- **AC-02** Test: Neues Spiel und Fortsetzen sind nur im Pausenmenü erreichbar; keine andere Seite startet ein Spiel.
- **AC-03** Test: Alle bisherigen Seiten sind aus dem Pausenmenü erreichbar und keine andere Seite verlinkt untereinander (projectRules).
- **AC-04** Test: Vollbild wird bei der ersten Eingabe angefordert und nach Verlust erneut, mit Hinweis bei Ablehnung (Logik rein, Browser gemockt).
- **AC-05** Abnahme (🧑): Xbox, Edge: Start → Spiel → Pausenmenü → Seiten → zurück, Vollbild bleibt, alles mit Pad.

## Offene Fragen

keine (entschieden 2026-11-09 🧑: Lobby-Aktionen je ein Eintrag im Pausenmenü; Werkzeugseiten unter „Seiten“).

## Notizen

Baut auf B-767 (padNav, frischer Client) auf. Vollbild-Verhalten: Edge auf der Xbox verlässt Vollbild bei Navigation; Shell-Iframe-Lösung bleibt.

**Gestaltung (entschieden 2026-11-09 🧑, Entwurf A „Kachel-Hub“):** Pausenmenü als Vollbild-Kachelseite im Stil der Landingpage (Nachtblau, Gold, Glas-Kacheln). Große goldene Kachel „Fortsetzen“ (Insel, Stufe, Welt, zuletzt gespielt), daneben Neues Spiel (grün), Raum beitreten (blau), Optionen (lila), Seiten (türkis); Spiel verlassen unten links (koralle). Pad: D-Pad/Stick bewegt, A wählt, B frei. Produktkontext: `PRODUCT.md`.
