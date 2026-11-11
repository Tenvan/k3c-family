# B-817 · Abnahme-Läufe NAT

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Zum Abschluss des Projekts NAT (Welle 3, NG9 und NG11) laufen die Gen- und Race-Läufe gesammelt (Regel B-742, `docs/arbeitsweise.md` › Sim, Balance und Gen). Offene Abnahme-Tickets einzelner NG-Sprints gibt es nicht (geprüft mit `plan_list`; B-774 gehört zu LG1). `task check:all` lief in NG10 nicht.

## Ziel

Die gesammelten Läufe für NAT sind einmal grün gelaufen und belegt.

## Beteiligte und Zielgruppen

Claude arbeitet autonom ab (gedacht für nachts), Ralf liest das Ergebnis.

## Anforderungen

- Läufe nur über `check_run`, Ergebnisse mit Messwerten in den Notizen.
- Rot heißt neues Ticket (Typ Problem) im Projekt NAT, kein Nachbessern hier.

## Nicht-Ziele

Sim, Balance und Lasttest (gesperrt bis auf Widerruf, Beschluss 2026-11-11).

## Regeln und Einschränkungen

B-742: Der Sprint wartet nicht auf dieses Ticket.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** –
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`check_run task:test` grün → AC-03 erfüllt, Dauer in den Notizen.

## Ausnahme- und Fehlerfälle

Ohne C-Compiler läuft `check:race` still durch: im Ergebnis als übersprungen nennen.

## Akzeptanzkriterien

- **AC-01** Sim: verschoben (Sperre 2026-11-11).
- **AC-02** Balance: verschoben (Sperre 2026-11-11).
- **AC-03** Gen: `check_run task:test` (ruft `gen:paper` auf, Grafik-Golden), grün.
- **AC-04** Race: `check_run task:check:race`, grün (ohne C-Compiler übersprungen, im Ergebnis so nennen).
- **AC-05** `task check:all` einmal grün (wurde in NG10 nicht gefahren).

## Offene Fragen

keine

## Notizen

Angelegt vom Review Welle 3 (2026-11-11), Projekt NAT.
