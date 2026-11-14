# RD2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Renderer und Stil nach der Sichtprüfung: Schatten, Materialien, Sichtbarkeit, Rig-Proportionen
- **Projekt:** PAP

## Ziel

Renderer und Stil liefern alles, was die Asset-Sprints MG9–MG11 brauchen.

## Ergebnis

2026-11-07 Claude: AC-01 bis AC-04 umgesetzt und belegt (Ergebnisse RD2.1 bis RD2.4), `task check` und `task check:go` grün, Race lokal übersprungen (kein gcc, CI). Offen bei Ralf: Sichtprüfung Monarch idle 4 gegen den Prototyp (B-712/AC-01) samt Optik der neuen Materialfarben. Behoben: Pfote schwebte über dem Boden (Reviewer RD2.4). Neues Ticket: B-735.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | B-712/AC-01 bis AC-04 |
| AC-02 | – | B-713/AC-01, AC-02 |
| AC-03 | – | B-714/AC-01 bis AC-03 |
| AC-04 | – | B-715/AC-01 bis AC-03 |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-712 | erledigt | AC-01 – | Stil zeigt Papierschatten wie der Prototyp und kennt Metall-, Kristall- und Biom … |
| B-713 | erledigt | AC-02 – | renderKey des Reiters enthält den Hash der Reittier-Spec |
| B-714 | erledigt | AC-03 – | Keyframes schalten Teile mit dem Feld sichtbar ein und aus |
| B-715 | erledigt | AC-04 – | Rigs vierbeiner und figur erlauben Proportionen, Füße und Gangarten je Tier oder … |
