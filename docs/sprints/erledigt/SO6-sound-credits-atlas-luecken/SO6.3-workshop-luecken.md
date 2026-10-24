# SO6.3 · Workshop: Sounds für die Lücken hören und wählen (🧑)

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** so6/3-workshop-luecken
- **Abhängig von:** –
- **Tickets:** B-492, B-558
- **Kriterien:** AC-03

## Ziel

🧑 hat für jede Lücke aus B-492 einen Sound gewählt oder die Lücke bestätigt.

## Kontext

Agent: Mensch (Geschmack). Claude hat vorentschieden und eingebaut (SO6.4, Tabelle in B-558). Anhören im Asset-Studio (Sound-Seite) oder `soundtest.html`; Änderung = anderen Kandidaten an Platz 1 ziehen, danach `task gen:soundatlas`.

## Erlaubte Dateien

- `data/assets.json` (über das Asset-Studio), `docs/assets/sounds.md`, `docs/backlog/B-558*`

## Nicht-Ziele

Neue Auslöser im Server.

## Schritte

1. Ralf hört je Zeile aus B-558 › Notizen und bestätigt oder ändert.
2. Claude zieht Katalog und Atlas nach.

## Fertig, wenn

- [x] AC-03: Jede Lücke hat einen gewählten Sound, bestätigt von 🧑 (2026-10-24).

## Prüfen

Manuell durch 🧑 (Asset-Studio, TV).

## Ergebnis

AC-03 geprüft (Ralf, 2026-10-24, PC): Die vorentschiedene Auswahl (B-558, Kenney, CC0) ist bestätigt: Münze geben (handlecoins2), Bau-Fortschritt (impactmining), Wiederbeleben (highup), Boss-Auftritt (lowthreetone), Hub-Ausbau (threetone2) ok. Keine Änderung am Katalog, kein `gen:soundatlas` nötig.
