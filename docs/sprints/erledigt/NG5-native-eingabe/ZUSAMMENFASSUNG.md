# NG5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Eingabe mit Gamepads und Tastatur
- **Projekt:** NAT

## Ziel

1–2 Gamepads und die Tastatur steuern Spieler und Menüs mit der heutigen Belegung. Am Ende sichtbar: zwei Pads steuern je einen Monarchen, ein Pad kann während des Spiels kommen und gehen.

## Ergebnis

2026-11-11 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions. Behobene Befunde: siehe Review-Session. Offen: Schriftdatei fehlt (Debug-Schrift); am echten Gerät ungeprüft (Fenster, Gamepads). Kein Abnahme-Ticket. Version: keine vorgeschlagen (noch kein spielbarer Stand). Am Gerät zu prüfen (🧑): zwei Pads, Ein- und Ausstecken, RT, Stick-Schwellen, Ziffernblock.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Belegung nach `docs/rules/bedienung.md` (`B-798/AC-01`). |
| AC-02 | – | Zuordnung bei Pad kommt, geht, zwei Pads, kein Pad (`B-798/AC-02`). |
| AC-03 | – | B zurück in Menüs, im Spiel unbelegt; View + Menu zurück ins Menü (`B-798/AC-03`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-798 | erledigt | AC-01 –, AC-02 –, AC-03 – | Im nativen Spiel steuern 1–2 Gamepads und die Tastatur je einen Spieler und die … |
