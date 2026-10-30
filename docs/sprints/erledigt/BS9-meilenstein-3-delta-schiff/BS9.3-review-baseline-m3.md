# BS9.3 · Review BS9 und Baseline M3 sichern

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** bs9/3-review-baseline-m3
- **Abhängig von:** BS9.2
- **Tickets:** B-637, B-640
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `pfad/…`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Konkrete, nummerierte Schritte in sinnvoller Reihenfolge.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

**Fertig mit Abweichung (2026-10-29, Claude).** Baseline `M3` gesichert (`ohnePass`, Stand mit Schiff-Bot), `frozen` bleibt `[basis]` (holz und schiff sind nicht grün). Review: ~25 Zeilen Bot plus ein Export in `engine/sim/ship.go`, `task check:diff` grün, kein Reviewer-Agent. Befund: Der Bericht hat keinen Bau-Tag des Schiffs, nur `affordableDay`.
