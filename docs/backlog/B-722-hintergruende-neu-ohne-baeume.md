# B-722 · Hintergründe sind neu gestaltet, mit Tiefe und ohne Bäume

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** MG11
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

PS1.3: Hintergründe flach, Höhlenfelsen wie Kuppelzelte, keine Rückwand im Eisenstollen, Glut wie rote Vögel; D-MG7-06 und Varianten v3–v5 (D-MG7-01) offen. PS1.4: Bäume im Hintergrund sind von rodbaren Bäumen kaum zu unterscheiden, deshalb keine Bäume im Hintergrund ([Bild](../playtests/ps1/ps1.4-baeume-hintergrund.webp)).

## Ziel

Hintergründe mit Tiefe, die nicht mit Spielobjekten verwechselt werden.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Alle Biome neu, mit Biom-Tönen aus B-712.
- Keine Bäume im Hintergrund (auch nicht im Wald; dort Hügel, Unterholz, Licht).
- Varianten v1–v5 je Biom und Ebene; Punkte aus D-MG7-06.

## Nicht-Ziele

Parallax-Code im Spiel.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Kein Hintergrund-Asset enthält ein Baum-Teil (Test über die Specs).
- **AC-02** fern/mitte/nah unterscheiden sich im Ton (Sichtprüfung); Höhle ohne Kuppeln, Eisenstollen mit Rückwand und Glut.
- **AC-03** 75 Assets (5 Biome × 3 Ebenen × 5 Varianten).

## Offene Fragen

keine

## Notizen

Quelle: PS1.3 Punkt 7, PS1.4 Bäume, D-MG7-01, D-MG7-06. Nach B-712. Lane `hintergruende`.
