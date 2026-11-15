# engine/store/

## Responsibility

Persistenz-Schicht (Repository über Dateien): Spielstände, Testberichte und Spielmetrik-Reports als JSON-Dateien.
Schreibt atomar, rotiert Sicherungen und prüft Namen und Größen. Kennt weder Räume noch Netz.

## Design

- Repository `Saves` (`saves.go`): je Slot eine Datei `<slot>.json` unter `Dir`, alle Zugriffe unter einer Sperre;
  Methoden `Load`, `Store`, `Delete`, `Discard` (Stand wandert in seine Sicherungen, wiederherstellbar, B-506),
  `Purge` (alte Testlauf-Stände nach Präfix), `Count`; `List` (`saves_list.go`) liefert je Slot einen `SaveInfo`
  (Name, `savedAt`, Version, Tag, Phase, Tiefen; unlesbare Datei nur mit `Error`), generisch gelesen ohne `engine/sim`.
- Header-Validierung: `parseSave`/`header` verlangt ein JSON-Objekt mit `campaignId` und `version`; `path` prüft den
  Slotnamen (`ErrSlot`); Obergrenze `MaxSaveBytes`.
- Rotating Backups (`backups.go`): `rotate`, `Backups`, `Restore`, Typ `Backup`, `BackupKeep` Sicherungen je Slot;
  wechselt die `campaignId`, wird der bisherige Stand gesichert (`backupPrev`).
- Atomic Write (`atomic.go`): `writeAtomic` (temporäre Datei im selben Ordner, dann Rename) und exportiertes `Rename`
  (ersetzt das Ziel, wiederholt unter Windows ein kurz verweigertes Ersetzen); `readRetry` liest mit Wiederholung.
- `Reports` (`reports.go`): `Store` schreibt Testberichte mit `receivedAt`/`remoteAddress`, `StoreSession`
  Spielmetrik-Reports (erfüllt `room.SessionStore`), `prune` hält höchstens `MaxReports`; Limit `MaxReportBytes`.
- Gemerkte Räume (`restart.go`, B-508): `RememberRooms`/`TakeRememberedRooms` schreiben bzw. lesen und löschen eine
  `.state`-Datei im Spielstand-Ordner (kein Spielstand), höchstens `RememberedRoomsMaxAge` alt.
- Sentinel-Fehler `ErrInvalid`, `ErrNotFound`, `ErrTooLarge`, `ErrSlot`.

## Flow

1. `Saves.Store(slot, data)` → `path` → `parseSave` → bisherigen Stand lesen, ggf. `rotate` → `writeAtomic` →
   Rückgabe des Sicherungsnamens.
2. `Saves.Load(slot)` → Datei lesen, ohne Datei `ErrNotFound`.
3. `Saves.Restore(slot, name)` → Sicherung wird aktueller Stand, der bisherige wird gesichert.
4. `Saves.Discard(slot)` verschiebt in die Sicherungen; `Delete` entfernt Datei samt Sicherungen.
5. `Saves.List()` → alle `<slot>.json` ohne Sicherungen, nach Name sortiert.
6. `Reports.StoreSession(data)` prüft Größe und JSON-Objekt und schreibt ohne Überschreiben (`writeNew`).

## Integration

- Abhängigkeiten: nur Standardbibliothek.
- Konsumenten: `engine/room` (`room.Store` über `*Saves`, `room.SessionStore` über `*Reports`, `ErrNotFound`),
  `native/k3c-game/local` (`session.go`, `saves.go`: `watchedStore` über `store.Saves`, `Discard`, `List`),
  `tools/k3c-workbench/internal/studioview` (`room.NewManager(&store.Saves{…})`),
  `tools/k3c-workbench/internal/world` (`store.Rename` für Welt-Dateien).
- Dateiorte: `saves/` und `reports/`.
