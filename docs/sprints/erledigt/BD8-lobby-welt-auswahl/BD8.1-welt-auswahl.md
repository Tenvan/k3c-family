# BD8.1 · Welt-Auswahl im Lobby-Dialog Neuer Spielstand

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** bd8/1-welt-auswahl
- **Abhängig von:** –
- **Tickets:** B-566
- **Kriterien:** AC-01

## Ziel

Der Dialog „Neuer Spielstand“ bietet eine Welt-Auswahl.

## Kontext

B-566; Server liefert `GET /api/worlds` (`docs/protocol.md` › HTTP: Welten).

## Erlaubte Dateien

- `engine/room`, `engine/net`, `docs/protocol.md`
- `src/core/saveStore.ts`, `src/scenes/lobby*`, `src/core/texts.*`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Weltliste vom Server. 2. Dialogzeile Welt. 3. Tests.

## Fertig, wenn

- [x] AC-01: Welt wählbar beim Anlegen, Gamepad und Tastatur.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 umgesetzt und geprüft: `go test ./engine/net ./engine/room`, `npx vitest run src/scenes src/core` (982 grün), `tsc`, oxlint. Annahme: Ohne Dev-Mode listet der Server nur `standard`, die Auswahl ist im Normalbetrieb einelementig; mehrere Welten ohne Dev-Mode wären eine Regeländerung (Frage an 🧑). Layout nicht im Browser geprüft.
