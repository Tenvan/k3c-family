# NA1.1 · Audio-Tests ohne Browser-Quelle und Lint-Befund im Lizenz-Generator

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/na1
- **Abhängig von:** –
- **Tickets:** B-1004, B-1005
- **Kriterien:** AC-01, AC-02

## Ziel

`go test ./audio/` und `golangci-lint run ./menu/...` im Modul `native/k3c-game` sind grün (B-1004, B-1005).

## Kontext

Modul `native/k3c-game` (eigenes Go-Modul, Befehle dort ausführen).

- **B-1004:** `audio/events_test.go` › `TestEventCuesMatchBrowser`, `TestCueSlotsMatchBrowser` und `audio/music_test.go` › `TestMusicSlotsMatchBrowser` lesen über `browserSource` (`events_test.go` Zeile 14) Dateien aus `src/audio/` des entfernten Browser-Clients (B-848) und scheitern mit „cannot find the path“. Die nativen Tabellen `eventCues`, `cueSlot`, `musicSlots` (`audio/events.go`, `audio/music.go`) sind jetzt die Quelle; `realRegister(t)` liefert das echte Register aus `data/sounds.json`.
- **B-1005:** `menu/licensefile/main.go` Zeile 82: `e.IsDir() || !(strings.HasPrefix(up, "LICENSE") || …)` → staticcheck QF1001.

Die CI fährt `go test -short ./...` in diesem Modul; der Lauf auf `main` ist rot (vermutet wegen der Audio-Tests).

## Erlaubte Dateien

- `native/k3c-game/audio/events_test.go`
- `native/k3c-game/audio/music_test.go`
- `native/k3c-game/menu/licensefile/**`

## Nicht-Ziele

Produktivcode in `audio/`; neue Klänge; andere Lint-Befunde; Kommentare in anderen Paketen, die noch Browser-Dateien nennen.

## Schritte

1. `go test ./audio/` und `golangci-lint run ./menu/...` laufen lassen, Befunde bestätigen.
2. Die drei Tests umbauen: native Tabelle gegen Register (jeder Slot hat eine Datei, kein Musikstück doppelt, Cues außer Bewegen und Bestätigen still); `browserSource` und ungenutzte Importe entfernen; Tests ohne „Browser“ benennen.
3. Bedingung in `licensefile/main.go` umformen (Hilfsfunktion oder De Morgan), kein `//nolint`.
4. `go test ./audio/ ./menu/...`, `golangci-lint run ./audio/... ./menu/...`.
5. `task check:diff`.

## Fertig, wenn

- [ ] AC-01: `go test ./audio/` grün; `grep -rn "MatchBrowser\|browserSource" native/k3c-game/audio` ohne Treffer; ein Slot ohne Datei lässt einen Test scheitern (B-1004/AC-01, AC-02).
- [ ] AC-02: `golangci-lint run ./menu/...` 0 Befunde, `go test ./menu/...` grün (B-1005/AC-01, AC-02).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 geprüft: `go test ./audio/` grün, Tests prüfen native Tabellen gegen das Register (`TestEventCuesHaveFiles`, `TestMusicSlotsHaveFiles`, `TestMissingFilesNames*`). AC-02 geprüft: `golangci-lint run ./...` 0 Befunde, Generator-Ausgabe byte-gleich. `task check:diff` grün. Commit bed313a94 auf `sprint/na1`.
