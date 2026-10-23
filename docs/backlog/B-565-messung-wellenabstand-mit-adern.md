# B-565 · Der Wellenabstand unter Tage mit besetzten Adern ist über 100 Seeds gemessen

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KB3
- **Projekt:** KMP
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-200/AC-01 verlangt eine Messung über 100 Seeds (Mine, Normal, 2 Spieler, mit und ohne 2 besetzte Adern: Median und p10 des Wellenabstands). LV1.2 hat nur den Go-Test (`TestAggressionspoolMitAdernNichtVorVierMinuten`: 2 Adern, 4 Bauern, keine Kills → 20 % Druck aus Lieferungen in 4 min, mit Grundwert 24 %) umgesetzt; ein Szenario mit besetzten Adern kennt `sim_test` nicht.

## Ziel

Der Korridor 4–12 min (`docs/rules/zielkorridore.md`) ist mit besetzten Adern gemessen.

## Beteiligte und Zielgruppen

Agent (Balancing-Tester, `tools/` bzw. `sim_test`); 🧑 entscheidet bei Verletzung des Korridors über Werte.

## Anforderungen

- Szenario „Adern besetzt“ im Balancing-Tester; Lauf über `sim_test`, Tabelle in B-200 › Notizen.

## Nicht-Ziele

Werte ändern (nur bei Verletzung des Korridors, Beschluss 🧑).

## Regeln und Einschränkungen

Messung nur über `sim_test` (B-348); Werte bleiben in `data/`, deterministisch über `engine/rng`.

## Beispiele

100 Seeds Mine, Normal, 2 Spieler, 2 besetzte Adern → Median und p10 des Wellenabstands liegen in 4–12 min.

## Ausnahme- und Fehlerfälle

Mine ohne zwei Adern im Seed → Seed übersprungen und gezählt.

## Akzeptanzkriterien

- **AC-01** Tabelle Median/p10 mit und ohne Adern über 100 Seeds in B-200.

## Offene Fragen

keine

## Notizen

Aus LV1.2 (B-200/AC-01). Go-Test als Untergrenze: 20 % Druck aus Lieferungen in 4 min.
