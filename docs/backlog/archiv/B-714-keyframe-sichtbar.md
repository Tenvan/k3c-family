# B-714 · Keyframes schalten Teile mit dem Feld sichtbar ein und aus

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RD2
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-07, Chat (mit Sprint RD2)

## Ausgangslage

D-MG3-07, D-MG4-06: Teile wie der Pfeil des Bogenschützen oder die Spritzer des Lavaschleims sind in jeder Animation sichtbar, weil Keyframes keine Sichtbarkeit kennen.

## Ziel

Teile erscheinen nur, wo die Animation sie braucht.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Keyframe-Feld `sichtbar` (bool) je Teil im Spec-Vertrag und im Renderer.
- Bogenschützen (Truppen, Goblin) und Lavaschleim nachziehen.

## Nicht-Ziele

Ein- und Ausblenden mit Übergang.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Schema und `paper-spec.md` kennen `sichtbar`; Test: Teil mit `sichtbar: false` fehlt im Frame.
- **AC-02** Pfeil der Bogenschützen nur in `angriff` bis zum Abschuss sichtbar.
- **AC-03** Lavaschleim-Spritzer nur in `angriff`.

## Offene Fragen

keine

## Notizen

Quelle: PS1.1 ✏️ D-MG3-07, D-MG4-06. Lane `renderer`.
