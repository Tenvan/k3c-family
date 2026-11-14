# W6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Anzeigen für Bau, Lager, Hub und Bürger
- **Projekt:** WRT

## Ziel

Spieler sehen, warum ein Bauplatz wartet, Lagerstand und Hub-Stufe mit Kosten sowie Berufe, Händler, Limit und Heilplatz. Am Ende sichtbar: HUD und Bauplätze am TV, von 🧑 abgenommen.

## Ergebnis

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). W6.1–W6.3 und W6.5 fertig (#273), W6.4 (Abnahme am Gerät) verworfen, B-117 und B-126 verworfen. Status `erledigt`, weil das Werkzeug für Sprints keinen Status `verworfen` kennt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Reine Funktionen für Wartegrund, Lagerstand-Text, Limit-Text und Berufsanzeige sind getest … |
| AC-02 | – | HUD und Bauplätze zeigen Wartegrund, Lagerstand, Hub-Stufe und Kosten (B-117/AC-02). |
| AC-03 | – | Berufe, Limit, Händler und Heilplatz werden angezeigt (B-126/AC-02). |
| AC-04 | – | 🧑 hat beide Anzeigen am PC (Tastatur) und an der Xbox (Controller, nach B-314) mit höchste … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-117 | verworfen | AC-01 erfüllt, AC-02 –, AC-04 – | Der Client zeigt Wartezeit, Lagerstand, Hub-Stufe, Adern und Plantage |
| B-126 | verworfen | AC-01 erfüllt, AC-03 –, AC-04 – | Der Client zeigt Berufe, Ausbildung, Händler, Truppen-Limit und Heilung |
