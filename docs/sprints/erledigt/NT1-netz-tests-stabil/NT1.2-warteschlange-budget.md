# NT1.2 · Warteschlange, verworfene Ereignisse, Snapshot-Budget

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** nt1/2-warteschlange-budget
- **Abhängig von:** NT1.1
- **Tickets:** B-280, B-190, B-263
- **Kriterien:** AC-04, AC-05, AC-06

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

- AC-04 bis AC-06: umgesetzt.
  - B-280: Ein neuer Zustand ersetzt den wartenden Zustand derselben Stufe, auch über andere Nachrichten hinweg (`engine/net/queue.go`, neuer Test).
  - B-190: `eventsDropped` gilt je Zustand und zählt auch ersetzte Zustände. Der Client warnt mit 🤒 am Beginn einer Reihe; den Zustand dafür hält jetzt der Strom (Review-Fix). Protokoll und Beispiel `s2c-snapshot-delta-dropped.json` sind ergänzt.
  - B-263 gemessen (40 Plätze, 4 Spieler, 1800 Ticks): `snap` Ø 26,2 KB, `delta` Ø 2,5 KB, Ereignisse Ø 9 B, also im Budget Q08. Der Wert steht in `docs/protocol.md`.
- Geprüft mit `go test ./engine/... ./cmd/...`, golangci-lint und `task check`, alle grün. Review (Opus): APPROVE.
- Abweichung: Zwei Tests sind an das neue Verhalten angepasst (Stufe 0 bekommt `snap 10` statt `snap 1, delta 10`). Laut Review ist das korrekt.
