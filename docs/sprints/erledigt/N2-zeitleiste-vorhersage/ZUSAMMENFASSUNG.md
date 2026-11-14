# N2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Flüssige Darstellung: Zeitleiste, Extrapolation, eigene Vorhersage
- **Projekt:** –

## Ziel

Figuren laufen gleichmäßig trotz schwankender Zustände, der eigene Monarch reagiert sofort. Am Ende sichtbar: im Debug-Overlay Latenz (Mittel, p95) und Puffer-Verzögerung; am TV kein Ruckeln (🧑).

## Ergebnis

- 2026-10-04, Review N2.3: AC-01, AC-02 (Ergebnis N2.1), AC-03, AC-04, AC-07, AC-08 (Ergebnis N2.2), AC-05 (`task check` grün) mit Nachweis; Ergänzungs-Tests in N2.3. - AC-06: angenommen, Validierung offen (N2.4, 🧑 an der Xbox; Fahrplan „Offen am Gerät“). - Behoben: Gummiband der Vorhersage unter Latenz (Vorlauf), Latenz-Fenster begrenzt, doppelter Tick, Zeitraffer-Schwelle, keine Vorhersage tot/angehalten, `moveX` gerastert, Glossar. B-278 → B-279 umbenannt (Kollision mit N1). - Abhängigkeit: `ack` beim Einreihen statt beim Senden (N1); B-277 und B-181 archiviert, B-279 bleibt offen. - Versio …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | B-277/AC-01 |
| AC-02 | – | B-277/AC-02 |
| AC-03 | – | B-277/AC-03 |
| AC-04 | – | B-277/AC-04 |
| AC-05 | – | B-277/AC-05 |
| AC-06 | – | B-277/AC-06 (Hardware, keine Abhängigkeit des Reviews) |
| AC-07 | – | B-181/AC-01 |
| AC-08 | – | B-181/AC-02 |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-277 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-06 – | Der Client zeichnet trotz schwankender Zustände flüssig und wartet bei der eigen … |
| B-181 | erledigt | AC-07 –, AC-08 – | Das Debug-Overlay zeigt die Latenz von Eingabe bis Bild |
