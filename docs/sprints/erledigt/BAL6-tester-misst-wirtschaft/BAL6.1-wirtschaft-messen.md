# BAL6.1 · Messgrößen der Wirtschaft, Bot-Profil wirtschaft, Ziele in balance-targets.json

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bal6/1-wirtschaft-messen
- **Abhängig von:** –
- **Tickets:** B-347, B-099
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

- AC-01: umgesetzt. Alle 13 Ziele der Tabelle „Korridore der Wirtschaft“ werden bewertet.
  - Neue Kennzahlen: Turm, Hub-Stufe 2 und 3, Gold, Vorrat und Kämpfer je Tagesbeginn.
  - Neues Bot-Profil `wirtschaft` (Standardszenario); Ziele mit Grad in `data/balance-targets.json`.
  - `sim.SiteGoldLeft` liest nur.
- Geprüft mit `go test` (Wurzel und Workbench), golangci-lint, `dev:test` und `task check:go`, alle grün. Golden ist unverändert.
- `sim_test` run-6 (3 Seeds): Burg hält Normal 100 %, Hart 67 %, Ultra 33 %, Gold am Morgen Median 46. Hub-Stufe 2 und 3 je 0 %.
- Befund für BR1.3: In Tiefe 0 reicht der Stein nicht für Hub-Stufe 2.
- Review: Die Laufzeit (geschätzt 35–40 min bei 100 Seeds) steht jetzt in Task-Beschreibung und README. Die Baseline schreibt B-625 neu.
