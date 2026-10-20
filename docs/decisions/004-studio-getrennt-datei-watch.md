# 004 · Asset-Studio getrennt vom Spielserver, der Server beobachtet die Dateien

Stand: 2026-10-20 · Status: **beschlossen und umgesetzt (TS1)** · Backlog: B-507 (löst die Server-Seite des Studios aus B-459 ab; betrifft B-488, WZ19)

## Kontext

- Bis TS1 lag das Studio-Backend in `engine/studio`. Der Spielserver band es ein (`cmd/k3c-server/studio.go`) und lieferte
  das Studio-Frontend unter `/studio/` aus (Build nach `dist/studio/`, Kachel „Welten bauen“ mit `studio.html`, Vite-Ausnahme
  für `/api/studio`, nur Dev-Mode und Loopback, `K3C_STUDIO_LAN`). Dasselbe Backend nutzte das Wails-Fenster `tools/k3c-studio`.
- Der Server lud `data/` nur per `go:embed` zur Build-Zeit; eine im Studio geänderte Welt kam erst nach Neubau und Neustart an.
- `deploy-pages` baute für die Präsentationsseite `site/` das ganze Spiel (Node, Vite), nur wegen ihrer Bilder.
- 🧑 hat am 2026-10-20 entschieden: Spieler ändern über den Server keine Assets mehr; der Server soll nur Server sein.

## Optionen

| Option | Bewertung |
|---|---|
| **A · Studio bleibt Seite des Servers** (Ist) | ein Prozess; Server hängt an Studio- und AI-Code, Assets änderbar über den Spielserver |
| **B · Studio eigenständig, Schnittstelle sind die Dateien, Server beobachtet sie** | saubere Trennung, Server ohne Studio baubar; der Server braucht einen Datei-Watch und einen Neustart-Weg |
| **C · Studio eigenständig, Server lädt nur nach manuellem Neustart** | einfach; im Studio Gebautes erscheint nicht von selbst im laufenden Spiel |

## Entscheidung

Option **B**:

1. **Trennung:** Das Studio-Backend liegt in `tools/k3c-studio/backend` (Modul `tools/k3c-studio`, samt Chat `chat_*`).
   `engine/` und `cmd/k3c-server` importieren nichts aus `tools/`; `engine/internal/rules/imports_test.go` erzwingt das.
   Die Richtung ist einseitig: Das Studio bindet die Engine als Bibliothek ein (`engine/sim`, `engine/level` …) und startet
   Vorschau und „Im Spiel öffnen“ ohne Spielserver mit eigenen Testspielständen.
2. **Keine Studio-Seite im Server:** `/studio/` und `/api/studio` antworten 404, Kachel „Welten bauen“, `studio.html`,
   Vite-Ausnahme, `dist/studio/` und `K3C_STUDIO_LAN` sind entfernt. Das Studio läuft nur als eigenes Fenster
   (`task k3c-studio`, Loopback-Port 8091), sein Frontend baut nach `tools/k3c-studio/frontend/dist/`.
3. **Schnittstelle sind die Dateien:** Das Studio schreibt `data/**` und `public/` (Grafiken, Sound), der Server liest sie
   und schreibt dort nie.
4. **Datei-Watch, immer aktiv (auch produktiv):** Der Server beobachtet `data/` (`K3C_DATA_DIR`) und `public/`
   (`K3C_PUBLIC_DIR`) im Takt von 1 s (`engine/watch`) und entprellt 2 s. Danach prüft ein Kindprozess
   `k3c-server -check-data` den neuen Stand mit demselben Ladecode wie beim Start. Gültig: Hinweis „Server startet neu“
   (Fehler-Code `restarting`, `docs/protocol.md` › Neustart des Servers) an alle Geräte, nach 3 s Ende mit Exit-Code 75.
   Ungültig: ❌ im Log, der alte Stand läuft weiter. Neuladen ohne Neustart gibt es nicht, auch keine Wege je Dateityp.
5. **Selbst-Neustart über einen Supervisor:** `k3c-server` startet sich selbst als Kindprozess (`K3C_CHILD=1`, `K3C_DATA_DIR`
   vorher gesetzt, Daten von der Platte) und startet es neu, wenn es mit 75 endet. Die PID des Supervisors bleibt, Starter,
   k3c-dev, Docker und `task serve` brauchen nichts Neues. Stürzt das Kind mit Daten von der Platte binnen 10 s ab, startet
   der Supervisor einmal mit dem eingebetteten Stand. Neustarts in laufenden Runden sind erlaubt; Spielstände bleiben gespeichert.
6. **GitHub Pages nur `site/`:** Der Workflow lädt `site/` ohne Node und ohne Build hoch; die Bilder liegen als Kopien in
   `site/img/`, `tests/pagesSite.test.ts` prüft jede Referenz. `task pages` kopiert nur noch `site/` nach `_site/`.

## Folgen

- Löst die Server-Seite des Studios aus B-459 ab (dort Studio „als Seite des Go-Servers“); der Chat aus B-488/WZ19 läuft im
  Fenster des Studios, seine Sicherheit (Loopback, keine Werkzeuge, kein Schlüssel) bleibt.
- Entscheidung 001 bleibt: die Simulation läuft weiter nur in Go; das Studio nutzt dieselbe Engine als Bibliothek.
- Geräte sehen nach einem Neustart die Raumliste, Räume öffnen nicht von selbst wieder (B-508). Grafiken und Sound aus
  `public/` erreichen die Clients erst nach einem Build, weil der Server `dist/` ausliefert (B-509). k3c-dev startet bei
  Änderungen in `data/` zusätzlich selbst neu (B-510); atomares Schreiben des Studios (B-511); übernommener Server nach
  Selbst-Neustart (B-512).
- Werte kommen beim Start von der Platte, wenn `data/` existiert; fehlt der Ordner (z. B. im Docker-Image ohne Volume),
  gilt der eingebettete Stand.
