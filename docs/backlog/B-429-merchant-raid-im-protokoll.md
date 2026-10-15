# B-429 · Protokoll und Client-Typen nennen merchant.raid

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Server sendet beim Händler `raid: true`, wenn der Besuch überfallen wird (`engine/sim/merchant.go`, `json:"raid,omitempty"`). `docs/protocol.md` › Wirtschaft (Tabelle `merchant`) und `Merchant` in `src/model/types.ts` nennen das Feld nicht. K5.1 liest es in `src/scenes/kampfAnzeige.ts` über einen lokalen Typ (`KampfWorld`) für das Event-Banner „Händler-Überfall“, weil `src/model/` dort ein Nicht-Ziel war.

## Ziel

`merchant.raid` steht in `docs/protocol.md` und als `raid?: boolean` in `Merchant` (`src/model/types.ts`); der lokale Zusatztyp in `kampfAnzeige.ts` entfällt.

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
