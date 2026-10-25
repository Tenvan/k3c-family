# RG1.1 · Messung und Ursachen für Burg, Hub-Kosten, Adern-Takt, Nacht 5, Grad-Kurve

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** rg1/1-messung-ursachen
- **Abhängig von:** –
- **Tickets:** –
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

- AC-01 bis AC-04 (Messteil): gemessen (run-7).
  - Die Burg hält auf normal in 80 % der Läufe (Korridor 75–90 %), die Grad-Kurve 100/100/80/60/35 fällt sauber ab. B-230 und B-346 sind wohl überholt (vermutet: alter Bot).
  - Die Abstufung nach Grad entsteht vor allem in Nacht 1; der Miniboss in Nacht 5 kostet 10–20 Punkte.
  - Hub-Stufe 2 ist für den Bot in Tiefe 0 nicht erreichbar: Das Kupfer-Tor (GEB17) verlangt 20 Kupfer, der Wald liefert keins.
- Vorschläge für RG1.2: Hub-Stufe 5, Schritt 3 von 200 auf 100 Gold; `percentPerGather` der Adern 1 → 2 (rechnerisch ca. 10,6 statt 19,4 min, nicht gemessen).
- Bericht: `docs/balancing/messung-2026-10-25.md` › RG1.1.
