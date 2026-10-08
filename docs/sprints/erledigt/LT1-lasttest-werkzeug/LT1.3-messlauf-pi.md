# LT1.3 · Messlauf am Pi über eine Nacht

- **Status:** verworfen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** SRV
- **Umgebung:** live
- **Branch:** lt1/3-messlauf-pi
- **Abhängig von:** LT1.2
- **Tickets:** B-175, B-042
- **Kriterien:** AC-06

## Ziel

🧑 hat am Pi 2 Räume × 3 Spieler über eine Nacht gemessen; Tabelle und Bewertung stehen in dieser Datei und in B-042.

## Kontext

Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews LT1.4; bis zur Messung gilt das angenommene Ziel aus B-042 (Tick-p99 < 10 ms bei 30 Hz, 2 Räume × 3 Spieler; Handmessung 2026-10-03: Nacht 10,2 bis 10,3 ms). Dauer etwa 15 Minuten Arbeit plus eine Nacht Laufzeit. Der Pi-Server braucht `K3C_STATUS_TOKEN`. Ein Agent kann den Bericht danach auswerten und die Tabelle übertragen.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status), `docs/backlog/B-042-*.md` (Ergebnis, Status, Archiv), `docs/backlog/README.md`
- Sprint-README (nur Tabelle der Sessions), `reports/` (Bericht)

## Nicht-Ziele

Code-Änderungen; Optimierungen am Server. Ziel verfehlt → Ticket (SRV), keine stille Absenkung des Ziels.

## Schritte

1. Stand mit `task load` auf den Pi bringen bzw. vom Rechner im Heimnetz gegen den Pi starten: `task load -- -url http://<pi>:8080 -token <T> -rooms 2 -players 3 -duration night`.
2. Nach dem Lauf: Markdown-Tabelle und Bewertung in das Ergebnis dieser Datei und in B-042 übernehmen.
3. Bewertung `verfehlt` oder `knapp` → Ticket mit Messwerten (SRV).
4. B-042 auf `erledigt` setzen und archivieren, wenn die Bewertung vorliegt; `Status: fertig`.

## Fertig, wenn

- [ ] AC-06: Tabelle und Bewertung des Pi-Laufs stehen im Ergebnis und in B-042; B-042 ist archiviert.

## Prüfen

Manuell durch 🧑 am Pi.

## Ergebnis

2026-10-07, **PC-Nachweis, Pi offen.** Auf Entscheidung 🧑 (Ralf, Interview mit Agent Claude Opus 5.5) ausgewertet: Messlauf gegen den Spielserver am Entwicklungs-PC (Windows, k3c-dev, `task start`), nicht am Pi. Gestartet vom Agenten: `task load -- -url http://localhost:8080 -token <Test-Token> -rooms 2 -players 3 -duration night`, Exit-Code 0. Parallel liefen ein eigener Raum von 🧑 (Abnahmen MON2.4/N2.4) und Vite. Bericht: `reports/load-20261007-065254.md` / `.json` (lokal, nicht im Repo).

| Raum | Phase | Proben | p99 min (ms) | p99 Mittel (ms) | p99 max (ms) | CPU Mittel (%) | CPU Spitze (%) | Bewertung |
|---|---|---|---|---|---|---|---|---|
| test-load-xmt4vj-0 | day | 120 | 1.26 | 1.62 | 8.45 | – | – | erreicht |
| test-load-xmt4vj-0 | dusk | 12 | 1.43 | 1.77 | 2.26 | – | – | erreicht |
| test-load-xmt4vj-0 | night | 61 | 1.26 | 1.88 | 4.79 | – | – | erreicht |
| test-load-xmt4vj-1 | day | 120 | 1.08 | 1.50 | 6.85 | – | – | erreicht |
| test-load-xmt4vj-1 | dusk | 12 | 1.28 | 1.42 | 1.64 | – | – | erreicht |
| test-load-xmt4vj-1 | night | 61 | 1.29 | 1.94 | 7.72 | – | – | erreicht |

**Gesamt am PC: erreicht** (Ziel p99 < 10 ms). CPU fehlt, weil `/api/status` unter Windows keine CPU-Quelle hat (B-175/AC-04: „wo die Quelle existiert“).

- **AC-06: nicht erfüllt.** Der PC-Lauf belegt nur, dass Werkzeug und Bewertung durchlaufen; er sagt nichts über den Pi (Handmessung 2026-10-03: Nacht 10,2 bis 10,3 ms). Der Pi-Lauf fehlt; B-042 bleibt eingeplant und wird nicht archiviert. Die Session bleibt `offen`.
- Keine neuen Tickets.

2026-10-08, **verworfen (PJ3.2, B-359):** Die Abnahme am Gerät ist nach HW1.5 verschoben (Projekt ABN); bisherige Nachweise oben gelten weiter.
