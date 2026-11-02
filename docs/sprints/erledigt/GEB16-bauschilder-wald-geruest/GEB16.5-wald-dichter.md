# GEB16.5 · Wald noch dichter, Golden und campaign-ausbau angepasst

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** geb16/5-wald-dichter
- **Abhängig von:** –
- **Tickets:** B-585
- **Kriterien:** AC-02

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

- AC-02: umgesetzt.
  - `forest.tree` von [14, 20] auf [22, 30], `stream.tree` von [3, 6] auf [5, 8]. `deco_test.go` prüft den neuen Wert.
  - Golden neu geschrieben mit `task go:golden`. Grund: Mehr Bäume verschieben Knoten und IDs (gewollt, B-585).
  - `campaign-ausbau.json`: drei Eingabewerte angepasst (Segment 5 ohne Zahlen, weil dort jetzt ein Fels steht; Segment 18 +10 und Segment 20 −10 Ticks). Zähler am Ende: nodesGone 4, nodesMarked 1, pickupsTaken 1, gebaut 1, Bogenschützen 2.
- Geprüft mit `go test ./engine/...`, golangci-lint und `task check`, alle grün.
- Abweichung: Die Golden-Daten wachsen auf 18,1 MB. Die Grenze steigt auf 20 MB; die Bestätigung ist als kritische Frage B-620 (Projekt KRT) offen, das Verkleinern bleibt GD1.
