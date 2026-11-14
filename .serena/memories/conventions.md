# Conventions

## Limits (hart, geprüft)
- Datei ≤ 400 Zeilen (`tests/projectRules.test.ts`), Funktion ≤ 60 Zeilen (funlen), gocyclo ≤ 15, Verschachtelung geprüft in `engine/internal/rules/nesting_test.go`.
- Go-Schichten (depguard): `engine/` kennt kein `cmd/`; `sim`/`level`/`rng` kennen kein `room`; `engine/` importiert nichts aus `tools/`.
- Komplexitäts-Budget und Domänen: `docs/arbeitsweise.md`. Klein bleiben, kein Framework-Overhead.

## Naming
- Datei-/Ordnernamen im Code englisch, Gruppen-Präfixe in `engine/sim`. Begriffe aus `docs/glossar.md` verbindlich; neue Begriffe dort zuerst eintragen.
- Welt-Koordinaten in Units (1 Unit = 32 px, `unitPx` im nativen Spiel). Keine Sprünge, nur horizontal.

## Input
- Spiel-Code fragt Aktionen ab, nie konkrete Tasten (`native/k3c-game/input`). Controller-Taste B geht in jedem Menü zurück, im laufenden Spiel unbelegt. Jede Mechanik muss mit 2 Spielern gleichzeitig funktionieren.

## Wails-Frontends
- Radix nur über `@webkit`, rohe Controls nur in `atoms/`, Importe nur nach unten (atoms → molecules → organisms → templates); geprüft von `tests/atomicDesign.test.ts`.

## Logging
- Jede Log-Meldung (Go `slog`) beginnt mit Emoji zur Aktion (🚀 Start, 🛑 Stopp, ✅, ❌, 💥, 💾 …; Liste in CLAUDE.md). Nur Ein-Codepoint-Emojis, kein U+FE0F. Neuer `ns` braucht Emoji in `engine/conlog` › `Topics`.

## Planung
- Tickets/Sprints/Sessions nur als Kopie von `docs/vorlagen/*.md` (geprüft von `tests/planning.test.ts`), Änderung per MCP `plan_*`. Neue Ideen sofort als Ticket in `docs/backlog/`, nicht nebenbei umsetzen.
