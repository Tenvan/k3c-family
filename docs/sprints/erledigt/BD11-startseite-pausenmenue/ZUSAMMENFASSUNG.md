# BD11 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Eine Startseite, Navigation im Pausenmenü, Vollbild halten
- **Projekt:** BED

## Ziel

Das Spiel ist die einzige Seite: `ip:8080` öffnet das Spielmenü; alle Seiten, Werkzeuge und Test-Szenarien laufen aus dem Menü, ohne Landingpage und ohne URL-Syntax; Vollbild bleibt. Am Ende sichtbar: Xbox/Pi: Adresse öffnen, mit dem Pad Spiel, Optionen, Werkzeuge und Stress-Szenen erreichen.

## Ergebnis

2026-11-09 🧑 Chat: BD11 abgenommen, AC-01 bis AC-07 erfüllt (Tests und Gerätetest Xbox/Pi mit v0.23.0). Behobene Befunde: Spiel nicht verlassbar (A vom Verlassen bestätigte „Fortsetzen“, `7951a652`), URL nur mit Port zeigte die Landingpage (jetzt das Spiel). Auffälligkeiten am Gerät: Xbox läuft nur im Canvas-Fallback (kein WebGL, keine Seite grün), Leistung trotzdem akzeptabel; offen bleibt eine Aussage, ob Pi 3 (Server) und Xbox (Client) generell tragen (B-763, B-764, RX1.5/RX1.6) und ob ein anderer Browser hilft (Xbox kennt nur Edge). Kein neues Abnahme-Ticket für Sim, Balance und Gen: der S …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | `GET /` liefert das Spiel, keine Landingpage (`B-768/AC-01`, Test). |
| AC-02 | erfüllt | Menü-Ebenen und Kacheln für alle Seiten und Szenarien, bedienbar per Pad, Tastatur, Tippen … |
| AC-03 | erfüllt | Werkzeuge als Panel im selben Dokument (`B-768/AC-03`, Test). |
| AC-04 | erfüllt | `RunConfig` als einzige Quelle für Start-Parameter (`B-768/AC-04`, Test). |
| AC-05 | erfüllt | Vollbild halten (`B-768/AC-05`, Test). |
| AC-06 | erfüllt | Stress-Kachel startet ohne Neuladen (`B-768/AC-06`, Test). |
| AC-07 | erfüllt | Abnahme am Gerät: Xbox und Pi, nur Pad (`B-768/AC-07`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-768 | erledigt | AC-01 erfüllt, AC-02 erfüllt, AC-03 erfüllt, AC-04 erfüllt, AC-05 erfüllt, AC-06 erfüllt, AC-07 erfüllt | Eine Startseite, Navigation im Pausenmenü, Vollbild halten |
