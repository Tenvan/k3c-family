# B-524 · skillHeld beim Tod zurücksetzen, damit der erste Druck nach dem Respawn castFailed meldet

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`stepSkills` (engine/sim/skills.go) merkt den gehaltenen Slot in `Player.skillHeld` (B-378). Der Tod-Zweig in `engine/sim/economy.go` (`continue` vor `stepSkills`) setzt das Feld nicht zurück. Reviewer-Befund aus BD2.1.

## Ziel

Nach dem Respawn meldet der erste neue Tastendruck ohne Ziel wieder `castFailed`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Im Tod-Zweig `p.skillHeld = 0` setzen (eine Zeile, `engine/sim/economy.go`).
- Deterministisch, 2 Spieler unabhängig.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Sim-Test: Spieler hält Skill 1 ohne Ziel (ein `castFailed`), stirbt, lebt wieder, drückt neu → ein zweites `castFailed`; `task check:go` grün.

## Offene Fragen

keine

## Notizen

BD2.1 durfte `economy.go` nicht ändern (Erlaubte Dateien); daher als Ticket.
