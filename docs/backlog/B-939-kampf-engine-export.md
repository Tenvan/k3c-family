# B-939 · Die Engine exportiert Preis, Zahlstand und Zahlbarkeit je Kampf-Zahlstelle und Spieler sowie die Kronen-Zustände

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** KS6
- **Projekt:** KSY
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

KS5-Review: `native/k3c-game/props/plan_combat.go` baut den Zahl-Schlüssel der Engine nach (`payKeyIs`) und liest Preise selbst aus `data/economy.json` (`combatPrices`); `view/crown.go` spiegelt die Kronen-Zustände aus `engine/sim/crown.go`, die unexportiert sind.

## Ziel

Das Spiel liest Preis, Zahlstand und Zahlbarkeit je Zahlstelle und Spieler sowie die Kronen-Zustände aus der Engine-API und rechnet nichts nach.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `native/k3c-game/**`
- **Abhängig von:** B-934
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –
- Sim-Code im eigenen Worktree (B-925).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Exportierte Engine-Funktion bzw. Felder für Preis, Zahlstand, Zahlbarkeit je Spieler; `payKeyIs`/`combatPrices` entfallen (Test).
- **AC-02** Kronen-Zustände exportiert, `view/crown.go` nutzt sie.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
