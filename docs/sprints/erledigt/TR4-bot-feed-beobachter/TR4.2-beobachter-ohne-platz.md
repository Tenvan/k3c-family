# TR4.2 · Protokoll und Raum: Beobachter-Gerät ohne Platz für sim_test

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** live
- **Branch:** tr4/2-beobachter-ohne-platz
- **Abhängig von:** TR4.1
- **Tickets:** B-352
- **Kriterien:** AC-02

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
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-02: umgesetzt.
  - `join` und `create` mit `observe: true` hängen ein Gerät ohne Platz an (`engine/room/observer.go`). Es bekommt Level und Zustände, `input`, `addSlot`, `pause` und `dev` ergeben `bad_request`.
  - Beobachter zählen nicht als verbunden (Autosave, Pause). Ein Raum bleibt bestehen, solange beobachtet wird.
  - `sim_test` mit Clients erlaubt `players` bis 4 (Workbench `botdev`, `mcpsrv/simtest_*`).
  - Protokoll additiv (Version 9), Beispiele `c2s-join-observe.json` und `c2s-create-observe.json`.
- Geprüft mit `go test ./engine/... ./cmd/...`, Workbench `go test`, golangci-lint und `task check`, alle grün.
- Review-Fixes: Beobachter lösen keine Dev-Aktion aus (Test `TestDevBeobachterAbgelehnt`). Die Frist EmptyFor läuft ab dem Abgang des letzten Beobachters.
- Offen: der Bot-Feed im LAN (zweite Anforderung von B-352, offene Frage für Ralf).
