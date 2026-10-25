# ST1.1 · Speichern alle 60 s und bei Tagesanbruch, Anzeige gesichert

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** st1/1-speichern-60s
- **Abhängig von:** –
- **Tickets:** B-186
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

- AC-01: umgesetzt (Anzeige am TV offen).
  - Autosave alle 1800 Raum-Ticks, also 60 s Echtzeit auch im Zeitraffer, und bei Tagesanbruch (`Cycle.Day` der Stufe 0).
  - Nach gelungenem Schreiben geht das Ereignis `saved` an jede Stufe und wird danach aus der Welt entfernt (Golden unverändert, `TestTaktAendertSpielverlaufNicht`). Das HUD zeigt 2 s „· gesichert“.
  - Protokoll additiv, Beispiel `s2c-snapshot-gesichert.json`.
- Geprüft mit `go test ./engine/... ./cmd/...`, golangci-lint, `tsc` und Vitest, alle grün.
- Review-Fix: Nach Game Over meldet der Raum kein „gesichert“ mehr (B-344).
- Offen (LOW): Ein „gesichert“ nach Verlassen oder Trennen erscheint erst beim nächsten Tick. Tagesanbruch zählt nur in Stufe 0, der 60-s-Takt sichert trotzdem.
- Abweichung: `SaveEvery` ist eine Konstante in `room.go` (Serverbetrieb, kein Balancing).
