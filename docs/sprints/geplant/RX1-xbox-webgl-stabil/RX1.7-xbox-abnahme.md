# RX1.7 · Xbox-Abnahme ohne Kontextverlust

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** rx1/7-xbox-abnahme
- **Abhängig von:** RX1.6
- **Tickets:** B-763, B-764
- **Kriterien:** AC-09, AC-10

## Ziel

🧑 bestätigt am TV: kein Kontextverlust, Spiel bedienbar, FPS-Wert notiert (B-763/AC-03, B-764/AC-03).

## Kontext

Voraussetzung: RX1.1 bis RX1.6 auf `develop` und ausgeliefert, PNG per `task gen:paper` neu erzeugt. Claude liest anschließend das Pi-Log.

## Erlaubte Dateien

- Keine Codeänderung; Ergebnis in dieser Datei und in den Tickets.

## Nicht-Ziele

Keine Leistungsoptimierung; Split-Screen-Leistung ist PF1.

## Schritte

1. 🧑: Xbox neu starten, Spiel mindestens 2 Minuten spielen, FPS-Wert notieren.
2. Claude: Pi-Log auf „WebGL Context lost“ prüfen, Texturspeicher und FPS festhalten.
3. Bei Fehlschlag: neues Ticket mit den Log-Werten.

## Fertig, wenn

- [ ] AC-09: Spiel bleibt nach einem Kontextverlust bedienbar, keine weiße Seite (oder es gab keinen Verlust).
- [ ] AC-10: Kein „WebGL Context lost“ im Server-Log, FPS-Wert notiert.

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

–
