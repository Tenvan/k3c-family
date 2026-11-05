# BAL9.3 · Meilenstein-Lauf mit 8 Seeds gegen M4, holz und schiff

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bal9/3-meilenstein-acht-seeds
- **Abhängig von:** BAL9.1, BAL9.2
- **Tickets:** B-656
- **Kriterien:** AC-01

## Ziel

Der Meilenstein-Lauf coop2 standard ist über 8 Seeds gegen `M4` beurteilt; `holz` und `schiff` sind grün oder begründet neu gefasst, `frozen` ist ergänzt, Baseline `M5` gesichert (B-656).

## Kontext

Lauf run-1 (B-656 › Notizen) scheiterte am Titan und an einem Messfehler von `castleHeld` (behoben in BAL9.1). BAL9.2 macht den Bot-Anteil schneller (byte-gleich). Entscheidung 🧑 (2026-10-29/31): 8 Seeds, `M4` als Basis, kein höheres Zeitlimit; bleibt `holz` oder `schiff` rot, wird das Ziel begründet neu gefasst, die Werte bleiben, 🧑 bestätigt im Bericht.

## Erlaubte Dateien

- `data/balance-targets.json` (Ziele, `frozen`)
- `reports/baseline/M5/` (Baseline `M5`, über `sim_test`/Baseline-Werkzeug)
- `docs/backlog/B-656-*.md` (über `plan_*`)

## Nicht-Ziele

Neue Mechaniken, Werteänderungen ohne Lauf, Spieleabend-Runde (BR1, BR2).

## Schritte

1. `sim_test start bots=coop2 world=standard days=40 seeds=8 baseline=M4` (nur über `sim_test`, blockierend warten).
2. `castleHeld` Nächte 1 bis 5 im Bericht prüfen (B-657/AC-03).
3. `holz`, `schiff` je Seed beurteilen, Bau-Tag des Schiffs je Seed nennen; rot → Ziel begründet neu fassen.
4. `frozen` um `holz` und `schiff` ergänzen, Baseline `M5` sichern.

## Fertig, wenn

- [ ] AC-01: Lauf mit 8 Seeds durch, Delta-Bericht je Werte-Gruppe (B-656/AC-01).
- [ ] AC-01: `holz` und `schiff` grün oder begründet neu gefasst (B-656/AC-02, AC-03).
- [ ] AC-01: `frozen` ergänzt, `M5` gesichert (B-656/AC-04).
- [ ] B-657/AC-03: `castleHeld` zeigt im 40-Tage-Lauf den Wert der Nächte 1 bis 5.

## Prüfen

```bash
task check:diff
```
Läufe nur über `sim_test`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
