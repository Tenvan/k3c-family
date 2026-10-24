# B-493 · Die Credits-Seite zeigt auch die Sounds unter public/audio

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SO6
- **Projekt:** SND
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Credits-Seite `lizenzen.html` liest über `src/tools/credits.ts` nur `public/grafik/CREDITS.md` und `public/sprites/CREDITS.md`. Die Sounds unter `public/audio/` haben ihre Credits in `public/audio/README.md` › Importe (k3c-workbench, Dateiliste in `tools/assetkit/catalog.go`); sie erscheinen nicht auf der Seite, und kein Test prüft ihre Vollständigkeit dort.

## Ziel

Die Credits-Seite nennt auch die Sounds, CC-BY-Stücke mit Urheber, Titel, Lizenz und Quelle; ein Test prüft, dass jede Datei unter `public/audio/` einen Eintrag hat.

## Beteiligte und Zielgruppen

Spieler und Entwickler lesen `lizenzen.html`; der Agent baut es; 🧑 bestätigt die Lizenzen in k3c-workbench.

## Anforderungen

- `src/tools/credits.ts` liest zusätzlich die Credit-Tabelle der Sounds (`public/audio/README.md`, Tabellen mit Kopf `| Ordner |`) mit eigener Wurzel `audio`.
- `src/tools/credits.test.ts` prüft Vollständigkeit und Verweise auch für `public/audio/` (Dateien unter `kandidaten/` und weitere Ordner).
- Selbst erzeugte Töne (`atlas`, `probe-*`, `public/audio-test/`) brauchen keinen Eintrag.

## Nicht-Ziele

Sound-Auswahl und Katalog (SO2), Lizenz-Bestätigung (🧑 in k3c-workbench).

## Regeln und Einschränkungen

Domäne PLAT (`src/tools/`, `lizenzen.html`); eine Credit-Datei je Wurzel (B-165); Komplexitäts-Budget; Format der Credit-Tabellen ändert dieses Ticket nicht.

## Beispiele

Eine neue OpenGameArt-Datei steht in `public/audio/README.md` → `lizenzen.html` zeigt Titel, Urheber, Lizenz und Quelle; fehlt der Eintrag, wird `task test` rot.

## Ausnahme- und Fehlerfälle

Tabellenzeile ohne Urheber oder Lizenz → Test scheitert mit dem Ordnernamen; Datei ohne Eintrag → Test nennt sie.

## Akzeptanzkriterien

- **AC-01** `lizenzen.html` zeigt jede Credit-Zeile der Sounds, CC-BY-Stücke hervorgehoben (Test `credits.test.ts`).
- **AC-02** Der Test scheitert, wenn eine Datei unter `public/audio/` (außer selbst erzeugten) keinen Eintrag hat.

## Offene Fragen

- Soll die Credit-Datei der Sounds weiter `public/audio/README.md` sein (so schreibt k3c-workbench sie, `tools/assetkit/catalog.go`) oder eine eigene `CREDITS.md` (so nennt es SO2.2)? Entscheidet 🧑; bis dahin gilt README.md.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
