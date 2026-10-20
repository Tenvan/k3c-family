# TS1.2 · Studio bindet die Engine ein, Vorschau und Spielstart ohne Server

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** ts1/2-studio-engine
- **Abhängig von:** TS1.1
- **Tickets:** B-507
- **Kriterien:** AC-02

## Ziel

Das Studio startet Vorschau und „Im Spiel öffnen“ mit eingebundener Engine und eigenen Testspielständen, ohne Spielserver und ohne Dev-Mode (AC-02).

## Kontext

Spec `B-507 › Anforderungen`. Heute nutzt das Studio für Vorschau und „Im Spiel öffnen“ (WZ18.6) den Spielserver im Dev-Mode (`app_game.go`, `app_preview.go`, `app_world*.go`). Neu: Das Studio bindet `engine/sim`, `engine/level`, `engine/room` (soweit nötig) als Bibliothek ein und hält einen eigenen Raum samt Testspielständen in einem eigenen Ordner (z. B. `tools/k3c-studio/.studio-saves/`, von git ignoriert), nie in `saves/` des Servers. Richtung: Studio → Engine, nie umgekehrt (Import-Test aus TS1.1 bleibt grün). Falls die Engine dafür Zugriffe braucht, die heute nur über `cmd/k3c-server` bestehen: Ticket anlegen und im Ergebnis nennen.

## Erlaubte Dateien

- `tools/k3c-studio/**` (Backend und Frontend-Aufrufe), `.gitignore` (eine Zeile für die Testspielstände)
- `tools/assetkit/**` nur wenn nötig

## Nicht-Ziele

Änderungen an `engine/` (Ticket statt Änderung), neue Studio-Funktionen, Server-Streichung (TS1.3), Watch (TS1.4).

## Schritte

1. Test zuerst (rot): Studio-Backend startet einen Raum mit Welt `standard` und einer Test-Welt ohne laufenden Server, Spielstand landet im Testordner, nicht in `saves/`.
2. Engine einbinden, die Aufrufe an den Spielserver (HTTP/WS, Dev-Mode) ersetzen.
3. Testspielstände anlegen/laden/löschen im Studio-Backend (nur Methoden, keine neue Oberfläche).
4. Prüfen, Commit `feat(dev): Studio mit eigener Engine und Testspielständen (TS1.2)`.

## Fertig, wenn

- [ ] AC-02: Go-Test: Vorschau und Spielstart ohne Spielserver; Spielstände nur im Testordner.
- [ ] `task check:go` und `task check:studio` grün.

## Prüfen

```bash
task check:go
task check:studio
```

## Ergebnis

- **AC-02 umgesetzt, geprüft:** `tools/k3c-studio/backend/engine.go` bindet die Engine ein (`NewEngine`: eigener `room.Manager` im Dev-Mode, Welten aus `data/worlds`, `store.Saves` unter `tools/k3c-studio/.studio-saves/`, Spiel-API über `engine/net.NewHandler`). `server.go` hängt `/api/…` und `/ws` der Engine neben `/api/studio/` ein, geschützt durch `gameGuard` (nur Loopback; Schreibanfragen nur bei gleicher Origin und `Sec-Fetch-Site` same-origin/none). „Im Spiel öffnen“ (`WorldOpenGame`) legt „voll ausgebaut“ über `Manager.FullSave` an und fragt keinen Spielserver mehr. Beim Schließen des Fensters beendet `stopServer` den Kontext der Engine und wartet bis 5 s (`EngineWait`), bis die Räume gespeichert sind. Go-Tests: `engine_test.go` (Raum in Welt `standard` und Test-Welt `probe` ohne Server, Spielstand nur im Testordner, kein `saves/`; Spiel-API; Spielstart ohne Server), `app_testsaves_test.go`, `server_test.go` (Spiel-API am Studio-Server, `TestGameGuard` fremde/gleiche Herkunft, `TestEngineWaitNachEnde`). Die Vorschau (`WorldPreview`) rief schon vorher `net.Preview` direkt auf.
- **Testspielstände:** `WorldTestSaves`, `WorldTestSaveCreate(seed)` (voll ausgebaut), `WorldTestSaveOpen(base, name)` (Name geprüft, „nicht gefunden“ getrennt von anderen Fehlern), `WorldTestSaveDelete(name)` (abgelehnt bei offenem Raum; Race zwischen Prüfung und Löschen bewusst hingenommen, Kommentar); nur Methoden, keine Oberfläche.
- **Prüfungen:** `task check:go` grün (check_run), `task check:studio` grün (Vitest, go test, golangci-lint).
- **Abweichungen:** Spiel-Adresse `<Adresse>/index.html#game.html?…` statt `/#…`, weil die Wurzel des Studio-Servers zum Studio führt. Vite-Proxy des Studio-Frontends zeigt auf 8091 (Server des Studios) statt 8080. `.gitignore`: eine Zeile plus Kommentar. Abhängige Tests (`rpc_test.go` Methodenmenge, `chat_history_test.go` Signatur `New(ctx, root, log)`) nachgezogen. Nachbesserung nach Review: `gameGuard`, `Engine.Wait`, Fehlerunterscheidung in `WorldTestSaveOpen`.
- Keine Änderung an `engine/`, kein neues Ticket nötig. Manuell geprüft: niemand.
