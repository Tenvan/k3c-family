# B-455 · Truhe gibt es auch geschlossen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB6 liefert nur die offene Truhe mit Gold (`truhe.json`). Ralf (2026-10-17): Truhen soll es auch geschlossen geben (in der Vorschau fehlte der Eindruck). Offen: Wann zeigt das Spiel welche (z. B. Verschluss, Beute-Stufen)?

## Ziel

Eine geschlossene Truhe liegt als Variante der Truhe vor (Vorschau) und ist im Spiel zugeordnet, sofern das Spiel den Zustand kennt.

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

- **AC-01** `truhe` hat eine geschlossene Variante (Daten-JSON, PNG 16/32/64/128), Vorschau zeigt beide.
- **AC-02** Spiel-Zuordnung oder begründet verschoben (Server kennt keinen Zustand geschlossen: dann Ticket SIM).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
