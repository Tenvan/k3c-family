# B-064 · k3c-dev hat eine Oberfläche mit Logs-Seite für Läufe und JSON-Logs

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** M4
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach M1 bis M3 (B-046, B-062, B-063, B-067) läuft `k3c-dev` als Programm ohne Fenster im Terminal. Dienste,
Konsolenpuffer, Log-Leser und Verdichtung gibt es nur für Agenten über MCP; ein Mensch sieht weder die Ausgabe der
Dienste und Prüfläufe noch die Logs.

## Ziel

`k3c-dev` hat eine Oberfläche mit Logs-Seite für Läufe und JSON-Logs. Nutzen: Entwickler sehen live, was Agenten
prüfen und was im Log steht, ohne Terminal und ohne Rohdateien.

## Beteiligte und Zielgruppen

Entwickler am Entwickler-PC (Windows); 🧑 stimmt den Abhängigkeiten zu und nimmt die Oberfläche ab.

## Anforderungen

- **Technik:** Wails v2 (`github.com/wailsapp/wails/v2`, aktuell v2.16.0) im Modul `tools/k3c-dev`; Frontend in
  `tools/k3c-dev/frontend/` mit React 19, `@radix-ui/themes` 3, Vite und TypeScript (Versionen wie im Hauptprojekt,
  wo es sie gibt), eingebettet per `//go:embed all:frontend/dist`. `wails build` erzeugt
  `tools/k3c-dev/build/bin/k3c-dev.exe`, `wails dev` startet die Entwicklung. Das Fenster ersetzt den Terminal-Start aus
  B-046; Schließen beendet Programm und MCP-Server.
- **Fenster:** Titel `K3C Dev`; nur eine Instanz (zweiter Start holt das offene Fenster nach vorn); Größe und Position
  werden beim Schließen in `<os.UserConfigDir()>/k3c/k3c-dev.json` gemerkt und beim Start wiederhergestellt.
- **Kopfzeile:** Titel, Reiter `Dienste` (B-068), `Logs` und `MCP` (MCP-Seite: B-065, bis dahin ein Hinweis), Badge
  `MCP 127.0.0.1:5180` (grün: lauscht; rot: Fehler, Grund im Tooltip), Badge `Mock` ohne Wails-Laufzeit, Schalter
  `Dark`. Reiter und Farbmodus werden gemerkt (`localStorage`).
- **Farbgebung wie die Landingpage** (`index.html`): Dark ist Standard und nutzt deren Palette (`--night-1` `#0b1026`,
  `--night-2` `#1b2550`, `--night-3` `#3b3a6b`, `--gold` `#ffd166`, `--gold-deep` `#e0a526`, `--text` `#f1f3f9`, `--dim`
  `#a9b3cf`, Karten `rgba(20, 28, 58, 0.72)` mit Linie `rgba(255, 255, 255, 0.10)`, Grün `#3fb950`); Akzentfarbe Gold,
  Fehler Rot `#ff6b6b`, Info Blau `#4dabf7` (Farben der Krone). Hell ist eine helle Variante mit denselben Akzenten. Alle
  Farben als CSS-Variablen an einer Stelle, Radix-Theme mit passender Akzent- und Grauskala.
- **Bausteine im Frontend selbst** (keine geteilte Bibliothek): Knopf mit Ladezustand, Status-Badge (Töne ok, warn,
  error, info, neutral), Hinweiskarte, Tooltip. Zahlen und Zeiten deutsch formatiert.
- **Backend-Vertrag an einer Stelle** (`src/api/`): Wails-Bindings oder, ohne Wails-Laufzeit, ein eingebauter Mock mit
  erfundenen, sich bewegenden Daten (Konsole läuft, Logs wachsen, Aufrufe kommen), damit die Oberfläche im normalen
  Browser (`npm run dev` im Frontend) testbar ist.
- **Ereignisse statt Abfrage-Intervall** für Live-Daten: `console:line`, `source:state`, `service:state`, `mcp:state`,
  `mcp:start`, `mcp:call` (die letzten beiden nutzt B-065).
- **Logs-Seite, Quellenleiste:** umschaltbare Leiste mit allen Quellen aus B-046 (`logs_sources`): Name mit
  Zustands-Punkt, Dienst nach seinem Zustand (B-067), Log-Datei grün (Einträge da) oder grau (keine), Lauf `check:<ziel>` blau (läuft), grün (ok) oder rot
  (rot oder Zeitlimit); Tooltip mit Details. Läufe erscheinen, sobald sie einmal gelaufen sind. Auswahl gemerkt; eine
  verschwundene Quelle fällt auf die erste zurück.
- **Reiter** `Konsole`, `Log`, `Fehler (verdichtet)`; `Log` und `Fehler` nur für Log-Dateien (bei Läufen deaktiviert,
  ein gemerkter Reiter fällt dann auf `Konsole` zurück). Reiter gemerkt.
