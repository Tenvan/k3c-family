# W11 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bauplätze im Protokoll, Abstand der Treppen, Ausbilden und Tauschen, Figuren
- **Projekt:** WRT

## Ziel

Der Client zeigt nur Bauplätze, die zur Hub- und Stufen-Lage passen, in lesbarem Abstand, dazu Ausbilden, Tauschen und die Wirtschafts-Figuren. Am Ende sichtbar: Hub am TV ohne überlappende Preisschilder, Aktionen-Overlay mit Ausbilden und Tauschen, Adern, Plantage und Lager als Figuren; von 🧑 abgenommen.

## Ergebnis

2026-10-25, Claude (W11.5): - Prüfläufe: `task check` und `task check:go` grün. Beim ersten Lauf ist `TestZweiGeraeteGetrennteReihen` unter Last abgebrochen (bekannt, B-284), einzeln grün. `check:race` lokal übersprungen (kein C-Compiler). - Kriterien: AC-01 durch W11.1 (Treppen-Abstand). AC-02 durch W11.2 (nur erreichbare Plätze mit `hubLevel`). AC-03 durch W11.3 (`train` und `trade` in `actions`). AC-04 Client-Teil durch W11.4. - Review zu W11.2/.3: zwei Befunde, beide behoben (Client ignoriert unbekannte Aktionen; Typen). Dazu ein Längen-Schutz in `filterSites`. - Offen: Die Abnahme am TV ( …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die Bauplätze im Hub haben bei den Treppen mindestens 8 Units Abstand, `task balance` und … |
| AC-02 | – | Das Protokoll trägt Bauplätze mit Platz- und Hub-Stufe; nicht erreichbare Plätze erscheine … |
| AC-03 | – | Der Server liefert Ausbilden und Tauschen in `actions` (B-410). |
| AC-04 | – | Der Client zeigt Ausbilden und Tauschen im Aktionen-Overlay sowie Adern, Plantage und Lage … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-409 | eingeplant | AC-01 – | Bauplätze im Hub stehen zu eng, besonders die Treppen |
| B-208 | erledigt | AC-02 – | Das Protokoll trägt die Bauplätze des Layouts sowie Platz- und Hub-Stufe zum Cli … |
| B-410 | eingeplant | AC-03 –, AC-04 – | Aktionen-Overlay zeigt Ausbilden und Tauschen nicht, der Server liefert sie nich … |
| B-408 | verworfen | AC-02 –, AC-04 – | Bauplätze aller Stufen werden schon im Hub einer niedrigen Stufe angezeigt |
| B-412 | verworfen | AC-04 – | Adern, Plantage und Lager haben keine Anzeige als Figuren im Client |
