# BAL4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Abgleich Spielmetrik und Simulator
- **Projekt:** BAL

## Ziel

Simulatorwerte und Werte echter Abende stehen nebeneinander, Abweichungen haben Ursache und Beschluss. Am Ende sichtbar: Abgleichtabelle in `docs/rules/`, beschlossene Änderungen in `data/`, `task balance` grün.

## Ergebnis

2026-10-25, Claude: Die Abgleichtabelle ist vorläufig (BAL4.1). Beschluss: keine Wertänderung, bis echte Spieleabende vorliegen (BAL4.2, vorentschieden), deshalb nichts umzusetzen (BAL4.3). Den echten Abgleich liefern P1 und BR1.2.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die Abgleichtabelle enthält jede gemeinsame Kennzahl mit Simulatorwert, Wert echter Abende … |
| AC-02 | – | Zu jeder Abweichung über der Toleranz stehen Ursache und Beschluss von 🧑 (B-160/AC-02). |
| AC-03 | – | Beschlossene Wertänderungen sind in `data/` umgesetzt, `task balance` und `task check:go` … |
| AC-04 | – | Kennzahlen mit zu wenigen Sitzungen sind als „nicht aussagekräftig“ markiert (B-160/AC-04) … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-160 | verworfen | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Spielmetrik echter Abende und Simulatorwerte sind abgeglichen |
