# GEB16.1 · Feld payable je Bauplatz im Snapshot und im Protokoll

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** geb16/1-payable-snapshot
- **Abhängig von:** –
- **Tickets:** B-587, B-588
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

- AC-01 (Server-Teil): umgesetzt.
  - Der Snapshot trägt `sites[].payable` und `nodes[].minable` (`engine/net/payable.go`), berechnet aus denselben Sim-Regeln wie `input.pay`: `sim.SitePayable` und `sim.NodeMarkable`; die Regel steckt jetzt in `nodeMarkable`.
  - Doku in `docs/protocol.md`, Beispiel `s2c-snapshot-wirtschaft.json`.
- Geprüft mit `go test ./engine/...` (neu `payable_test.go`, auch Wegfall im Delta) und golangci-lint, beide grün. Golden ist unverändert.
- Review (Protokoll): keine schweren Befunde. Das Feld hängt nicht vom Spieler ab, gilt also für 2 Spieler bzw. Split-Screen. Notiz: Die Rechenkosten pro Tick sind gering, eine Map hilft, falls der Lasttest sie zeigt.
- B-588: Der Server meldete das Abbaubare bisher nicht, deshalb gibt es das neue Feld `minable`.
