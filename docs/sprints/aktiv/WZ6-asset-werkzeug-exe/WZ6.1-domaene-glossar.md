# WZ6.1 · Domänentabelle und Glossar für das Asset-Studio

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** wz6/1-domaene-glossar
- **Abhängig von:** –
- **Tickets:** B-459
- **Kriterien:** AC-05

## Ziel

`tools/k3c-studio/` und `tools/assetkit/` stehen als DEV in der Domänentabelle; das Glossar kennt Asset-Studio, Welt-Betrachter, Chunk-Vorschau, Live-Vorschau und Gebäude-Planer (AC-05).

## Kontext

Entscheidungen 🧑 2026-10-17 (B-459 › Offene Fragen): Wails-App `tools/k3c-studio`, Name Asset-Studio, EXE `bin/k3c-studio.exe`, Domäne DEV. Die Domänentabelle steht in `docs/arbeitsweise.md` › Domänen; die Zeile DEV nennt heute `tools/k3c-dev/` (eigenes Go-Modul mit Oberfläche, MCP-Server), `cmd/k3c-load/`, `cmd/k3c-tui/` und die Werkzeug-Seiten `src/tools/`. WZ6.2 legt zusätzlich das gemeinsame Go-Modul `tools/assetkit` an (Asset-Logik aus `tools/k3c-dev/internal/assets`); es gehört ebenfalls zu DEV.

Das Glossar `docs/glossar.md` ist eine alphabetische Tabelle `Begriff | Bedeutung | Quelle`. Verwandte Einträge, die stehen bleiben: Asset-Katalog, Asset-Slot, Slot-Register (nennt „k3c-dev schreibt es, die Dev-Seite liest es“), Mini-Szene (nennt heute `assetvorschau.html`). Bisherige Namen derselben Sache wie Asset-Studio: Ressourcen-Manager (B-298), Asset-Manager (B-435), Assets-Seite der Workbench. Der Level-Betrachter (B-092, `src/tools/levelView.ts` auf `leveltest.html`) heißt künftig Welt-Betrachter (B-460).

Bedeutungen der neuen Begriffe (aus den Tickets):

- **Asset-Studio** (B-459): eigene Wails-App `tools/k3c-studio`, EXE `bin/k3c-studio.exe`, mit je einer Seite für Grafik und Sound; Heimat von Welt-Betrachter, Gebäude-Planer und den Seiten für Monarch und Gegner. Die MCP-Tools `assets_*` bleiben in der Workbench k3c-dev.
- **Welt-Betrachter** (B-460): Seite im Asset-Studio mit Chunk-Vorschau oben und Live-Vorschau darunter; früher Level-Betrachter.
- **Chunk-Vorschau** (B-460): Streifen eines generierten Levels aus Chunks mit Objekten (Seed, Biom), im Welt-Betrachter beim Öffnen maximiert.
- **Live-Vorschau** (B-460): Ansicht des Levels mit der Spielgrafik (`worldRenderer`), Hub-Stufe umschaltbar, Gebäude einzeln ein- und ausschaltbar; gleiche Eingaben ergeben das gleiche Bild.
- **Gebäude-Planer** (B-462): Seite im Asset-Studio, die je Hub-Stufe die freigeschalteten Gebäude mit Platz (`offsetUnits`), Tiefe (`fromDepth`) und Bedingungen zeigt und gültig nach `data/hub.json` zurückschreibt.

## Erlaubte Dateien

- `docs/arbeitsweise.md` (nur Domänentabelle, Zeile DEV)
- `docs/glossar.md`

## Nicht-Ziele

- Kein Code, keine Ordner unter `tools/` (das Gerüst macht WZ6.2).
- Bestehende Glossar-Einträge (Asset-Katalog, Asset-Slot, Slot-Register, Mini-Szene) nicht umschreiben; ihre Verweise auf k3c-dev und `assetvorschau.html` ziehen die Sessions nach, die den Code ändern (WZ6.5, WZ6.6).
- Tickets und erledigte Sprints nicht umbenennen: B-298 (Ressourcen-Manager), B-435 (Asset-Manager) und B-092 (Level-Betrachter) behalten ihre Namen als Historie.
- Keine weiteren Zeilen der Domänentabelle, keine Grenzfall-Liste, keine sonstigen Prozessänderungen in `docs/arbeitsweise.md`.
- Keine Begriffe für spätere Tickets ohne Entscheidung (Chunk-Planung B-461, Monarch-Seite B-463, Gegner-Seite B-464).

## Schritte

1. `docs/arbeitsweise.md` › Domänen, Zeile DEV: `tools/k3c-studio/` (Asset-Studio, eigenes Go-Modul mit Wails-Oberfläche) und `tools/assetkit/` (gemeinsame Asset-Logik von k3c-dev und Asset-Studio) ergänzen. Sonst nichts in der Datei ändern.
2. `docs/glossar.md`: Asset-Studio (mit „früher Ressourcen-Manager, Asset-Manager, Assets-Seite der Workbench“), Chunk-Vorschau, Gebäude-Planer, Live-Vorschau und Welt-Betrachter (mit „früher Level-Betrachter“) alphabetisch einsortieren, Bedeutung wie im Kontext, Quelle je Ticket (B-459, B-460, B-462).
3. Prüfen wie unten; Ergebnis mit Nachweis zu AC-05 ausfüllen.

## Fertig, wenn

- [x] AC-05: `docs/arbeitsweise.md` nennt `tools/k3c-studio/` und `tools/assetkit/` in der Zeile DEV; `docs/glossar.md` enthält Asset-Studio, Chunk-Vorschau, Gebäude-Planer, Live-Vorschau und Welt-Betrachter alphabetisch einsortiert, je mit Bedeutung und Quelle (B-459, B-460, B-462).
- [ ] `task check` grün (Planungs- und Projektregel-Tests eingeschlossen). Typecheck und Tests grün; Lint rot durch einen Fehler, der schon auf `develop` steht (B-465).

## Prüfen

```bash
task check
```

Läuft k3c-dev, stattdessen `check_run task:check`. Sichtprüfung: `git diff docs/arbeitsweise.md` zeigt nur die Zeile DEV, `git diff docs/glossar.md` nur die fünf neuen Zeilen an ihrer alphabetischen Stelle. Keine manuellen Prüfungen (Browser, Fenster, Xbox) in dieser Session.

## Ergebnis

- **AC-05 umgesetzt, geprüft:** Zeile DEV in `docs/arbeitsweise.md` nennt `tools/k3c-studio/` und `tools/assetkit/`; fünf neue Glossar-Zeilen (Asset-Studio, Chunk-Vorschau, Gebäude-Planer, Live-Vorschau, Welt-Betrachter) alphabetisch eingefügt, Quellen B-459, B-460, B-462. Sichtprüfung per `git diff`: nur die Zeile DEV und die fünf Zeilen.
- **Prüfung:** `check_run task:typecheck` und `task:test` grün (inkl. `planning` und `projectRules`). `task:lint` rot: `max-lines-per-function` in `tools/k3c-dev/frontend/src/planning/SprintsBacklog.tsx` (61 Zeilen), steht schon auf `origin/develop` `1328fca7` (seit #329), außerhalb der erlaubten Dateien → Ticket B-465.
- **Geprüft von:** Claude (autonom), keine manuellen Prüfungen.
- **Abweichungen:** Sprint in dieser Session aktiviert (`Status: aktiv`, `Start-Commit: 1328fca7`); im Worktree fehlte `node_modules`, `task install` nachgeholt.
- **Neue Tickets:** B-465.
