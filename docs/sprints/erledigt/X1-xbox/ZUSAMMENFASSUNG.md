# X1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Xbox-Machbarkeit
- **Projekt:** –

## Ziel

Die Steuerung und der Ton auf der Xbox sind geprüft. Am Ende sichtbar: Bericht in `reports/` (mit Audio-Ergebnis), Steuerungstabelle in `game-design.md` ohne „vermutlich“, Audio-Ergebnis in `game-design.md`.

## Ergebnis

2026-10-03, Agent (Claude Opus 5.5) in X1.3, Doku-Sprint ohne Review. AC-01 und AC-05 (Bericht): Ergebnis X1.2; AC-04: Ergebnis X1.1; AC-02, AC-03, AC-05 (Doku): Ergebnis X1.3. Neue Tickets: B-194 (Ruckeln im Split-Screen auf der Xbox), B-195 (Debug-Overlay auf der Xbox nicht aufrufbar). Version: v0.5.1 vorgeschlagen (Patch: nur Doku und Messung, keine neue Funktion); gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Ein Bericht der Xbox liegt in `reports/` (B-006/AC-01). |
| AC-02 | geprüft | Die Steuerungstabelle enthält kein „vermutlich“ mehr, inklusive Skill-Tasten (B-006/AC-02, … |
| AC-03 | geprüft | Sprite-Budget und „HTTPS ja/nein“ stehen in `game-design.md`. |
| AC-04 | geprüft | Die Gamepad-Testseite prüft Audio: Zustand des `AudioContext`, Abspielversuch ohne Geste, … |
| AC-05 | geprüft | Ein Bericht der Xbox mit Audio-Ergebnis liegt in `reports/`, Format- und Autoplay-Ergebnis … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-006 | erledigt | AC-01 geprüft, AC-02 geprüft | Gamepad-Test auf der Xbox ist ausgewertet |
| B-026 | erledigt | AC-02 geprüft | Skill-Tasten am Controller sind festgelegt |
| B-166 | erledigt | AC-04 geprüft, AC-05 geprüft | Audio-Autoplay und Formate auf Edge der Xbox sind geprüft |
