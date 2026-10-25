# BAL6.3 · Profil Mauern zuerst schärfen, Sensitivität prüft Wirkung

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bal6/3-profil-sensitivitaet
- **Abhängig von:** BAL6.1
- **Tickets:** B-300, B-301
- **Kriterien:** AC-02, AC-03

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

- AC-02: umgesetzt. `walls` wartet nur auf Mauern einer offenen Linie und zahlt dann Türme und Hub-Ausbau; damit spielt es messbar anders als `saver` (Test in `profiles_test.go`). Vorher erkannte es nie offene Linien.
- AC-03: umgesetzt. `vary` und `variant` lehnen Dateien ab, die die Sim nicht neu lädt (Test in `sensitivity_test.go`).
- Geprüft mit `go test` und golangci-lint im Workbench-Modul.
- Hinweis: `coop2` und `coop4` erben das neue Verhalten von `walls`.
