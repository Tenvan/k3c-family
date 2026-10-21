# B-516 · Prompt-Grundgerüst des Studio-Chats ist eine feste Instruktion im Code statt Text im Eingabefeld

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ21
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme WZ19.5 (2026-10-20, Ralf im Chat): Der Button „Prompt-Grundgerüst“ schreibt das Gerüst (Format, Regeln, Daten) heute ins Eingabefeld, Ralf ergänzt seine Wünsche und sendet alles zusammen. Gewünscht: Das Gerüst gehört als feste Instruktion in den Code. Der Chat schickt es bei Welt-Aufträgen selbst mit, und Ralf schreibt nur seinen Wunsch („Erstelle eine Welt Nordsee“). Aufruf der CLI siehe `tools/k3c-studio/backend/chat_cli.go` (`chatArgs`, z. B. `--append-system-prompt`, Verfügbarkeit in der CLI-Version prüfen); Gerüst heute im Frontend des Welten Editors (WZ19.3).

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
