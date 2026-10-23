# B-534 · Entscheidung: Spielmenü ist die Lobby, neuer Spielstand ohne Welt-Auswahl

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** KR1
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-506 ließ offen, ob „Spielmenü“ die Lobby oder das Pausenmenü im Spiel meint und ob es beim Neuanlegen eine Welt-Auswahl gibt.

## Ziel

Ralf bestätigt oder ändert den autonom gewählten Umfang.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Default (umgesetzt in BD5.2): Spielstände anlegen und löschen in der Lobby; die Lobby zeigt die Start-Welt (`?world=`, sonst `standard`) und je Spielstand seine Welt; das Pausenmenü im Spiel zeigt die Welt des Raums. Neuer Spielstand heißt `spiel-N` (nächste freie Zahl), keine Welt-Auswahl (andere Welten nur im Dev-Mode).
- Alternative A: Welt-Auswahl in der Lobby (Liste der Welten vom Server, neue API).
- Alternative B: Namen frei eingeben (Bildschirmtastatur auf der Xbox).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ralf hat Default oder Alternative gewählt; bei Abweichung gibt es ein Umsetzungs-Ticket.

## Offene Fragen

- Welcher vierte Punkt der Meldung aus WZ18.8 war gemeint? Entscheidet Ralf.

## Notizen

–
