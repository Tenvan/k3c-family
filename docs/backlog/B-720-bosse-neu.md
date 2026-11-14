# B-720 · Bosse sind neu gestaltet, jeder mit eigener Silhouette

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** MG10
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

PS1.3: Alle Bosse nutzen die Krieger-Grundform. D-MG5-04: Lava-Golem und Splitter-Titan schwer zu unterscheiden, Wangenpunkt bei beiden Golems, Lava-Golem-Arme in idle hinter dem Rumpf, heller dünner Schwanz der Ratten-Königin, Holzschwert des Troll-Königs kontrastarm.

## Ziel

Jeder Boss hat eine eigene, bedrohliche Silhouette.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Alle Bosse neu mit Rig-Proportionen je Figur (B-715) und den neuen Materialien (B-712); Punkte aus D-MG5-04 erledigt.

## Nicht-Ziele

Neue Bosse oder Kampfwerte.

## Regeln und Einschränkungen

Assets nur über MCP `assets_*` (Entscheidung 005); Golden nur bei gewollter Änderung neu.

- **Bereich:** `public/grafik/bosse/**`, `tools/paper/golden/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Silhouetten aller Bosse als schwarze Flächen nebeneinander unterscheidbar (Sichtprüfung).
- **AC-02** Jeder Punkt aus D-MG5-04 erledigt.
- **AC-03** Golden der Gruppe `bosse` neu, Tests grün.

## Offene Fragen

keine

## Notizen

Quelle: PS1.3 Punkt 5, D-MG5-04. Nach B-712, B-715. Lane `bosse`.
