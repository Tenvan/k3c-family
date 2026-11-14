# BD2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Rückmeldungs-Ereignisse gedrosselt und geprüft, Lobby startet nie still leer
- **Projekt:** BED

## Ziel

Rückmeldungs-Ereignisse belasten das Netz nicht mehr je Tick, ihr Beispiel ist gegen die Simulation geprüft, und die Lobby meldet einen fehlenden Spielstand, statt still neu zu starten. Am Ende sichtbar: `task check:go` und `task check` grün, Hinweis in der Lobby.

## Ergebnis

–

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Eine gehaltene Skill-Taste ohne Ziel erzeugt höchstens ein `castFailed` je Tastendruck (B- … |
| AC-02 | – | Ein Server-Test vergleicht die Schlüssel von `s2c-snapshot-rueckmeldung.json` mit den Erei … |
| AC-03 | – | Ein gewählter, nicht mehr vorhandener Spielstand startet kein leeres Spiel, sondern zeigt … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-378 | erledigt | AC-01 geprüft | Eine gehaltene Skill-Taste ohne Ziel meldet castFailed einmal je Tastendruck |
| B-377 | erledigt | AC-02 – | Ein Server-Test prüft das Beispiel der Rückmeldungs-Ereignisse gegen die Simulat … |
| B-382 | erledigt | AC-03 – | Ein gewählter Spielstand startet nie leer neu |
