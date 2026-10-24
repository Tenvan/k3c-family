# GEB16.3 · Gerüst-Grafiken je Größenklasse sowie Axt und Spitzhacke

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb16/3-geruest-werkzeug-grafik
- **Abhängig von:** –
- **Tickets:** B-586, B-588
- **Kriterien:** AC-03, AC-04

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

- AC-03 und AC-04 (Grafik): umgesetzt.
  - Gerüste `geruest-1/2/3/4/6` je Breite der Gebäude, mit Stangen, Riegeln, Brettern, Streben und halbfertiger Wand, ab Breite 4 mit Leiter.
  - Symbole `symbol-axt` und `symbol-spitzhacke`.
  - Slots `site:scaffold:<n>`, `sign:axe`, `sign:pickaxe`; Zuordnung und Regeldatei nachgezogen.
- Geprüft mit `go test` und golangci-lint in `tools/buildings`, Asset-Tests und Studio `src/graphics`. Claude hat die PNGs selbst angesehen.
- Abweichung: fünf Größenklassen (1, 2, 3, 4, 6) statt drei, passend zu den Gebäudebreiten.
