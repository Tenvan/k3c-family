# GEB17.4 · Daten, Simulation und Protokoll setzen die Ausbauschritte um (hubStep)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** geb17/4-sim-protokoll-hubstep
- **Abhängig von:** GEB17.3
- **Tickets:** B-592
- **Kriterien:** AC-03

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

- AC-03: umgesetzt.
  - `data/hub.json` hat `levels[].steps`; die Summen entsprechen genau den alten Kosten und Bauzeiten, dazu kommt das Tor (20 Kupfer, Eisen bzw. Kristall).
  - `World.HubStep`: Schritt 3 hebt `HubLevel`. Der Zustand meldet `hubStep`, `hubUpgrade.steps` und den Wartegrund `waitingNextIsland`.
  - Der Spielstand speichert `hubStep` additiv. Protokoll und Beispiele sind nachgezogen.
- Geprüft mit `go test ./engine/... ./cmd/... ./tools/...` (neu `hub_step_test.go`, 2 Spieler) und golangci-lint. Golden ist unverändert (kein Golden-Lauf baut den Hub aus).
- Review (Opus): APPROVE, kein Deadlock. Kupfer kommt über den Tiefen-Eingang aus der Mine oder vom Händler.
- Risiko: Das Balancing ändert sich, weil Hub-Stufe 2 jetzt Kupfer braucht; die Messung kommt mit einem eigenen Sprint.
