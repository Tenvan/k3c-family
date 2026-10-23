# B-566 · Lobby bietet beim Anlegen eines Spielstands eine Welt-Auswahl

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** BD8
- **Projekt:** BED
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-534 (KR1.1): Statt „spiel-N ohne Welt-Auswahl“ soll die Lobby beim Anlegen eines Spielstands eine Welt-Auswahl anbieten. Heute heißt ein neuer Stand `spiel-N`, andere Welten gibt es nur im Dev-Mode (BD5.2).

## Ziel

Die Lobby zeigt beim Anlegen eines Spielstands die Welten, die der Server kennt; der neue Stand startet in der gewählten Welt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Server liefert die Liste der wählbaren Welten (neue API, Protokoll-Version prüfen).
- Lobby zeigt die Auswahl, per Gamepad bedienbar (B nicht belegen).
- Namensvergabe `spiel-N` bleibt (freie Namenseingabe ist nicht gewählt).
- Texte zentral in `src/core/texts.*.ts`.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Beim Anlegen kann die Welt gewählt werden; der Stand trägt sie und die Lobby zeigt sie.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-534 (KR1.1, Alternative A). Spec noch Entwurf.
