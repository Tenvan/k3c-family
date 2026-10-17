# B-463 · Eine Monarch-Seite im Asset-Werkzeug zeigt Reiten, Schlagen, Skills und weitere Vorschauen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ10
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Monarch nutzt das Sheet `medieval-king` (`data/sprites.json` › `players`) mit den Animationen `idle`, `run` und `attack`; er reitet (`mounts`, `riderWaist`) und hat Angriff, Skills und Presets (`data/monarch.json` › `attack`, `skills`, `presets`). Für Sprinten und Skills gibt es keine eigene Animation. `figuren.html` und `aufstellung.html` (`src/tools/spriteReference.ts`) zeigen Figuren statisch bzw. in Aufstellung; Animationen sieht man nur im Spiel.

## Ziel

Eine Monarch-Seite im Asset-Werkzeug (B-459) spielt alle Vorschauen des Monarchen ab: stehen, laufen, sprinten, reiten (je Reittier), schlagen, Skills und weitere Zustände; je Spielerfarbe und im Spielmaßstab, mit Auswahl der Grafik aus dem Slot-Register.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Neue Animationen oder Skills (CLI/REG); Werte von Angriff und Skills (REG); Gegner (B-464).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Monarch-Seite zeigt jede Animation des Monarchen aus `data/sprites.json` einzeln abspielbar (Schleife, Einzelbild, Tempo).
- **AC-02** Reiten zeigt den Monarchen auf jedem Reittier aus `mounts` mit `riderWaist`-Ausrichtung wie im Spiel.
- **AC-03** Schlagen und jeder Skill aus `data/monarch.json` zeigen ihre Animation bzw. ihren Effekt; fehlende Grafik wird als Lücke markiert.
- **AC-04** Spielerfarben sind umschaltbar; der Maßstab entspricht dem Spiel.

## Offene Fragen

1. „usw.“: Welche Zustände gehören noch dazu (Münzen werfen, getroffen, Respawn, Wiederbeleben)? Entscheidet 🧑.
2. Spielen die Vorschauen in Phaser mit denselben Animationsdefinitionen wie das Spiel (Annahme) oder als eigene Canvas-Abspielung?
3. Ersetzt die Seite `figuren.html`/`aufstellung.html` für den Monarchen?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
