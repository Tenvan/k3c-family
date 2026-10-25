# GD1.1 · Wege messen und wählen (vorentschieden: gzip mit lesbaren Diffs)

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** gd1/1-wege-messen
- **Abhängig von:** –
- **Tickets:** B-419
- **Kriterien:** AC-01

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

- AC-01 (Wahl): vorentschieden (autonom, Claude, 2026-10-25) laut Ralfs Ziel. Ralf bestätigt über B-624 (KRT).
- Gewählt ist gzip für die großen Golden-Dateien mit textconv-Diffs. Die Prüfschärfe bleibt voll, die Grenze fällt zurück auf 16 MB. Messwerte vorher und nachher kommen mit GD1.2.
