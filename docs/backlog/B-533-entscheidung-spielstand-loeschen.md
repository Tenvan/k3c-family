# B-533 · Entscheidung: Spielstand löschen per HTTP ohne Token, Stand bleibt als Sicherung

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BD5 (B-506) braucht Löschen von Spielständen aus dem Spielmenü. Bisher gibt es nur `store.Saves.Delete` (entfernt Stand **und** Sicherungen, intern für Testräume) und keine HTTP-Schnittstelle. `GET/POST /api/save` laufen ohne Token.

## Ziel

Ralf bestätigt oder ändert die autonom gewählte Lösch-Semantik.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Default (umgesetzt in BD5.1): `DELETE /api/save?slot=NAME` ohne Token (wie POST), Protokoll-Version bleibt 8. Der Stand wandert in die rotierenden Sicherungen `saves/backups/NAME/` (wiederherstellbar mit `/api/save/restore`), ältere Sicherungen bleiben. In einem offenen Raum geladen → `409`.
- Alternative A: endgültig löschen samt Sicherungen.
- Alternative B: Löschen nur mit Token oder nur im Dev-Mode.
- Alternative C: eigene WebSocket-Nachricht `deleteSave` (Protokoll v9).

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

- Default bestätigen? Entscheidet Ralf.

## Notizen

Default konservativ: kein Datenverlust, ein versehentliches Löschen ist über die Sicherungen umkehrbar. Rotation behält 5 Sicherungen je Slot (BackupKeep).
