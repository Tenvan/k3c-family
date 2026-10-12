# B-415 · Lint ist rot: tools/cockpit überschreitet die Dateigrenze

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** REL
- **Erstellt:** 2026-10-12
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach dem Merge von `origin/develop` (PR #278, Cockpit-Plugin als Rohfassung) meldet `task lint` im Worktree 21 Fehler und 10 Warnungen, u. a. `tools/cockpit/hooks/core.ts` mit 570 Zeilen (Grenze 400, `max-lines`) und `src/tools/soundtest.ts:117` (`no-unused-expressions`). `task check` ist damit rot, ohne dass ein Sprint-Commit die Ursache wäre. Ungeprüft: ob die Fehler schon auf `develop` bestehen oder nur im Worktree auftreten (dort fehlen installierte Pakete in `tools/k3c-dev/frontend`).

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

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
