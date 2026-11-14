# WZ8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Chunks je Insel und Stufe planen: Seed oder feste Objekte
- **Projekt:** WZG

## Ziel

Je Insel und Stufe lassen sich Chunks planen, mit fester Art und Inhalt aus Chunk-Seed oder festen Objekten. Der Generator setzt sie deterministisch ein, der Welt-Betrachter bearbeitet sie mit sofortiger Chunk- und Live-Vorschau, auch für ungespeicherte Entwürfe. **Am Ende sichtbar:** im Asset-Studio eine geplante Sondergegend als ungespeicherter Entwurf in Chunk- und Live-Vorschau (beide aus dems …

## Ergebnis

2026-10-17 (Review WZ8.6): AC-01 bis AC-09, AC-11 bis AC-14 mit Nachweis aus Code, Tests und getrennten Reviewern (Ergebnisse WZ8.2–WZ8.6). 2026-10-18 (WZ8.7, 🧑 am PC): AC-10 abgenommen; Punkt 5 der Abnahme-Liste erst nach der Korrektur B-486 (stumme Ablehnung beim Speichern, getrennter Reviewer: keine schweren Befunde). Befunde und neue Tickets: B-475 (Annahmen zu festen Ereignis-Objekten), B-485 (Bedienung des Chunk-Editors, Sprint WZ17), B-486 (behoben), B-487 (Laden und Prüfung ohne Meldung bei Ablehnung). Version: v… vorgeschlagen: keine Änderung (Werkzeug- und Generator-Sprint, Golden un …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `B-461/AC-01` Format und Regeln als REG-Beschluss von 🧑, Regeltext in `docs/rules/`, Gloss … |
| AC-02 | geprüft | `B-461/AC-02` Ohne Planung sind `level-*.json` und `sim-*.json` byte-gleich. |
| AC-03 | – | `B-461/AC-03` Der Generator setzt geplante Chunks deterministisch ein (Golden-Fall mit Tes … |
| AC-04 | geprüft | `B-461/AC-04` Der Inhalt geplanter Chunks kommt nur aus dem eigenen Strom. |
| AC-05 | geprüft | `B-461/AC-05` Die Biom-Summen gelten inklusive Planung (`level.Validate` über 500 Seeds je … |
| AC-06 | geprüft | `B-461/AC-06` Eine ungültige Planung wird beim Laden abgelehnt und benannt, je Fall ein Te … |
| AC-07 | geprüft | `B-461/AC-07` Feste Objekte wirken in der Sim wie gewürfelte, mit 2 Spielern; `sim.CreateI … |
| AC-08 | – | `B-461/AC-08` Die Vorschau-Funktion aus WZ7.1 nimmt die Planung als Option „geplante Chunk … |
| AC-09 | – | `B-461/AC-09` Editor im Welt-Betrachter mit sofortiger Chunk- und Live-Vorschau, speichert … |
| AC-10 | – | `B-461/AC-10` 🧑 nimmt eine geplante Sondergegend als Entwurf in Chunk- und Live-Vorschau a … |
| AC-11 | geprüft | `B-461/AC-11` Chunk-Vorschau auf Insel und Stufe, gleiche Quelle wie die Live-Vorschau. |
| AC-12 | geprüft | `B-461/AC-12` Schreibregel: atomar, format-stabil, Stand-Prüfung (veraltete Datei wird nic … |
| AC-13 | geprüft | `B-461/AC-13` Benchmark der Level-Erzeugung: ≤ +10 % ohne Planung, ≤ +10 % mit Test-Planun … |
| AC-14 | – | Das Review weist jedes Kriterium nach; `task check:all` und `task check:go` sind grün, Gol … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-461 | erledigt | AC-01 –, AC-02 geprüft, AC-03 –, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft, AC-07 geprüft, AC-08 –, AC-09 –, AC-10 –, AC-11 geprüft, AC-12 geprüft, AC-13 geprüft | Chunks einer Insel-Stufe lassen sich planen, per Seed oder mit festen Objekten |
