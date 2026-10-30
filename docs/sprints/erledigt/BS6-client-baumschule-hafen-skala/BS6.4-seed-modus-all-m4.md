# BS6.4 · Seed-Modus all, Abschluss-Lauf, Baseline M4

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** bs6/4-seed-modus-all-m4
- **Abhängig von:** BS6.3
- **Tickets:** B-634, B-640
- **Kriterien:** AC-04

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

**Fertig mit Abweichung (2026-10-29, Claude).** `seedMode` in `data/balance-targets.json` steht wieder auf `all`. Der Lauf mit allen 101 Seeds (run-2) bricht am Limit von 1 h ab („Zeitlimit erreicht“, ~4 min je Seed unter Last); der Abschluss-Lauf lief mit 8 Seeds (run-3, 29 min, `baseline=M3`) und ist als `M4` gesichert (`ohnePass`). Delta: `holz` und `schiff` gleich; **`basis` kippt in 1 von 13 Zielen** (Gold am Morgen je Spieler, Median), aber die Baseline `M3` hat 1 Seed, der Lauf 8 – der Vergleich ist nicht gleichwertig (Seed-Streuung, kein Beleg für eine Regression). **AC-04 nur teilweise:** Lauf über alle Seeds scheitert am Zeitlimit (Ticket-Kandidat: Limit oder parallele Last), Vergleich mit gleicher Seed-Zahl fehlt. Empfehlung 🧑: M4 mit 8 Seeds als neue Vergleichsbasis nehmen.
