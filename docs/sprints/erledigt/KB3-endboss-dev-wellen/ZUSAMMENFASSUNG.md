# KB3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Endboss-Auslöser anzeigen, Dev-Overlay-Link, Wellenabstand unter Tage messen
- **Projekt:** KMP

## Ziel

Auslöser des Endbosses sichtbar, Dev-Link mit `?dev=1`, Wellenabstand über 100 Seeds gemessen. Am Ende sichtbar: Overlay, Dev-Seite und Messbericht.

## Ergebnis

- 2026-10-23, Claude autonom: AC-01 bis AC-03 umgesetzt bzw. gemessen. Befund: Wellenabstand mit Adern über dem Korridor (Median 19,37 min), neues Ticket B-579 (Entscheidung 🧑). Annahme: `noticeUnits` 40.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Aktionsliste nennt den Bau-Auslöser des Endbosses, das Overlay zeigt ihn (B-563/AC). |
| AC-02 | – | Die Entwicklerseite öffnet das Spiel mit `?dev=1` (B-564/AC). |
| AC-03 | – | Der Wellenabstand ist über 100 Seeds über `sim_test` gemessen (B-565/AC). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-563 | erledigt | AC-01 – | Die Aktionsliste nennt den Bau-Auslöser des Endbosses, das Overlay zeigt ihn |
| B-564 | erledigt | AC-02 – | Die Entwicklerseite öffnet das Spiel mit Debug-Overlay (?dev=1) |
| B-565 | erledigt | AC-03 – | Der Wellenabstand unter Tage mit besetzten Adern ist über 100 Seeds gemessen |
