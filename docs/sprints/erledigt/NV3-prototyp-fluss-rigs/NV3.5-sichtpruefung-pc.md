# NV3.5 · Sichtprüfung am PC und Entscheidung

- **Status:** fertig
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** nv3/5-sichtpruefung-pc
- **Abhängig von:** NV3.4
- **Tickets:** B-789
- **Kriterien:** AC-05

## Ziel

🧑 prüft den ausgebauten Prototyp am PC mit dem Auge und entscheidet, ob dieser Aufbau die Grundlage des nativen Spiels wird.

## Kontext

`task native:ebiten` am Entwicklungs-PC, zwei Gamepads. Zu beurteilen: Fluss (Aufteilung, Fließen, drei Bahnen), stehende Schicht `fern`, Rig-Figuren gegen Sheets.

## Erlaubte Dateien

- Planung B-789, NV3 (über MCP)

## Nicht-Ziele

Code ändern; Befunde werden Tickets.

## Schritte

1. Prototyp starten, mit zwei Gamepads durch das Level reiten.
2. Entscheidung und Befunde in B-789 › Notizen.

## Fertig, wenn

- [ ] AC-05: Sichtbefund und Entscheidung von 🧑 in B-789 › Notizen.

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

AC-05 geprüft: Sichtprüfung am PC durch 🧑 Ralf am 2026-11-11, bestanden („sieht super aus“, Chat). Der Aufbau (Fluss im unteren Drittel, feste Schicht `fern`, Figuren als Rig) ist die Grundlage für das native Spiel (NG1). Keine Befunde.
