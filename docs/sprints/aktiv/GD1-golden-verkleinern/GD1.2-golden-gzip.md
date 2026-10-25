# GD1.2 · Golden als gzip mit lesbaren Diffs, Abweichungs-Test

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** gd1/2-golden-gzip
- **Abhängig von:** GD1.1
- **Tickets:** B-419
- **Kriterien:** AC-01, AC-02

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

- AC-01 und AC-02: umgesetzt.
  - 15 Golden-Dateien (`sim-*`, `campaign-*`) liegen als `.json.gz`, entpackt byte-gleich mit vorher; `snapshotEvery` ist unverändert.
  - Größe: vorher 18 756 751 B, nachher 760 737 B.
  - `engine/internal/golden` liest und schreibt gz deterministisch (Zeitstempel 0, zweiter Lauf ohne Diff).
  - Diffs bleiben lesbar über `.gitattributes` `diff=gzip` und `task go:golden-setup`; die Anleitung steht in `docs/arbeitsweise.md`.
  - Die Grenze liegt wieder bei 16 MB.
- Geprüft mit `task check` und `task check:go` (grün), Studio `go test` und Vitest. Abweichungs-Test `golden_gz_test.go`: ein geändertes `stock.wood` wird erkannt.
- `level-*.json` und `rng.json` bleiben JSON, weil der Studio-Test sie importiert.
