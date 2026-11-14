# KB4.1 · Dev-Aktionen boss, bossPhase, event und night im Server

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb4/1-dev-aktionen-server
- **Abhängig von:** –
- **Tickets:** B-603
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
  - Die Dev-Aktionen `boss`, `bossPhase`, `event` (fullMoon, bloodMoon, merchantRaid) und `night` laufen nur im Dev-Mode, sonst `forbidden`; falsche Argumente ergeben `bad_request`.
  - Protokoll additiv, Beispiele `c2s-dev-{boss,bossphase,event,night}.json`.
- Geprüft mit `go test ./engine/... ./cmd/...` (Golden unverändert) und golangci-lint, beide grün.
- Review (Opus): Das normale Spiel bleibt unverändert, inklusive rng-Reihenfolge von `merchantArrive`. Fix: Die Dev-Nacht springt nicht auf die nächste Insel über. Die Doku nennt die Folgen im Spielstand.
- Abweichung: Den Boss „Eisenherr“ aus dem Beispiel gibt es nicht, das Beispiel nutzt `goblinLeader`.
