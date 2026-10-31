# GEB17.6 · Eigene Grafik je Schritt der Wachstumsreihe

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb17/6-grafik-wachstumsreihe
- **Abhängig von:** GEB17.3
- **Tickets:** B-590
- **Kriterien:** AC-05

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

- AC-05: umgesetzt.
  - 15 unterscheidbare Ansichten: Hub-Stufe 1 zeigt Lagerfeuer, Zelt und Hütte; Hub-Stufe 2–5 zeigt in Schritt 1 die Burg der Stufe, in Schritt 2 dazu Banner und Fackeln, in Schritt 3 zusätzlich Wehrtürmchen im Material der Stufe.
  - Der Generator kennt das Stufen-Feld `basis` (Test `TestStufeBasis`).
  - `hubGrowth.ts` wählt nur nach `hubLevel` und `hubStep`; damit ist auch der Neustart der Reihe je Schritt aus GEB17.4 behoben.
- Geprüft mit `task check` (grün), Vitest (15 Kombinationen, 15 Texturen), Studio-Vitest, `go test ./tools/buildings/...` und einer Sichtprüfung der PNGs durch Claude.
- Abweichungen: Die Ansichten für Hub-Stufe 5, Schritt 2 und 3 sind im Spiel nicht erreichbar. `burg-1` bleibt nur als Rückfall.
