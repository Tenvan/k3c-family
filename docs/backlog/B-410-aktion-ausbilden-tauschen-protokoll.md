# B-410 · Aktionen-Overlay zeigt Ausbilden und Tauschen nicht, der Server liefert sie nicht in actions

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WRT
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W6.3 (B-126/AC-02) soll Ausbilden und Tauschen im Aktionen-Overlay zeigen. `PlayerAction` kennt nur `attack`, `skill`, `learn`, `respec`; `docs/protocol.md` hat keine Aktion für Ausbilden oder Tauschen (nur das Zahlziel am Händler über `buyPaid`). Der Client erfindet nichts, die Einträge sind in W6.3 `verschoben`.

## Ziel

Der Server nennt Ausbilden (Beruf) und Tauschen (Händler) als Einträge in `actions`; danach zeigt das Aktionen-Overlay sie (`actionHints.ts`, CLI).

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
