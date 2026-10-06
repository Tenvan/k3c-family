# BAL4.2 · Workshop: Abweichungen, Ursachen und Beschlüsse

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Umgebung:** offline
- **Branch:** bal4/2-workshop-abweichungen
- **Abhängig von:** BAL4.1
- **Tickets:** B-160
- **Kriterien:** AC-02

## Ziel

🧑 hat Toleranzen und die Definition „echter Abend“ festgelegt; zu jeder Abweichung über der Toleranz stehen Ursache und Beschluss (ändern oder belassen) in der Abgleichtabelle.

## Kontext

Ein Agent bereitet nur vor; 🧑 entscheidet. Dauer Richtwert 45 Minuten.

- **Vorbereitung durch den Agenten:** Tabelle `docs/rules/abgleich-simulator.md` aus BAL4.1 (Vorschlag des Dateinamens, dort nachsehen). Je Abweichung eine **vorgeschlagene** Ursache aus B-160 › Anforderungen (Bot zu gut oder zu schlecht, Wert falsch, Messung ungleich) und ein Vorschlag, klar als „Vorschlag“ gekennzeichnet. Beispiel aus B-160: Simulator 82 % gegen echter Abend 55 % → Ursache „Bot reagiert schneller als Kinder“ → Kind-Bot-Profil nachziehen (BAL3) oder Wert anpassen.
- **Offen und hier zu entscheiden (nicht durch Q12/Q24 beantwortet):** Toleranz je Kennzahl, Definition „echter Abend“, Mindestzahl Sitzungen je Kennzahl für „aussagekräftig“. Der Agent erfindet keine dieser Zahlen.
- Vorgaben: Werte ändert nur 🧑 per Beschluss (`docs/arbeitsweise.md` › Domänen); Beschluss Q12 (Metrik nur Geräte-Kürzel, keine Namen) gilt auch für die Tabelle.
- Beschlüsse „ändern“ nennen Datei, Pfad und neuen Wert in `data/*.json`; das setzt BAL4.3 um, nicht diese Session.

## Erlaubte Dateien

- `docs/rules/abgleich-simulator.md` (Toleranz, Ursache, Beschluss)
- `docs/sprints/` (Status dieser Session), `docs/backlog/` (Status und neue Tickets)

## Nicht-Ziele

Keine Änderung in `data/`, kein Code, keine neue Spec; offene Punkte werden Tickets.

## Schritte

1. 🧑 legt Toleranzen und die Definition „echter Abend“ fest; der Agent trägt sie in die Tabelle ein (Kopf: „Beschlossen von 🧑 am <Datum>“).
2. Je Abweichung über der Toleranz: Ursache bestätigen oder ändern, Beschluss (ändern mit Pfad und Wert, oder belassen mit Grund) eintragen.
3. Kennzahlen unter der Mindestzahl Sitzungen bleiben „nicht aussagekräftig“ und bekommen keinen Beschluss.
4. Offene Punkte als Ticket oder Offene Frage führen.

## Fertig, wenn

- [ ] AC-02: Jede Abweichung über der Toleranz hat in der Tabelle eine Ursache und einen Beschluss von 🧑 (ändern oder belassen).
- [ ] Toleranzen, Definition „echter Abend“ und Mindestzahl Sitzungen stehen mit „Beschlossen von 🧑 am <Datum>“ in der Datei.

## Prüfen

Manuell durch 🧑 (Gespräch); danach `task check`.

## Ergebnis

–
