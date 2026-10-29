# BS8.3 · Review BS8 und Baseline M2 sichern

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** bs8/3-review-baseline-m2
- **Abhängig von:** BS8.2
- **Tickets:** B-636, B-640
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

**Fertig mit Abweichung (2026-10-29, Claude).** Baseline `M2` gesichert (`ohnePass`, Stand mit rodenden Bots, run-1), im Editor unter Basis-Läufe zu sehen (Sichtprüfung 🧑 offen, wie AC-04 in BS7). `frozen` bleibt `[basis]`: `holz` ist nicht grün und wird nicht eingefroren (Abweichung von AC-04). Review: kleine Änderung (~10 Zeilen Bot-Code plus Test), `task check:diff` grün, kein Reviewer-Agent nötig. Neue Befunde: Bot-Roden verschiebt den Spielverlauf bei einem Seed stark (Ratten-Königin Tag 23 → 33); mehr Seeds nötig (BS6.4).
