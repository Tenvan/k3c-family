# B-716 · Reittiere haben eigene Silhouetten, Füße und einen einheitlichen Maßstab

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** MG9
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-07 (auf Ralfs Zustimmung im Chat)

## Ausgangslage

D-MG2-06: Weil Horn, Geweih und Ohren den Rahmen sprengten, sind die Reittiere ~4 % unterschiedlich skaliert. D-MG2-08/PS1.3: Wolf, Höllenhund, Elefant wirken wie umgefärbte Pferde.

## Ziel

Jedes Reittier ist an der Silhouette erkennbar, alle im selben Maßstab.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Rahmen wächst ohne Skalierung (einheitlicher Maßstab der Gruppe).
- Eigene Wolfssilhouette, Füße bzw. Pfoten je Tier, Gangarten je Tier mit dem Rig aus B-715.

## Nicht-Ziele

Neue Reittiere.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Alle Reittiere haben dieselbe Skala (px je Einheit), Test in `tools/paper`.
- **AC-02** Wolf, Höllenhund, Elefant mit eigener Silhouette und Fußform (Sichtprüfung).
- **AC-03** Elefant ohne `galopp`.

## Offene Fragen

keine

## Notizen

Quelle: PS1.1 ✏️ D-MG2-06, D-MG2-08; PS1.3 Punkt 2. Nach B-712, B-715. Lane `reittiere`.
