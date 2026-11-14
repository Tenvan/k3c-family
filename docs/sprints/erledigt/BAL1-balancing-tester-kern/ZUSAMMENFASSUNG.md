# BAL1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Balancing-Tester: Kern und Replay
- **Projekt:** –

## Ziel

Ein Werkzeug spielt viele deterministische Läufe mit Bots und liefert Kennzahlen als JSON; jeder Lauf ist als Datei wiederholbar. Am Ende sichtbar: ein Befehl im Terminal erzeugt den Kennzahlen-Report, eine Replay-Datei lässt sich in k3c-dev abspielen und liefert denselben Endzustand-Hash.

## Ergebnis

2026-10-04, Agent (Claude Opus 5.5) in BAL1.4, leichtes Review. AC-01, AC-02: Ergebnis BAL1.1; AC-03 bis AC-05: Ergebnis BAL1.2; AC-06, AC-07: Ergebnis BAL1.3 (AC-07 nach dem Review-Commit erneut grün, `TestGleichzeitigesSpeichern` flaky, B-187). Behobener Befund: `replay_run` und `ReadReplay` hatten keine Obergrenze für die Ticks aus der Datei (konnte den MCP-Server blockieren); jetzt höchstens 1 000 000 Ticks, Test `TestZuvieleTicksAbgelehnt`. Neue Tickets: keine. B-159 archiviert, B-099 bleibt eingeplant (BAL2, BAL3). Version: v0.5.1 vorgeschlagen (Patch: nur Entwickler-Werkzeug ohne Änderu …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Ein Lauf mit denselben Seeds, Parametern und Daten liefert byte-gleiche Kennzahlen (B-099/ … |
| AC-02 | – | Mindestens zwei Bot-Profile und die Kennzahlen aus B-099 › Anforderungen sind umgesetzt un … |
| AC-03 | – | Aufnahme und Wiedergabe eines Bot-Laufs liefern denselben Endzustand-Hash (B-159/AC-01). |
| AC-04 | – | Der Tester schreibt auf Wunsch je Lauf eine Replay-Datei mit Seed, Parametern, Datenstand- … |
| AC-05 | – | Unbekannte Version und abweichender Datenstand ergeben Fehler beziehungsweise Warnung (B-1 … |
| AC-06 | – | k3c-dev spielt eine Replay-Datei ab und nennt Endzustand-Hash und Burgfall-Tick (B-159/AC- … |
| AC-07 | – | `task check:go` ist grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-099 | verworfen | AC-01 –, AC-02 – | Ein automatischer Balancing-Tester prüft Regeln und Werte gegen messbare Ziele |
| B-159 | erledigt | AC-03 –, AC-04 –, AC-05 –, AC-06 – | Ein Lauf ist als Datei aus Seed und Eingaben wiederholbar |