- **Konsole:** Kopf mit Quellname, Zeilenzahl, `Mitlaufen: an/aus` und Knopf `Leeren` (leert nur die Anzeige); Zeilen
  mit Nummer, ANSI-Farben (16 Standardfarben als Klassen, 256 Farben und RGB als Stil), Zeilen mit `WARN` oder `ERROR`
  bekommen eine farbige Randmarke. Lädt den Puffer und hängt Live-Zeilen per Ereignis an; Zeilen, die während des Ladens
  kamen, werden über ihre Nummer einsortiert; höchstens 2000 Zeilen. Mitlaufen, solange unten; sonst Knopf `Zum Ende ↓`.
  Leerer Puffer → Hinweis.
- **Log:** Filterleiste `Mindest-Level` (Alle, DEBUG, INFO, WARN, ERROR), `Namespace`, `Suche` (regulärer Ausdruck auf
  `msg`), `Limit` (100, 200, 500), `Aktualisieren`; Textfelder 300 ms entprellt. Tabelle Zeit, Level-Badge, `ns`, `msg`;
  eine Zeile mit Daten ist aufklappbar (JSON eingerückt). Älteste oben, neueste unten; alle 2 s nachladen, solange die
  Ansicht unten steht. Fußzeile `N Einträge · X KB gelesen · läuft mit`, Warnhinweis, wenn das Budget erreicht ist
  (ältere Treffer möglich). Fehlende Datei → Hinweis.
- **Fehler (verdichtet):** Erklärsatz (gleichartige Meldungen, Zahlen, Pfade, IDs und Texte maskiert, je eine Zeile),
  Auswahl `WARN`/`ERROR`, `Aktualisieren`. Zeile: Anzahl, Level-Badge, `ns`, Beispiel (eine Zeile, gekürzt), darunter
  klein der Fingerabdruck, rechts das Zeitfenster `08:03–13:51`.
- Frontend-Logik (ANSI-Zerlegung, Einsortieren, Filter, Formatierung) liegt in reinen Funktionen mit Tests, die das
  Vitest des Hauptprojekts mitlaufen lässt.

## Nicht-Ziele

MCP-Seite (B-065); Dienste-Seite (B-068); Tasks, Commits, Releases, Einstellungen; Wails v3 (Beta); eine
Diagramm- oder Komponenten-Bibliothek außer Radix Themes; Betrieb ohne Windows.

## Regeln und Einschränkungen

Komplexitäts-Budget auch für TypeScript und CSS im Frontend (je Datei ≤ 400 Zeilen, CSS je Bereich eine Datei).
Die Oberfläche zeigt nur, gerechnet wird in Go. Neue Abhängigkeiten nur die hier genannten (Freigabe 🧑). Die Prüfung
im Browser gegen den Mock macht ein Agent nur, wenn die Session sie nennt und 🧑 sie für den Lauf freigibt.

## Beispiele

- Agent ruft `check_run npm:test` → in der Quellenleiste erscheint `check:npm:test` mit blauem Punkt, die Konsole läuft
  mit; nach dem Lauf wird der Punkt grün oder rot.
- Log von `k3c-dev`, Mindest-Level `WARN` → nur Warnungen und Fehler, neueste unten.

## Ausnahme- und Fehlerfälle

- Port belegt → Fenster öffnet trotzdem, Badge rot mit Grund, Logs-Seite funktioniert.
- Zweiter Start → kein zweites Fenster, das erste kommt nach vorn.
- Ungültiger regulärer Ausdruck in `Suche` → Meldung an der Leiste, letzte Treffer bleiben stehen.
- Unbekannter Zustand einer Quelle → neutraler Punkt statt Fehler.

## Akzeptanzkriterien

- **AC-01** `wails build` baut `k3c-dev.exe`; das Fenster startet den MCP-Server, ist einzeln und merkt Größe und
  Position; die CI baut es auf Windows.
- **AC-02** Kopfzeile, Farbmodus (Landingpage-Palette, Hell-Variante) und Mock-Modus funktionieren; im Browser läuft die
  Oberfläche gegen den Mock.
- **AC-03** Quellenleiste und Konsole zeigen Läufe und das eigene Log live über Ereignisse, mit ANSI-Farben, Einsortieren
  und 2000-Zeilen-Grenze (Tests der reinen Funktionen).
- **AC-04** `Log` filtert nach Level, Namespace, Suche und Limit, läuft mit und zeigt Budget-Hinweis; `Fehler
  (verdichtet)` zeigt die Gruppen aus B-046 (Tests der reinen Funktionen).
- **AC-05** `npm run check:dev` (Go, Frontend-Typecheck, Tests) und der CI-Job sind grün, die Grenzen sind eingehalten.

## Offene Fragen

keine

## Notizen

Aus B-046 Revision 2 abgeleitet (2026-09-30); Dienste als Quellen nach B-067. Farbgebung nach Vorgabe von 🧑 (2026-09-30, Chat).
