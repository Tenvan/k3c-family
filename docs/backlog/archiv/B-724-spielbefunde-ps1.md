# B-724 · Im Spiel stehen Bogenschützen auf der Plattform, Reiten passt zum Tempo, Bäume und Tor haben die richtige Größe

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** US3
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

PS1.4 im Spiel: Bogenschütze schwebt über dem Turm ([Bild](../playtests/ps1/ps1.4-bogenschuetze-turm.webp)); Reit-Animation passt nicht zur Laufgeschwindigkeit; auf dem Pferd kein Idle; Bäume zu klein ([Bild](../playtests/ps1/ps1.4-baeume-hintergrund.webp)); Tor/Portal zu klein am berittenen Monarchen ([Bild](../playtests/ps1/ps1.4-tor-portal.webp)).

## Ziel

Das Spiel zeigt die Papier-Grafik in stimmigen Größen und Bewegungen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Bogenschütze steht auf der Plattform des Turms, auch nach Gebäudewechsel (Höhe aus dem Gebäude-Asset).
- Frame-Rate der Reit-Animation folgt dem Tempo (keine rutschenden Hufe).
- Stand beritten zeigt `reiten-idle` (B-717).
- Bäume in Spielhöhe (B-723), Tor und Portal größer als der berittene Monarch.

## Nicht-Ziele

Assets selbst (B-717, B-723).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Bogenschütze auf jeder Turmstufe auf der Plattform (Screenshot je Stufe).
- **AC-02** Hufe rutschen bei schritt/trab/galopp nicht (Sichtprüfung im Spiel).
- **AC-03** Stand beritten zeigt Idle.
- **AC-04** Bäume ≈ 360 px, Tor und Portal überragen den berittenen Monarchen.

## Offene Fragen

keine

## Notizen

Quelle: PS1.4 Spiel ✏️. Nach B-717, B-723.

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). Befunde aus dem Browser-Spiel; Größen und Bewegungen im nativen Spiel prüfen dessen Sprints.
