# B-579 · Wellenabstand unter Tage mit besetzten Adern liegt über dem Korridor 4–12 Minuten

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Messung aus KB3 (B-565, 2026-10-23): Über 100 Seeds mit 2 besetzten Adern kommt die erste Welle bei Median 19,37 min, ohne Adern in 60 min keine. Der Korridor 4–12 min ist verletzt. Alle Seeds liefern denselben Wert, weil die Adern in der Messung auf den Hub gesetzt sind.

## Ziel

Der Wellenabstand unter Tage liegt mit besetzten Adern im Korridor 4–12 Minuten.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Ursache in der Aggressionsrate (1 %/min) und der Ader-Gewichtung klären; Werte nur in `data/*.json`.
- Messung mit echten Ader-Entfernungen und Folgewellen, Lauf über `sim_test`.
- Entscheidung 🧑: Werte ändern oder Korridor anpassen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Gemessene erste Welle und Folgewellen mit besetzten Adern liegen über 100 Seeds im Korridor (Messbericht in den Notizen).

## Offene Fragen

Werte ändern oder Korridor anpassen? Entscheidet 🧑 (Projekt BAL ruht, Balancing zurückgestellt, B-359).

## Notizen

Gefunden in KB3 (B-565). Messtest: `engine/sim/aggression_veins_seeds_test.go` mit `K3C_MESSUNG=1`.
