# B-1004 · Die drei MatchBrowser-Tests in native/k3c-game/audio prüfen ohne die entfernten Browser-Quellen

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** verworfen
- **Sprint:** NA1
- **Projekt:** NAT
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`go test ./audio/` im Modul `native/k3c-game` scheitert (geprüft 2026-11-15) an drei Tests: `TestEventCuesMatchBrowser` und `TestCueSlotsMatchBrowser` (`native/k3c-game/audio/events_test.go`) und `TestMusicSlotsMatchBrowser` (`native/k3c-game/audio/music_test.go`). Sie lesen über `browserSource` die Dateien `src/audio/events.ts`, `ui.ts` und `music.ts` des Browser-Clients; der ist seit B-848 entfernt („open ..\..\..\src\audio\events.ts: The system cannot find the path specified“). Die CI fährt `go test -short ./...` im Job `k3c-game` (`.github/workflows/ci.yml`); der CI-Lauf auf `main` e0baf94f1 ist rot (Ursache vermutet: diese Tests, ungeprüft).

## Ziel

Die Audio-Tests sind grün und prüfen weiter, was nach dem Wegfall des Browsers noch zählt: Jeder Klang-Slot hat eine Datei.

## Beteiligte und Zielgruppen

Agenten und CI; 🧑 gibt frei.

## Anforderungen

- Die drei Tests lesen keine Datei unter `src/` mehr; `browserSource` entfällt.
- Sie prüfen stattdessen die nativen Tabellen (`eventCues`, `cueSlot`, `musicSlots`) gegen das echte Register (`realRegister`, `data/sounds.json`): jeder Slot hat eine Datei, kein Musikstück doppelt, Cues außer Bewegen und Bestätigen bleiben still.
- Testnamen ohne „Browser“; Kommentare ohne Verweis auf `events.ts`, `ui.ts`, `music.ts`.
- Kein Produktivcode geändert; Klänge für 2 Spieler unverändert.

## Nicht-Ziele

Neue Klänge oder Slots; übrige Kommentare im Spiel, die noch Browser-Dateien nennen (z. B. `lightView.ts` in `native/k3c-game/world/lightdraw.go`).

## Regeln und Einschränkungen

Nur Testdateien. Kein Browser-Client mehr (`tests/projectRules.test.ts`).

- **Bereich:** `native/k3c-game/audio/events_test.go`, `native/k3c-game/audio/music_test.go`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

`go test ./audio/` im Modul `native/k3c-game` → `ok`.

## Ausnahme- und Fehlerfälle

Ein Slot der nativen Tabelle fehlt im Register → der Test scheitert mit dem Slot-Namen (das ist der gewollte Schutz).

## Akzeptanzkriterien

- **AC-01** `go test ./audio/` in `native/k3c-game` ist grün; `grep -rn "MatchBrowser\|browserSource\|\"src\"" native/k3c-game/audio` ohne Treffer.
- **AC-02** Ein Test scheitert, wenn ein Slot aus `eventCues` oder `musicSlots` keine Datei im Register hat (Test mit entferntem Eintrag oder bestehende Prüfung in den drei Tests).

## Offene Fragen

Verworfen (B-1143, 2026-11-19, autonom, Claude): Die Go-Tests liegen in `native/k3c-game/audio`, das mit RU37 gelöscht wird.

## Notizen

Gefunden beim Planen 2026-11-15 (`go test ./audio/`). Vermutlich Ursache der roten CI auf `main`.
