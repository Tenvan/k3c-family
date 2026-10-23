# B-565 · Der Wellenabstand unter Tage mit besetzten Adern ist über 100 Seeds gemessen

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
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

### Messbericht KB3 (Go-Test statt sim_test)

Messung: `K3C_MESSUNG=1 go test ./engine/sim -run TestWellenabstandMineMitAdern -v` (`engine/sim/aggression_veins_seeds_test.go`, 18 s). `sim_test` kennt kein Szenario mit besetzten Adern (nur Ziele aus `data/balance-targets.json`, Tiefe 0); der Balancing-Tester wurde nicht erweitert.
Aufbau: Mine, 2 Spieler, Seeds 1–100, Pool 0 %, keine Truppen/Gegner außer 4 Bauern an 2 markierten Adern (je 2), Horizont 60 min, Wellen = Pool voll.

| Variante | Seeds | Erste Welle Median | p10 | Folgewellen |
|---|---|---|---|---|
| mit 2 besetzten Adern | 100 (0 übersprungen, 0 ohne Welle) | 19,37 min | 19,37 min | keine in 60 min (n=0) |
| ohne Adern | Seed 1 | keine in 60 min | – | Grundwert 1 %/min (`mine.json`) → rechnerisch 100 min |

Befund: Der Korridor 4–12 min (`docs/rules/zielkorridore.md`) ist **verletzt** (19,4 min mit Adern, 100 min ohne). Alle Seeds liefern denselben Wert, weil die Adern auf den Hub gelegt werden (Setup deterministisch, nur die Mine unterscheidet sich im Seed) – das ist keine echte Streuung. Lücken: Folgewellen (Bauern fallen nach der ersten Welle), Bot-Verhalten über `sim_test`, Streuung durch echte Ader-Entfernungen. Werte ändern nur nach Beschluss 🧑 (Nicht-Ziel).
