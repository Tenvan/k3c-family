# F3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Feedback-Ereignisse in der Simulation
- **Projekt:** –

## Ziel

Die Simulation erzeugt die beschlossenen Feedback-Ereignisse deterministisch und mit Obergrenze je Tick. Am Ende sichtbar: Go-Tests je Ereignistyp, `task check:go` grün, Ereignisse im Ergebnis von `sim_run` (k3c-dev).

## Ergebnis

- 2026-10-03, Review F3.3: AC-01 bis AC-05 belegt in den Ergebnissen F3.1, F3.2 und F3.3; `task check` und `task check:go` grün. - Keine schweren Befunde; Golden-Diff betrifft nur `events`, `rng.json` unverändert. B-139 archiviert; offen bleiben B-189 (Flüchtender ohne `kill`) und B-190 (`eventsDropped` im Protokoll, F4). - Obergrenze für F4: K = 32 je Tick und Stufe (vorläufig), größter gemessener Tick 19, Mittel ≤ 0,1 Ereignisse je Tick; Budget ≤ 200 Byte je Tick und Client misst F4. - Version: v0.5.0 gesetzt (2026-10-03, Bestätigung 🧑; Minor: Feedback-Ereignisse in der Simulation; gemeinsam …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Je Ereignistyp der beschlossenen Liste prüft ein Test Typ, Ort und Beteiligte (B-139/AC-01 … |
| AC-02 | geprüft | Ereignisfolgen sind bei gleichem Seed und gleichen Eingaben identisch (B-139/AC-02). |
| AC-03 | geprüft | Die Obergrenze je Tick schützt Tod- und Bau-Ereignisse und zählt Verworfene (B-139/AC-03). |
| AC-04 | geprüft | Auf der Insel entsteht ein Ereignis nur in seiner Stufe (B-139/AC-04). |
| AC-05 | geprüft | Golden-Daten sind aktualisiert und `task check:go` grün (B-139/AC-05). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-139 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft | Die Simulation meldet Feedback-Ereignisse für Treffer, Münzen, Schläge und Tod |
