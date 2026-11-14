# SO8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Skill-Sounds je Klasse
- **Projekt:** SND

## Ziel

Der Skill jeder Klasse (Tank, Zauberer, Heiler, Dieb) klingt eigen. Am Ende hörbar: Skill wirken im Spiel spielt den Klassen-Ton.

## Ergebnis

2026-10-24: AC-01 bis AC-03 erfüllt (SO8.1, SO8.2 › Ergebnis; SO8.3 › Ergebnis), AC-04 am 2026-10-31 von Ralf (🧑) abgenommen (vier Töne gehört, im Chat bestätigt; Claude hat nicht mitgehört), Sprint erledigt. Review ohne schwere Befunde, keine Fixes. Mit im Branch: Planung WZ29 Revision 3, Sprints GEB17 und KB4. Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Vier Slots `sfx:skill:<linie>` mit Kandidaten und Credit, Atlas neu (B-576/AC-01). |
| AC-02 | erfüllt | Das Wirken eines aktiven Skills spielt den Ton seiner Linie, Rückfall `sfx:skill` (B-576/A … |
| AC-03 | erfüllt | Für passive und einzelne Skills lässt sich ein Ton über `sfx:skill:<skillId>` definieren, … |
| AC-04 | – | 🧑 hat die vier Töne gehört und bestätigt oder geändert (B-576/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-576 | erledigt | AC-01 erfüllt, AC-02 erfüllt, AC-03 erfüllt, AC-04 – | Jede Klasse hat einen eigenen Skill-Sound |
