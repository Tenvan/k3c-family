# B-825 · Die CI führt die Tests wieder aus (seit #531 abgeschaltet)

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ41
- **Projekt:** WZG
- **Erstellt:** 2026-11-12
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-12 (Dialog)

## Ausgangslage

Seit #531 (5028b0f48, 2026-11-08, „CI ohne Tests (vorläufig)“) laufen in `.github/workflows/ci.yml` keine Tests mehr: Job check ohne `task test`, Job go ohne `task go:test`/`go:race` (nur gen:paper, Lint, Cross-Build), go-arm64 `if: false`, k3c-workbench ohne `go test`. Kein Ticket verfolgt das Wiedereinschalten (Befund WZ41, 2026-11-12). B-756/B-757 sind damit nur formal grün, B-758 (Release mit Tests) wäre ein Blindflug.

## Ziel

Die CI prüft auf `develop` wieder Tests: Client (Vitest), Go (Wurzel, k3c-workbench, natives Spiel), ohne Sim-, Balance- und Lasttest-Pakete (Sperre 2026-11-11).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Tests je Job wieder an, Grafik-abhängige Tests nach `gen:paper` (B-756).
- Sim/Balance-Pakete bleiben ausgenommen (wie `tools/checkdiff` › `simDirs`).
- Laufzeit je Job dokumentiert; zu langsame Tests als Tickets.

## Nicht-Ziele

Race-Lauf, Sim/Balance, Release-Workflow (B-758).

## Regeln und Einschränkungen

Die Abschaltung war eine bewusste Maßnahme (#531); Wiedereinschalten entscheidet 🧑.

- **Bereich:** `.github/workflows/ci.yml`
- **Abhängig von:** WZ42 (schnelle, stabile Workbench-Tests)
- **Braucht 🧑:** Entscheidung (wann wieder einschalten)
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** CI-Lauf auf `develop` führt Vitest und `go test -short` der drei Go-Module aus und ist grün.
- **AC-02** Sim/Balance-Pakete sind ausgenommen und im Workflow begründet.
- **AC-03** Laufzeit je Job im Ergebnis.

## Offene Fragen

Entschieden 🧑 Ralf, 2026-11-12 (Dialog): Wiedereinschalten **nach WZ42** (schnelle, stabile Workbench-Tests).

## Notizen

Gefunden von WZ41 (2026-11-12).
