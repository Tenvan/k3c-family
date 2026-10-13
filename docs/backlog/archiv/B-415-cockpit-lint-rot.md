# B-415 · Lint ist rot: tools/cockpit überschreitet die Dateigrenze

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** REL
- **Erstellt:** 2026-10-12
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach dem Merge von `origin/develop` (PR #278, Cockpit-Plugin als Rohfassung) meldet `task lint` 22 Fehler und 11 Warnungen, u. a. `tools/cockpit/hooks/core.ts` mit 570 Zeilen (Grenze 400, `max-lines`), `tools/cockpit/hooks/register.tsx` (14 Treffer) und je ein Treffer in fünf Seiten unter `src/tools/` (z. B. `soundtest.ts`, `gamepadTest.ts`). `task check` ist damit rot, ohne dass ein Sprint-Commit die Ursache wäre.
Bestätigt am 2026-10-13 (GR7.5) direkt auf `develop`, nicht nur im Worktree. Zusätzlich scheitern 6 Testdateien unter `tools/cockpit/hooks/` (`assist`, `mermaid`, `panel`, `register`, `settings`, `skins`) mit „Cannot find package 'claude-code/testing'“; alle 2171 übrigen Tests sind grün.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task lint` ist auf `develop` grün: `tools/cockpit/` entweder unter die Grenzen gebracht (Dateien ≤ 400 Zeilen) oder bewusst von der Prüfung ausgenommen (`.oxlintrc.json`, mit Begründung), `soundtest.ts` ohne Befund.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

2026-10-13: `tools/cockpit` ist ein Claude-Code-Mod mit eigener Toolchain (`claude plugin validate` und `claude plugin test`; `claude-code/testing` liefert nur der Host) und kein Spiel-Code. Es steht deshalb wie `.claude` in `ignorePatterns` von `.oxlintrc.json` und im `test.exclude` von `vite.config.ts`. `task check` ist grün (Lint ohne Fehler, 2171 Tests). Der Mod selbst blieb unverändert; seine Grenzen (`register.tsx` 1455 Zeilen) gelten für ihn nicht. Prüfen des Mods bleibt bei `claude plugin test` laut `tools/cockpit/DEVELOPMENT.md`.
