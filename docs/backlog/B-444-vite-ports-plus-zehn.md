# B-444 · Vite-Ports von Spiel und k3c-dev liegen 10 höher und kollidieren nicht mit dem Vite-Standardport

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-16, Chat): „vite Port kollidiert mit anderen, setze 10 höher“. Gemessen: Das Spiel-Vite lauscht auf `:::5173` (alle Adressen), ein Vite aus `ErpApi\tools\go\averp-migrate\frontend` mit Vite-Standardport auf `::1:5173`; `localhost:5173` trifft damit das fremde Projekt. Das k3c-dev-Vite (5181) kollidierte nicht. Entscheidung Ralf (Chat): beide um 10 hoch.

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

- **AC-01** Spiel-Vite der Repo-Wurzel auf 5183, Worktrees auf 5193, 5203 … (Versatz 10 bleibt, Basis aus `services.json`); `vite.config.ts`, Taskfile, `services.json`, Fallbacks im Code und aktuelle Doku nennen 5183.
- **AC-02** k3c-dev-Frontend-Vite auf 5191 (`vite.config.ts`, `.claude/launch.json`, README, Codemaps); 5191 liegt auf keinem Worktree-Slot (5183 + 10·n).
- **AC-03** `task check` und `check_run dev:test` grün (bis auf vorbestehende Fehler, B-440).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
