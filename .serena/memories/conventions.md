# Conventions

## Limits (hart, geprüft)
- Datei ≤ 400 Zeilen (`tests/projectRules.test.ts`), Funktion ≤ 60 Zeilen (funlen), gocyclo ≤ 15, Verschachtelung geprüft in `engine/internal/rules/nesting_test.go`.
- Go-Schichten (depguard): `engine/` kennt kein `cmd/`; `sim`/`level`/`rng` kennen weder `room` noch `net`; `room` kennt kein `net`; `engine/` und `cmd/k3c-server` importieren nichts aus `tools/`.
- Komplexitäts-Budget und Domänen: `docs/arbeitsweise.md`. Klein bleiben, kein Framework-Overhead.

## Naming
- Datei-/Ordnernamen im Code englisch, Gruppen-Präfixe in `engine/sim`. Begriffe aus `docs/glossar.md` verbindlich; neue Begriffe dort zuerst eintragen.
- Welt-Koordinaten in Units (1 Unit = `UNIT_PX` = 32 px). Keine Sprünge, nur horizontal.

## Seiten & Navigation (jede Seite außer Landing)
- `installPageChrome()` aus `src/core/shell.ts` aufrufen, oben ~70 px frei lassen.
- Spielerseiten in `src/landing/pages.ts`, Werkzeugseiten in `src/tools/devTiles.ts` eintragen.
- Vollbild nur `toggleFullscreen()` (`src/core/fullscreen.ts`); Seitenwechsel nur `openPage()`, zurück `goHome()`.

## Input
- Spiel-Code fragt Aktionen über `PlayerInput` ab, nie Tasten. Controller-Taste B nie belegen; View+Menu = zurück zur Landing. Jede Mechanik muss mit 2 Spielern gleichzeitig funktionieren.

## Wails-Frontends
- Radix nur über `@webkit`, rohe Controls nur in `atoms/`, Importe nur nach unten (atoms → molecules → organisms → templates); geprüft von `tests/atomicDesign.test.ts`.

## Logging
- Jede Log-Meldung (Go `slog`, Client `clientLog`) beginnt mit Emoji zur Aktion (🚀 Start, 🛑 Stopp, ✅, ❌, 💥, 💾 …; Liste in CLAUDE.md). Nur Ein-Codepoint-Emojis, kein U+FE0F. Neuer `ns` braucht Emoji in `engine/conlog` › `Topics`.

## Planung
- Tickets/Sprints/Sessions nur als Kopie von `docs/vorlagen/*.md` (geprüft von `tests/planning.test.ts`), Änderung per MCP `plan_*`. Neue Ideen sofort als Ticket in `docs/backlog/`, nicht nebenbei umsetzen.
