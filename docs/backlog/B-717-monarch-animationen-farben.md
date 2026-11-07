# B-717 · Monarch hat Reit-Idle, Gangarten je Reittier, give/hit/death/respawn und vier gut unterscheidbare Spielerfarben

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** MG9
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

PS1.4: Auf dem Pferd ist kein Idle zu sehen. D-US2-03/D-US1-02: Das Monarch-Asset kennt nur das Pferd. D-US1-03: `give`, `hit`, `death`, `respawn` fallen auf `idle` zurück. PS1.3: Spieler 4 (`leder`) verschwimmt auf dem braunen Pferd; die Satteldecke bleibt blau (B-711).

## Ziel

Monarch ist in jeder Lage animiert und die vier Spieler sind klar unterscheidbar.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Animation `reiten-idle` (Stand auf dem Reittier).
- `<gangart>-<reittier>` für alle Reittiere (nach B-716).
- Keyframes `give`, `hit`, `death`, `respawn` für Monarch und Truppen.
- Spieler 4 kontrastreiche Farbe statt `leder`; Satteldecke folgt der Rockfarbe (löst B-711).

## Nicht-Ziele

Einbau ins Spiel (B-724).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `figuren/monarch` hat `reiten-idle` und je Reittier `schritt/trab/galopp-<id>`.
- **AC-02** Monarch und Truppen haben `give`, `hit`, `death`, `respawn`.
- **AC-03** Varianten spieler1–4: Rock und Satteldecke in vier unterscheidbaren Farben, keine auf Braun (Sichtprüfung).

## Offene Fragen

keine

## Notizen

Quelle: PS1.1 ✏️ D-US2-03, D-US1-02, D-US1-03; PS1.3 Punkt 1; PS1.4 (Reit-Idle). Schließt B-711. Lane `monarch`.
