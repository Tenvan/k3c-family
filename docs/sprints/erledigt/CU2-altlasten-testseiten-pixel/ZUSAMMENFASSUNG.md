# CU2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Altlasten: Testseiten des Browser-Clients und Pixel-Reste entfernen
- **Projekt:** NAT

## Ziel

Das Repo enthält keine Testseiten des Browser-Clients und keine Verweise auf alte Pixel-Grafik mehr. Am Ende sichtbar: Spielmenü › Seiten ohne tote Kacheln, `dist/` nur mit `index.html`, `game.html`, `monitor.html`, `lizenzen.html`; `TestHTMLEigenstaendig` grün; die Suchen aus B-822 und B-824 ohne Treffer.

## Ergebnis

2026-11-12: Kriterien in CU2.1 (B-822) und CU2.2 (B-824) belegt; Review CU2.3 (k3c-reviewer), Befunde behoben. Ralf-Queue: Einträge, die entfernte Seiten bzw. alte Werkzeuge planen (Liste in CU2.1 und CU2.2), mit B-823 klären. Kein Abnahme-Ticket: kein Sim/Balance (gesperrt), kein Gen-Feature.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Seiten, `src/stress/`, `data/stress.json`, `public/audio-test/` und ihre Module fehlen, Su … |
| AC-02 | – | Kacheln, Menü, Panel und `GameScene` ohne Verweise, Tests grün (`B-822/AC-02`). |
| AC-03 | – | CI-Smoke-Test angepasst, `task check` und `task build` grün, `dist/` nur mit bleibenden Se … |
| AC-04 | – | `CLAUDE.md`, Roadmap, SND und Codemaps ohne die Seiten (`B-822/AC-04`). |
| AC-05 | – | Liste betroffener Planung für die Ralf-Queue (`B-822/AC-05`). |
| AC-06 | – | Workbench-Bericht mit Papier-Pfaden, `TestHTMLEigenstaendig` grün (`B-824/AC-01`). |
| AC-07 | – | Render-Tests und Kommentare ohne alte 32-px-Grafik (`B-824/AC-02`). |
| AC-08 | – | `soundatlas` mit Quelle `data/sounds.json` (`B-824/AC-03`). |
| AC-09 | – | Doku und `CLAUDE.md` ohne Pixel-Verweise, Suche ohne Treffer (`B-824/AC-04`). |
| AC-10 | – | Liste alter Planung mit Pixel-Werkzeugen für die Ralf-Queue (`B-824/AC-05`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-822 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 – | Die Testseiten des Browser-Clients sind samt Code, Kacheln und Tests entfernt |
| B-824 | erledigt | AC-06 –, AC-07 –, AC-08 –, AC-09 –, AC-10 – | Nach dem Papierschnitt verweist kein Code, Test und keine Doku mehr auf alte Pix … |
