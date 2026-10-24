# B-577 · Der Dieb bekommt Backstab und Smoke Bomb

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** SK3
- **Projekt:** SKL
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-560 (KR5.1): Variante (b), die Dieb-Skills werden jetzt gebaut. Heute ist `presets.thief.activeSkills` leer (SK2.2), `backstab` und `smokeBomb` fehlen in `data/monarch.json › skills` (B-504); `docs/rules/monarch.md` sagt „der Dieb folgt später als Inhalt“.

## Ziel

Der Dieb hat Backstab und Smoke Bomb als lernbare aktive Skills; das Preset Dieb nennt sie.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Backstab: 40 Schaden, 80 von hinten, Abklingzeit 12 s (Vorgabe aus game-design, Werte nur in `data/monarch.json`).
- Smoke Bomb: unsichtbar 5 s, Abklingzeit 25 s.
- Neue Wirkungstypen `backstab` und `stealth` in `engine/sim`; Verhalten der Gegner gegen Unsichtbare (kein Ziel) mit 🧑 klären.
- Golden-Änderung mit Begründung (`task go:golden`), BAL-Lauf über `sim_test`; zwei Spieler gleichzeitig.
- Regeltext `docs/rules/monarch.md` nachziehen (Dieb nicht mehr „später“).
- Darstellung kommt über B-578.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `presets.thief.activeSkills` nennt `backstab` und `smokeBomb`; der Preset-Test aus SK2.2 bleibt grün.
- **AC-02** Sim-Tests für beide Wirkungen; Golden mit Begründung; BAL-Lauf ohne Auffälligkeit.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-560 (KR5.1, Variante b). Spec noch Entwurf; Regel- und Balancing-Wirkung, Werte und Unsichtbarkeits-Regel bestätigt 🧑 vor der Freigabe. Neuer SIM-Sprint im Projekt SKL.
