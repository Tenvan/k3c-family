# BAL9.1 · castleHeld wertet nur Nacht 1 bis 5 im Fenster des Ziels

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bal9/1-castleheld-fenster
- **Abhängig von:** –
- **Tickets:** B-657
- **Kriterien:** AC-02

## Ziel

Die Kennzahl `castleHeld` wertet nur Nacht 1 bis 5 aus (bis zum Morgen von Tag 6), auch in einem Lauf über 40 Tage (B-657).

## Kontext

`tools/k3c-workbench/internal/balance/targets.go` (Map `measures`) maß `castleHeld` als `CastleFallTick == nil` über den ganzen Lauf. Die Tage eines Ziels (`days: 5`) taugen nicht als Fenster, weil `sim_test days=40` sie über den Scope (`scope.go`) für alle Ziele überschreibt. Den Morgen nach Nacht 5 liefert `Metrics.Dawns` (Eintrag `Day: 6`, auch der Morgen nach dem letzten Tag).

## Erlaubte Dateien

- `tools/k3c-workbench/internal/balance/targets.go`, `targets_test.go`
- `tools/k3c-workbench/internal/balance/insights_plain.go`, `insights_plain_test.go` (Klartext der Kennzahl)

## Nicht-Ziele

Keine Balancing-Werte, keine Baseline. Inselstatistik `castleFell` (Offene Frage in B-657) bleibt unverändert.

## Schritte

1. Fenster als Konstante `castleHeldNights = 5`, Kennzahl über `Dawns` (Tag 6) und `CastleFallTick`.
2. Lauf ohne Morgen von Tag 6 und ohne Burgfall: kein Wert (nicht auswertbar, nicht verletzt).
3. Tests für Fall nach Tag 5, Fall an Tag 3, kurzen Lauf.

## Fertig, wenn

- [x] AC-02 (B-657/AC-01): Burgfall nach Tag 5 → `castleHeld` = 1 (Test `TestCastleHeldFenster`).
- [x] AC-02 (B-657/AC-02): Burgfall an Tag 3 → 0 (Test `TestCastleHeldFenster`).
- [ ] AC-02 (B-657/AC-03): `sim_test` 40 Tage zeigt den Wert der Nächte 1 bis 5 (Lauf in BAL9.3).

## Prüfen

```bash
task check:dev
```
Gezielt: `go test ./internal/balance/ -run 'CastleHeld|Anteil|Plain|Summar|Grad|Progress'` in `tools/k3c-workbench`.

## Ergebnis

- **AC-02 geprüft (B-657/AC-01, AC-02):** `castleHeld` (`targets.go`) wertet bis zum Morgen von Tag 6 (`Dawns`, Fenster `castleHeldNights = 5`, unabhängig von `Target.Days`, das `sim_test days=40` überschreibt). Test `TestCastleHeldFenster`: Fall Tag 41 → 1, Fall Tag 3 → 0, gehalten → 1, Lauf endet mit Fall vor Tag 6 → 0, Lauf kürzer als 5 Tage ohne Fall → kein Wert (nicht verletzt, bleibt aus dem Anteil).
- Test-Helfer `metricsWith` hat jetzt den Morgen von Tag 6; Klartext der Kennzahl „hielt die Burg Nacht 1 bis 5“ statt „bis zum Ende“ (`insights_plain.go`).
- **B-657/AC-03 offen:** braucht `sim_test` (40 Tage), läuft in BAL9.3.
- Prüfungen: `go test ./internal/balance/...` grün, `golangci-lint` 0 Befunde. Inselstatistik `castleFell` (offene Frage B-657) unverändert.
