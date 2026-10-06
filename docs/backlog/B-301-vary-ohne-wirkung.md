# B-301 · Ein Sensitivitäts-Pfad ohne Wirkung ergibt einen Fehler

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`sim.UseData` (`engine/sim/data.go`) lädt buildings, troops, economy, hub, monarch, enemies und waves neu, nicht aber `difficulty.json`, `biomes/*.json` und den Skill-Katalog. Ein `--vary` auf diese Dateien im Balancing-Tester (`tools/k3c-dev/internal/balance/vary.go`) läuft still ohne Wirkung.

## Ziel

Kein Sensitivitäts-Bericht, der eine Variation zeigt, die gar nicht gewirkt hat.

## Beteiligte und Zielgruppen

Entwickler und Agenten, die Sensitivitäts-Läufe auswerten.

## Anforderungen

- Ein Pfad in einer Datei, die `UseData` nicht neu lädt, ist ein Fehler mit Pfad, bevor der erste Lauf startet.

## Nicht-Ziele

Neu-Laden von difficulty, biomes oder Skills.

## Regeln und Einschränkungen

Keine Datei in `data/` wird geschrieben.

## Beispiele

`task balance:sensitivity -- --vary difficulty.json:grades.hard.waveSize` → Fehler „difficulty.json wird nicht neu geladen“.

## Ausnahme- und Fehlerfälle

nicht relevant: das Ticket ist selbst ein Fehlerfall.

## Akzeptanzkriterien

- **AC-01** Test: `--vary` auf `difficulty.json` liefert einen Fehler mit Pfad.

## Offene Fragen

keine

## Notizen

Hinweis aus dem Review BAL3.4 (2026-10-05).
