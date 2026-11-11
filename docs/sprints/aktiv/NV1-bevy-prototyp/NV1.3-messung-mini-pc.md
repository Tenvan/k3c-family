# NV1.3 · Beide Prototypen am Mini-PC messen, echte Gamepads prüfen

- **Status:** offen
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** nv1/3-messung-mini-pc
- **Abhängig von:** NV1.2
- **Tickets:** B-776, B-777
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

🧑 misst beide Prototypen am Mini-PC und prüft zwei echte Gamepads; die Werte stehen in B-776 und B-777 › Notizen.

## Kontext

Paket: `task native:proto:paket` (beide EXE, `messen.cmd`). Die Entscheidung für Ebitengine ist schon gefallen (Entscheidung 008); die Messung bestätigt sie auf dem Zielgerät. Nachgetragen am 2026-11-11, weil AC-06 keiner Session zugeordnet war.

## Erlaubte Dateien

- Planung B-776, B-777, NV1 (über MCP)

## Nicht-Ziele

Code ändern.

## Schritte

1. Paket auf den Mini-PC kopieren, `messen.cmd` starten.
2. Zwei Gamepads und Tastatur von Hand prüfen (AC-03).
3. Bericht zurückspielen, Werte in die Notizen der Tickets.

## Fertig, wenn

- [ ] AC-03: zwei echte Gamepads steuern je einen Monarchen (beide Prototypen oder nur Ebitengine).
- [ ] AC-06: Messwerte vom Mini-PC in B-776 und B-777 › Notizen.

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
