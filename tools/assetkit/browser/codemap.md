# tools/assetkit/browser/

## Responsibility
Paket `browser` (B-614): öffnet eine Adresse im Standard-Browser des Systems, gedacht für den Web-Start (`-open`) der Wails-Programme.

## Design
- `browser.go`: einzige Funktion `Open(url)`.
  - Nimmt nur `http://` und `https://` an; alles andere ist ein Fehler, damit nie eine Datei oder ein fremdes Protokoll gestartet wird.
  - Befehl je Betriebssystem: Windows `rundll32 url.dll,FileProtocolHandler`, macOS `open`, sonst `xdg-open`.
  - Startet den Prozess und wartet nicht: `cmd.Wait` läuft in einer Goroutine, damit kein Zombie-Prozess bleibt.
- Nur Standardbibliothek (`os/exec`, `runtime`).

## Flow
Aufrufer → `Open(url)` → Präfix prüfen → `exec.Command(...).Start()` → sofort zurück.

## Integration
- Teil des Go-Moduls `k3c/tools/assetkit` (`tools/assetkit/go.mod`), Import `k3c/tools/assetkit/browser`.
- Im aktuellen Checkout importiert kein Paket außerhalb von `.claude/worktrees/` dieses Paket.
- Kein Test im Ordner.
