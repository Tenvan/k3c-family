# B-324 · Die Client-Typen der Gegner- und Wellendaten passen zu den JSON-Dateien

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

K1.1 hat das globale `attacksPerSecond` aus `data/waves.json` entfernt und in `data/enemies.json` je Gegner `attacksPerSecond`, `aoe`, `swarmSize`, `phases` und `kiteDistance` ergänzt. `src/model/data.ts` deklariert `WAVES.attacksPerSecond` weiter als `number` (per `as unknown as`, der Typcheck merkt es nicht), und `EnemyData` kennt die neuen Felder nicht. Kein Client-Code liest sie heute.

## Ziel

Die Client-Typen beschreiben die Daten so, wie sie in `data/` stehen; niemand verlässt sich auf ein Feld, das es nicht mehr gibt.

## Beteiligte und Zielgruppen

Entwickler am Client (Anzeige K5).

## Anforderungen

- `WAVES` ohne `attacksPerSecond`; `EnemyData` mit den neuen Feldern als optionale Werte.

## Nicht-Ziele

Anzeige der Traits (K5).

## Regeln und Einschränkungen

`src/model` enthält nur Typen und Daten, keine Logik.

## Beispiele

`WAVES.attacksPerSecond` im Client-Code → Typfehler.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Typ-Anpassung.

## Akzeptanzkriterien

- **AC-01** `src/model/data.ts` enthält kein `attacksPerSecond` in `WAVES`, `EnemyData` die Felder aus `data/enemies.json`; `task check` grün.

## Offene Fragen

keine

## Notizen

Gefunden in K1.1 (Suche nach Lesern von `attacksPerSecond`).
