# WE8.1 · Regeltext: Überfall, Sandsturm und Nebel in bosse.md (B-991, B-992)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we8
- **Abhängig von:** –
- **Tickets:** B-991, B-992
- **Kriterien:** AC-01

## Ziel

`docs/rules/bosse.md` § 2 beschreibt die Events Überfall, Sandsturm und Nebel mit Regel, Begründung und Daten-Feldern (B-991, B-992).

## Kontext

Regelwerk Events: `docs/rules/bosse.md` § 2 (Tabelle Regel, Begründung, Daten, Zielkorridor), Daten `data/events.json` (heute `fullMoon`, `bloodMoon`, `merchantRaid`). Specs: B-991 (Event `raid`: `everyNights`, `kind`, `share`, `speedFactor`, `rewardGold`), B-992 (`sandstorm`: `speedFactor`, `rangeFactor`; `fog`: `rangeFactor`; nur Nacht, nur Tiefe 0). Wetter-Rahmen: B-969 (Sprint SI1); weicht er ab, gilt B-969 und B-992 bekommt eine Revision. Events sind in der Basis aus (`everyNights: 0`) und werden je Welt über `overrides › events` eingeschaltet (B-982, `docs/rules/welten.md` § 1.1). Begriffe zuerst ins Glossar (`docs/glossar.md`, über den letzten Commit des Sprints). Zahlen sind vorläufig, Balancing später.

## Erlaubte Dateien

- `docs/rules/bosse.md`

## Nicht-Ziele

Code, `data/events.json` (WE8.2, WE8.3), Zielkorridore festlegen (bleiben offen, Balancing gesperrt).

## Schritte

1. B-969 lesen (Stand, Wetter-Rahmen); Abweichungen zu B-992 im Ergebnis nennen.
2. Drei Zeilen in `bosse.md` § 2 nach dem Muster der vorhandenen Events, § 3 (Offen und Annahmen) um die vorläufigen Werte ergänzen.
3. Begriffe Überfall, Sandsturm, Nebel für das Glossar im Ergebnis notieren.

## Fertig, wenn

- [ ] AC-01: `bosse.md` § 2 nennt je Event Regel, Begründung und Daten-Felder; keine Platzhalter.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 geprüft: `docs/rules/bosse.md` § 2 nennt Überfall (B-991), Sandsturm und Nebel (B-992) mit Auslöser, Wirkung, Belohnung und den Daten-Feldern (`raid.*`, `sandstorm.*`, `fog.*`); § 2 Begründung und § 3 Annahmen ergänzt. Keine Platzhalter.

B-969 gelesen: Wetter-Rahmen (Vorboten, Dauer eine Nacht, eigenes System `data/weather.json`) passt zu Sandsturm und Nebel. Abweichung: keine, B-992 bleibt gültig; Wetter selbst ist nicht Teil der Events.

Begriffe im Glossar eingetragen: Überfall, Sandsturm, Nebel (alphabetisch, `tests/planning.test.ts` grün).

Zielkorridore und Zahlen bleiben offen (Balancing gesperrt), siehe Sammelticket B-1259, Frage 8.

Manuell geprüft: niemand. Keine Code-Änderung.
