# WZ14 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Geplante Chunks tragen Ruinen und Gegnerlager
- **Projekt:** WZG

## Ziel

`B-468 › Ziel`: Ein geplanter Chunk kann eine Ruine tragen (gebautes Nutzgebäude mit 25 % HP an fester Lage, auch als Vorposten außerhalb der Mauern); sie wird beim Laden geprüft, wirkt im Spiel wie ein gebautes Gebäude ihrer Art, ist Hub-neutral, wird repariert (außerhalb der Mauern nur ohne Gefahr) und kehrt nach einem Burgfall zurück; Editor und Vorschau des Studios setzen und zeigen sie. **Am …

## Ergebnis

2026-10-21, Review WZ14.8 (Claude, getrennter Reviewer Opus): AC-01–AC-05 belegt (Ergebnisse WZ14.1–WZ14.7), Golden unverändert, keine Planung mit Ruine in `data/worlds/`, Protokoll 7, `PreviewChunkObjects` unverändert. Behoben: S1 – Vorschau-Ausbau und „alles bauen“ setzten bei Ruinen MaxHP = HP (keine Schäden sichtbar, Reparatur gedeckelt); L2 – „nicht gebaut“ in der Vorschau verlor die Kennung `ruin` (Test `building_ruin_preview_test.go`). Neue Tickets: B-521 (types.ts über 400 Zeilen), B-522 (Zufalls-Ressourcen auf der Ruine), B-523 (Ruine auf Hub-Platz-Lage). `task check:all`, `task check …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `B-468/AC-01` Regeltext, Glossar, Werte in `data/ruins.json` – WZ14.1, WZ14.2 |
| AC-02 | geprüft | `B-468/AC-02` Format `buildings` lesen und prüfen, je Fehlerfall ein Go-Test – WZ14.2 |
| AC-03 | – | `B-468/AC-03` Ruine als Platz, Vorposten-Schutz, Wiederaufbau, Burgfall, Dev-Befehl, Spiel … |
| AC-04 | – | `B-468/AC-04` Golden byte-gleich, Benchmark ≤ +10 % – WZ14.2, WZ14.3 |
| AC-05 | – | `B-468/AC-05` Protokoll, Vorschau, Grafik, Editor, Dev-Befehl im Studio – WZ14.5, WZ14.6, … |
| AC-06 | geprüft | `B-468/AC-06` Review, alle Prüfungen grün – WZ14.8 |
| AC-07 | geprüft | `B-468/AC-07` Abnahme durch 🧑 am PC – WZ14.9 |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-468 | erledigt | AC-01 –, AC-02 geprüft, AC-03 –, AC-04 –, AC-05 –, AC-06 geprüft, AC-07 geprüft | Geplante Chunks tragen Ruinen und Gegnerlager |
