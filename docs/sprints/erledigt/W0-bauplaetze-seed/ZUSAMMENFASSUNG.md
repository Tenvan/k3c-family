# W0 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bauplätze aus dem Seed
- **Projekt:** –

## Ziel

Jede Stufe hat nach dem Seed alle Bauplätze des neuen Modells: feste Hub-Plätze mit Hub-Stufe aus den Daten, je Seite fünf Mauerlinien mit Mauer-, Turm- und Tor-Platz (Linie 1 fest, Linien 2–5 je Seed nur nach außen um 0 bis +4 Units gestreut) und eine Farm je Seite; Linien und Tore sind nur in der Reihenfolge der Beschlüsse bezahlbar. Am Ende sichtbar: `task check:go` grün mit Tests für Layout, F …

## Ergebnis

- 2026-10-04 (W0.4, autonom): AC-01–AC-08 mit Nachweis (W0.1: AC-04; W0.2: AC-01, AC-02; W0.3: AC-03, AC-05, AC-06; W0.3b: AC-07, AC-08); `task check` und `task check:go` grün (`-race` übersprungen, kein C-Compiler). - Golden geprüft: W0.1 +7 Plätze, W0.3 +26 Linien-Plätze (beide „Q43“), W0.3b Camps (`level-*`: nur Chunk-Arten/Ressourcen, Portale, Breite und Hub gleich; „B-261“); `rng.json` unverändert. `cause` ist Zusatzfeld, Protokoll v3 und Client-Tests unberührt. - Keine schweren Befunde. Aufgeräumt: leerer `TestSiteDataMauerUndTurmSindLinie1` entfernt, Prüfung „keine Mauer/Turm/Tor in `si …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Das Layout jeder Stufe enthält je Seite 5 Mauerlinien mit Mauer-, Turm- (8 Units innen) un … |
| AC-02 | erfüllt | Ressourcen-, Portal- und Camp-Positionen sind gegenüber vor W0 unverändert, `testdata/gold … |
| AC-03 | erfüllt | Linie k ist erst ab Hub-Stufe k und nach gebauter Mauer der Linie k−1 derselben Seite beza … |
| AC-04 | erfüllt | Jeder Bau aus `docs/rules/materialien-gebaeude.md` § 3 hat genau einen Hub-Platz (Ausnahme … |
| AC-05 | erfüllt | Die Fixtures `testdata/saves/v1/` und `v2/` sowie ein Spielstand der aktuellen Version lad … |
| AC-06 | erfüllt | Zwei Spieler bezahlen Plätze gleichzeitig ohne Reihenfolge-Effekt (Test). |
| AC-07 | erfüllt | Das Ereignis `playerDown` nennt in `cause` die Gegnerart bei Nahkampf und Geschoss (fester … |
| AC-08 | erfüllt | Jedes Camp liegt bei jeder Streuung der Linien (0…+4) ≥ 4 Units (2 × `payRangeUnits`) von … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-206 | erledigt | AC-01 erfüllt, AC-02 erfüllt, AC-03 erfüllt, AC-04 erfüllt, AC-05 erfüllt | Alle Bauplätze sind feste Punkte aus Daten und Level-Seed, Mauerlinien schalten … |
| B-182 | erledigt | AC-07 erfüllt | Das Ereignis playerDown nennt, was den Monarchen getötet hat |
| B-261 | erledigt | AC-02 erfüllt, AC-08 erfüllt | Ein Camp liegt nie so nah an einem Linien-Platz, dass Zahlziele sich überlagern |
