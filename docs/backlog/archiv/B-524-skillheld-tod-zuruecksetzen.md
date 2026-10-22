# B-524 · skillHeld beim Tod zurücksetzen, damit der erste Druck nach dem Respawn castFailed meldet

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** BD7
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22 (mit BD7)

## Ausgangslage

`stepSkills` (engine/sim/skills.go) merkt den gehaltenen Slot in `Player.skillHeld` (B-378). Der Tod-Zweig in `engine/sim/economy.go` (`continue` vor `stepSkills`) setzt das Feld nicht zurück. Reviewer-Befund aus BD2.1.

## Ziel

Nach dem Respawn meldet der erste neue Tastendruck ohne Ziel wieder `castFailed`.

## Beteiligte und Zielgruppen

Spielende (Rückmeldung beim Skill-Druck); Agent in SIM.

## Anforderungen

- Im Tod-Zweig `p.skillHeld = 0` setzen (eine Zeile, `engine/sim/economy.go`).
- Deterministisch, 2 Spieler unabhängig.

## Nicht-Ziele

Andere Laufzeit-Felder beim Tod.

## Regeln und Einschränkungen

Domäne SIM, deterministisch, Golden unverändert; Feld `skillHeld` ist ohne JSON (kein Spielstand, kein Protokoll).

## Beispiele

Skill 1 ohne Ziel gehalten (`castFailed`), Monarch stirbt mit gehaltener Taste, lebt wieder, Taste neu gedrückt → zweites `castFailed`.

## Ausnahme- und Fehlerfälle

Taste während des ganzen Todes gehalten: nach dem Respawn zählt das weitere Halten als neuer Druck (ein `castFailed`), Default autonom; danach wieder eins je Druck.

## Akzeptanzkriterien

- **AC-01** Sim-Test: Spieler hält Skill 1 ohne Ziel (ein `castFailed`), stirbt, lebt wieder, drückt neu → ein zweites `castFailed`; `task check:go` grün.

## Offene Fragen

keine

## Notizen

BD2.1 durfte `economy.go` nicht ändern (Erlaubte Dateien); daher als Ticket.
