# B-1013 · Abnahme-Läufe WE5

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WE5
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint WE5 (Welt-Format-Nachträge: Endboss je Insel, Overrides für Events und Bauplätze, Skins) ist umgesetzt und geprüft (Review WE5.7). Sim-, Balance-, Gen- und Race-Läufe gehören ins Abnahme-Ticket (B-742).

## Ziel

Die Abnahme-Läufe für WE5 sind gefahren oder begründet verschoben.

## Beteiligte und Zielgruppen

Claude (autonom), 🧑 für den Rechner mit gcc.

## Anforderungen

- Gen und Race laufen einmal am gemergten Stand.
- Sim und Balance bleiben verschoben, solange die Sperre gilt.

## Nicht-Ziele

Codeänderungen; Befunde werden eigene Tickets.

## Regeln und Einschränkungen

- **Bereich:** `reports/**`
- **Abhängig von:** WE5.7
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –
- Sim und Balancing gesperrt (Beschluss 🧑 2026-11-11).

## Beispiele

nicht relevant: reine Prüfläufe.

## Ausnahme- und Fehlerfälle

`check:race` läuft lokal ohne C-Compiler still durch → als übersprungen nennen.

## Akzeptanzkriterien

- **AC-01** Sim (`sim_test`): verschoben (Sperre 2026-11-11).
- **AC-02** Balance: verschoben (Sperre 2026-11-11).
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün.
- **AC-04** Race: `check_run task:check:race` grün auf einem Rechner mit gcc, sonst als übersprungen genannt.

## Offene Fragen

keine

## Notizen

Angelegt aus der Review-Session WE5.7.
