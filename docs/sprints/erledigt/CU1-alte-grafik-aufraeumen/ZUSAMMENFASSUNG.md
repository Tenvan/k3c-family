# CU1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Cleanup zum Schluss: alte Pixel-Grafik, Register und Gebäude-Renderer entfernen
- **Projekt:** PAP

## Ziel

Im Repo gibt es keine alte Pixel-Grafik und keinen Code mehr, der sie erzeugt oder lädt. Am Ende sichtbar: `public/grafik-db32/`, `public/atlas/`, `tools/buildings/` fehlen; `task check:all` grün.

## Ergebnis

2026-11-13, CU1.6 (Claude, autonom): AC-01, AC-03–AC-05 siehe CU1.1–CU1.6 (Rest `catalog.go` → B-760, Browser-Client-Reste durch 009 überholt); AC-02 verschoben → B-836; `task check`, `task check:go` grün. Behoben: Zeile `atlas/` in `tools/codemap.md`; Review ohne schwere Befunde. Neue Tickets: B-835 (eingecheckte `k3c-server.exe`), Abnahme-Ticket B-836 (Gen, Race, `check:all`, `buildall`). Offen für Ralf: Sichtprüfung Gerüste, Plantage und Sound im nativen Spiel. Version: keine vorgeschlagen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | B-700/AC-01: kein alter Pack, kein Atlas, kein `public/sprites/`, kein `public/grafik-db32 … |
| AC-02 | verschoben | `task check:all` und `task buildall` grün. |
| AC-03 | geprüft | B-700/AC-03: `CREDITS.md` und `LICENSE-ASSETS` nennen keine Grafik-Packs mehr; Sound-Credi … |
| AC-04 | geprüft | D-US2-02: Gerüste (`geruest-<klasse>` je Größenklasse, `hub-geruest`) und Plantage (`setzl … |
| AC-05 | geprüft | Spiel und Werkzeugseiten laden nur Papier-Assets; Maße und Reittier-Schlüssel kommen aus ` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-700 | erledigt | AC-01 geprüft, AC-03 geprüft | Alte Pixel-Grafiken, Register und Gebäude-Renderer werden nach der Papierschnitt … |
| B-701 | erledigt | – | PAP: kritische Fragen und vorab getroffene Entscheidungen werden gesammelt von R … |
