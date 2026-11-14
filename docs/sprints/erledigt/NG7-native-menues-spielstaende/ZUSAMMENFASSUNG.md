# NG7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Menüs, Lobby, Spielstände, Optionen und Pause
- **Projekt:** NAT

## Ziel

Das Spiel führt vom Start bis ins Spiel und zurück. Am Ende sichtbar: Hauptmenü → Lobby mit zwei Spielern → Spiel → Pause → Hauptmenü → Weiter.

## Ergebnis

2026-11-11 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions, behobene Befunde siehe Review-Session. Offen: Schriftdatei fehlt (Debug-Schrift); Fenster und Geräte am echten PC ungeprüft. Kein Abnahme-Ticket. Version: keine vorgeschlagen (noch kein spielbarer Stand). Entscheidung 🧑 offen: Endet das Spiel auch nach einem Sieg?

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Hauptmenü und Menü-Gerüst (`B-800/AC-01`). |
| AC-02 | – | Lobby mit 1–2 Spielern (`B-800/AC-02`). |
| AC-03 | – | Spielstände über `view.Session` (`B-800/AC-03`). |
| AC-04 | – | Optionen, Pause, Spielende (`B-800/AC-04`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-800 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Das native Spiel hat Hauptmenü, Lobby für 1–2 Spieler, Spielstand-Menü, Optionen … |
