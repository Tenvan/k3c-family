# engine/store/

## Responsibility

Persistenz-Schicht (Repository über Dateien): Spielstände, Testberichte und Spielmetrik-Reports als JSON-Dateien. Schreibt atomar, rotiert Sicherungen und prüft Namen und Größen. Kennt weder HTTP noch Räume.

## Design

- Repository: `Saves` (`saves.go`) – je Slot eine Datei `<slot>.json` unter `Dir`, alle Zugriffe unter einer Sperre; Methoden `Load`, `Store`, `Delete`, `Purge`, `Count`; `List` (`saves_list.go`) liefert je Slot einen `SaveInfo` (Name, `savedAt`, Version, Tag, Phase, Tiefen der Spieler; unlesbare Datei nur mit `Error`), generisch gelesen ohne `engine/sim`.
- `Reports` (`reports.go`) – `Store` schreibt `gamepad-<zeit>.json` mit `receivedAt`/`remoteAddress`, `StoreSession` schreibt Spielmetrik-Reports als `session-<zeit>.json` (erfüllt `room.SessionStore`), `prune` hält höchstens `MaxReports` Dateien; Limits `MaxReportBytes`, `MaxSaveBytes`.
- Atomic Write: `writeAtomic` (`atomic.go`) – temporäre Datei im selben Ordner, dann Rename; ein abgebrochener Schreibvorgang lässt den alten Stand heil.
- Rotating Backups: `rotate`, `Backups`, `Restore` (`backups.go`), `BackupKeep` Sicherungen je Slot; Wechsel der `campaignId` sichert den bisherigen Stand.
- Header-Validierung: `parseSave`/`header` verlangt ein JSON-Objekt mit `campaignId` und `version`; `path` validiert Slotnamen (`ErrSlot`).
- Sentinel-Fehler `ErrInvalid`, `ErrNotFound`, `ErrTooLarge`, `ErrSlot` übersetzen Aufrufer in Statuscodes bzw. Protokoll-Codes.

## Flow

1. `Saves.Store(slot, data)` → `path` (Name prüfen) → `parseSave` → bisherigen Stand lesen, ggf. `rotate` in `backupDir(slot)` → `writeAtomic` → Rückgabe des Sicherungsnamens.
2. `Saves.Load(slot)` → Datei lesen, ohne Datei `ErrNotFound`.
3. `Saves.Restore(slot, name)` → gewählte Sicherung wird aktueller Stand, der bisherige wird selbst gesichert.
4. `Saves.Delete(slot)` entfernt Datei samt Sicherungen; `Purge(prefix, before)` löscht alte Testlauf-Spielstände (nur mit nichtleerem Präfix).
5. `Saves.List()` → alle `<slot>.json` (ohne Sicherungen) lesen, nach Name sortiert.
6. `Reports.Store(data, remote)` → JSON-Objekt prüfen, `receivedAt` ergänzen → `writeNew` → `prune(MaxReports)`; `StoreSession` prüft Größe und JSON-Objekt und schreibt ohne Überschreiben.

## Integration

- Abhängigkeiten: nur Standardbibliothek (Dateisystem, `encoding/json`, `time`).
- Konsumenten: `engine/room` (Interface `room.Store`, implementiert von `*Saves`; `room.SessionStore` von `*Reports`; nutzt `ErrNotFound`), `engine/net` (`Config.Saves`/`Config.Reports` für `/api/save*`, `/api/saves`, `/api/report`, `/api/status/save`), `cmd/k3c-server` (Verdrahtung, `Purge` beim Start).
- Dateiorte: `saves/` und `reports/` (Konfiguration über `cmd/k3c-server`).
