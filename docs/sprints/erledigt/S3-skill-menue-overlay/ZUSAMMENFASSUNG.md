# S3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Skill-Menü, Tasten und Aktionen-Overlay
- **Projekt:** –

## Ziel

Schlag, Skill-Slots 1–4 und Skill-Menü sind mit Controller, Tastatur und Touch bedienbar; gültige Aktionen erscheinen als Overlay am Ort mit der Taste des zuletzt benutzten Geräts. Am Ende sichtbar: 🧑 spielt am Gerät Schlag, Skill, Punkte verteilen und liest die Aktionen im Overlay.

## Ergebnis

2026-10-05, Review S3.5 (Diff `src/` gelesen, `task check` und `task check:go` grün): keine schweren Befunde (B unbelegt, View + Menu unverändert, kein `Math.random()`, Menüs und Overlays je Slot getrennt). - **AC-01 bis AC-04** nachgewiesen in S3.1 (Slot-Belegung getestet), S3.2 (Menü, Tests für 2 Spieler, Browser-Pane), S3.3 (Aktion → Taste/Text getestet, Overlay im Pane mit 2 Spielern); Wiederbeleben-Teil von AC-04 verschoben (B-120). - **AC-05** angenommen, Validierung offen (S3.4); S3.4 steht im Fahrplan unter „Offen am Gerät“, B-124 und B-125 bleiben bis dahin offen. - Neue Tickets: B-28 …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Reine Funktion für die Slot-Belegung je Gerät ist getestet (B-124/AC-01). |
| AC-02 | – | Skill-Menü und Slots sind mit Controller, Tastatur und Touch bedienbar (B-124/AC-02). |
| AC-03 | – | Reine Funktion Aktion → Text und Symbol je Gerät ist getestet (B-125/AC-01). |
| AC-04 | verschoben | Das Overlay zeigt die Aktionen am Ort für Bauen, Zahlen, Wiederbeleben und Schlag (B-125/A … |
| AC-05 | – | 🧑 hat Slots, Menü, Tasten und Overlay am Gerät abgenommen (B-124/AC-03, B-125/AC-03). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-124 | erledigt | AC-01 –, AC-02 –, AC-05 – | Der Client hat Schlag, Skill-Slots, Skill-Menü und die Tasten für Controller, Ta … |
| B-125 | erledigt | AC-03 –, AC-04 verschoben, AC-05 – | Gültige Aktionen erscheinen überall in der Welt als Overlay am Ort |
