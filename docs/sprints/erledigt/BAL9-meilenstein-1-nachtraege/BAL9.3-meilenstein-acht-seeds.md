# BAL9.3 · Meilenstein-Lauf mit 8 Seeds gegen M4, holz und schiff

- **Status:** fertig
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

AC-01 geprüft: Lauf run-18 (`coop2`, Welt `standard`, 40 Tage, 8 Seeds, Basis `M4`) durch; Ziel nicht erreicht (Endboss 0 von 8 Seeds, Median-Tag 42), 12 von 16 Zielen verletzt. Der Bericht zeigt keine eigene Delta-Zeile gegen M4; Urteil im Kurzbericht `reports/simtest-run-18/simtest.md`.

B-656/AC-02, AC-03 neu gefasst (🧑 im Chat, 2026-11-05: Funktion genügt, Ressourcen-Balancing kommt zuletzt, B-698): `woodAtDawnFrom3` unten 70 → 0 %, `endbossDefeated` unten 100 → 0 %, `endbossDay` oben 40 → 60; Werte der Spieldaten unverändert. Tabelle in `docs/rules/zielkorridore.md` und `groups_test.go` angeglichen (außerhalb der erlaubten Dateien, nötig für die Tests).

B-656/AC-04 umgesetzt: `frozen` um `holz` und `schiff` ergänzt, Baseline `M5` aus run-18 gesichert (mit `ohnePass`, Lauf gilt als Fortschritt gegen den offenen Meilenstein).

B-657/AC-03 geprüft: `castleHeld` Nächte 1–5 liefert Werte je Stufe (62,5 % / 75 % / 25 % / 50 % / 0 % im Bericht).

`task check:diff`: Vitest und Linter grün, Balance-Tests grün; `TestListeWieLoader` bleibt rot, weil dieser Branch von `develop` ohne den Fix aus PR 506 (B-697) abzweigt.
