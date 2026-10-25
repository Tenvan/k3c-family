# BAL7.1 · Holzfluss messen über sim_test, Wertvorschlag

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bal7/1-holzfluss-messen
- **Abhängig von:** –
- **Tickets:** B-589
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

- AC-01: gemessen (`sim_test` run-7, 20 Seeds, Profil `wirtschaft`, Stand mit GEB16, W12, KB1, GEB17 und BAL6).
  - Holz liegt im Korridor: 85 % der Läufe starten ab Tag 3 mit mindestens 100 Holz (Ziel 70–100 %).
  - Der dichtere Wald wirkt vor allem an Tag 1–2 (ca. 360 Holz pro Tag), danach bremst die Bot-Regel `woodWant` 150.
- Vorschlag: keine Änderung, als Alternative `tree.amount` 10 → 8. Ralf entscheidet in BAL7.2.
- Bericht: `docs/balancing/messung-2026-10-25.md` › BAL7.1.
