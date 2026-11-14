# BI2.2 · Kristallfarbe im nativen Spiel

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/bi2
- **Abhängig von:** BI2.1
- **Tickets:** B-901
- **Kriterien:** AC-02

## Ziel

Kristall-Adern zeigen im nativen Spiel ihre Farbe (B-901).

## Kontext

`props/nature.go` › `variantLook` kennt nur Baumarten; `props/assets.go` › `objectAssets` nennt die Assets je Level.

## Erlaubte Dateien

- `native/k3c-game/props/nature.go`, `native/k3c-game/props/assets.go`
- `native/k3c-game/props/nature_test.go`, `native/k3c-game/props/assets_test.go`

## Nicht-Ziele

Leuchten (BI2.4).

## Schritte

1. Test zuerst: Variante → Bild, Asset-Liste.
2. `variantLook` um `crystalVein` erweitern, Assets ergänzen.
3. `check:diff`, Commit.

## Fertig, wenn

B-901/AC-01, AC-02 per Unit-Test erfüllt, `check:diff` grün.

## Prüfen

`check_run task:check:diff`.

## Ergebnis

AC-02 erfüllt (B-901/AC-01, AC-02): `variantLook` wählt für `crystalVein` je Farbe das Asset (`crystalKinds` in `props/nature.go`), unbekannte Farben fallen auf `ader-kristall` zurück; `objectAssets` nennt die neuen Assets. Tests `TestVariantLookKristallfarbe`, `TestJedeGewaehlteGrafikLiegtVorUndIstGemeldet` (mit Varianten).
