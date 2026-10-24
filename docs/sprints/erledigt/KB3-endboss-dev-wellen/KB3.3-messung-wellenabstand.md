# KB3.3 · Wellenabstand unter Tage mit Adern über 100 Seeds messen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** kb3/3-messung-wellenabstand
- **Abhängig von:** –
- **Tickets:** B-565
- **Kriterien:** AC-03

## Ziel

Der Wellenabstand unter Tage mit besetzten Adern ist über 100 Seeds gemessen.

## Kontext

B-565; `sim_test` kennt kein Ader-Szenario, deshalb Go-Test.

## Erlaubte Dateien

- `engine/sim/aggression_veins_seeds_test.go`, `docs/backlog/B-565-*.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Messtest. 2. Bericht in die Notizen.

## Fertig, wenn

- [x] AC-03.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-03 gemessen: erste Welle Median 19,37 min (p10 19,37), 0 übersprungene Seeds; Korridor 4–12 min verletzt, Folgeticket B-579. Grenzen: Adern liegen auf dem Hub, keine echte Streuung, keine Folgewellen. Messtest `K3C_MESSUNG=1`.
