# BAL8.3 · Review BAL8

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bal8/3-review
- **Abhängig von:** BAL8.2
- **Tickets:** B-680, B-681
- **Kriterien:** AC-01, AC-02

## Ziel

Review BAL8: alle Kriterien, schließt den Sprint ab.

## Kontext

Prüfung von BAL8.1 und BAL8.2. Ralf bestätigt die Regeldatei (BAL8 AC-01).

## Erlaubte Dateien

- keine Codeänderung außer Review-Fixes unter etwa 30 Zeilen
- Planungsdateien über plan_*

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander (check:race lokal nur mit gcc).
2. Diff lesen, Kriterien AC-01 und AC-02 belegen, Determinismus und 2 Spieler prüfen.
3. Abnahme in höchstens fünf Zeilen in die Sprint-README.

## Fertig, wenn

- [ ] AC-01 und AC-02 belegt, Abnahme in der Sprint-README eingetragen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-10-31: `task check:go` grün (1 min 46 s), `task check:diff` grün (3 min 36 s), `task check` zuletzt grün (WZ35.4). `check:race` lokal ohne gcc nicht lauffähig, B-681/AC-03 in CI nachzuweisen. Unabhängiges Review (Sonnet) zu BAL8.2: 7 Befunde hoch (gemischte Quelle), behoben mit `TestWertesatzEineQuelleJeWert`; mittlere Befunde in B-692. AC-02 belegt. AC-01 (`docs/rules/welten.md` § 1): Regeldatei liegt vor, Bestätigung durch Ralf steht aus.
