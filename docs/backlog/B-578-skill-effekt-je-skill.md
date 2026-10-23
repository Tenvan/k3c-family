# B-578 · Jeder Skill hat einen eigenen Effekt statt des Rings je Klasse

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** SKL
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-561 (KR5.1): Variante (b), jeder Skill bekommt einen eigenen Effekt. Heute zeigt SK2.3 einen Ring in der Farbe der Linie (`LINE_COLORS`) beim Wirken eines aktiven bzw. Lernen eines passiven Skills.

## Ziel

Jeder Skill zeigt im Spiel einen eigenen Effekt; der Ring je Klasse bleibt als Rückfall.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Tabelle Skill → Effekt statt `LINE_COLORS` (CLI), etwa 24 Einträge, bei B-577 samt Backstab und Smoke Bomb.
- Keine neuen Ereignisse: Der Client erkennt Wirken und Lernen wie in SK2.3 (Anstieg der Abklingzeit, neuer Eintrag in `skills`).
- Skill ohne Eintrag: Ring je Klasse; ein Test prüft, dass jeder Skill aus `data/monarch.json` Effekt oder Rückfall hat.
- Zwei Spieler gleichzeitig; Effekte nach Grafikstil (DB32), Abnahme am TV durch 🧑 (SK2.5).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Jeder Skill hat einen eigenen Effekt oder den festgelegten Rückfall; Test prüft jede Linie aus `data/monarch.json`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-561 (KR5.1, Variante b). Spec noch Entwurf; Effekt-Look je Skill mit 🧑 abstimmen. Reihenfolge: nach B-577, damit die Dieb-Skills gleich mitgehen.
