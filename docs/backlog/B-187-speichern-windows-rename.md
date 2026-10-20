# B-187 · Speichern übersteht unter Windows eine kurz gesperrte Zieldatei

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** ST1
- **Projekt:** LST
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`TestGleichzeitigesSpeichern` (`engine/store/backups_test.go`) schlug im Review F2.4 einmal fehl:
`rename …\.tmp-2501151175 …\autosave.json: Access is denied.` (Windows, `task check:go`); fünf Wiederholungen waren grün.
`writeAtomic` (`engine/store/atomic.go`) ersetzt die Zieldatei mit `os.Rename`; alle Zugriffe von `Saves` laufen unter
`s.mu`, ein Konflikt im eigenen Prozess scheidet aus. Vermutet (ungeprüft): Virenscanner oder Indexer halten die gerade
geschriebene Datei kurz offen, dann verweigert Windows das Ersetzen. Linux (Pi, Docker) ist nicht betroffen.

## Ziel

Ein Spielstand oder Bericht geht auf einem Windows-Server (`task serve` am PC) nicht verloren, weil eine fremde
Anwendung die Datei kurz geöffnet hält; der Test ist nicht mehr wackelig.

## Beteiligte und Zielgruppen

Familie beim Spielen mit Server am Windows-PC; Entwickler und Agenten (roter `task check:go`).

## Anforderungen

- Ein kurzzeitig verweigertes Ersetzen (`ERROR_ACCESS_DENIED`, `ERROR_SHARING_VIOLATION`) wird begrenzt wiederholt, danach meldet `Store` den Fehler wie bisher.
- Die Datei bleibt atomar ersetzt: Leser sehen den alten oder den neuen Stand, nie einen halben.

## Nicht-Ziele

Andere Speicherorte, Dateisperren zwischen Prozessen.

## Regeln und Einschränkungen

Domäne SRV (`engine/store/`). Keine neue Abhängigkeit (`golang.org/x/sys` nur mit Zustimmung von 🧑).

## Beispiele

Scanner hält `autosave.json` 50 ms offen → zweiter Versuch gelingt, Spielstand gespeichert.

## Ausnahme- und Fehlerfälle

Datei bleibt dauerhaft gesperrt → Fehler nach den Wiederholungen, temporäre Datei wird entfernt (wie heute).

## Akzeptanzkriterien

- **AC-01** Ein Go-Test simuliert ein zweimal verweigertes Ersetzen; `writeAtomic` gelingt beim dritten Versuch.
- **AC-02** `TestGleichzeitigesSpeichern` läuft unter Windows 50-mal hintereinander grün (`go test -count=50 -run TestGleichzeitigesSpeichern ./engine/store`).

## Offene Fragen

Ursache bestätigen (Defender-Ausnahme für `.work/tmp` testweise setzen, 🧑). Sonst keine.

## Notizen

Umgesetzt 2026-10-20 auf `sprint/rename-retry` (nicht in ST1 geplant): `store.Rename` in `engine/store/atomic.go` wiederholt unter Windows ein mit `ERROR_ACCESS_DENIED` (5) oder `ERROR_SHARING_VIOLATION` (32) verweigertes `os.Rename` bis zu 6-mal im Abstand von 20 ms; `writeAtomic` der Engine und das des Studios (`tools/k3c-studio/backend/plan_file.go`, B-514) nutzen es. Ohne neue Abhängigkeit (`syscall.Errno`). AC-01: `engine/store/rename_test.go` (gelingt im dritten Versuch, dauerhafter Fehler nach 6 Versuchen, andere Fehler sofort; Windows-Tests laufen unter Linux nicht). AC-02: `go test -count=50 -run TestGleichzeitigesSpeichern ./engine/store` grün. Ursache weiter unbestätigt (Defender-Ausnahme, 🧑); die Wiederholung wirkt unabhängig davon.
