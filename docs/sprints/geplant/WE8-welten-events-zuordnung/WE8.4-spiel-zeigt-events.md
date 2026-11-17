# WE8.4 · Spiel: Ankündigung und Schleier für Überfall, Sandsturm, Nebel (B-995)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we8
- **Abhängig von:** WE8.3
- **Tickets:** B-995
- **Kriterien:** AC-04

## Ziel

Das Spiel kündigt Überfall, Sandsturm und Nebel an und zeigt Sandsturm und Nebel als Schleier in Tiefe 0 (B-995).

## Kontext

Natives Spiel `native/k3c-game`: HUD und Hinweise `hud/` (`hints.go`, `texts.go`), Effekte `fx/fx.go`, Ansicht je Bildhälfte `view/`. Vorbild: Ankündigung und Darstellung von Vollmond und Blutmond (Grep nach `fullMoon`, `bloodMoon`). Die Engine meldet das aktive Event im Zustand (WE8.2, WE8.3, `docs/protocol.md`). Jede Bildhälfte zeigt die Stufe ihres Spielers; Wetter gilt nur in Tiefe 0. Das Spiel zeichnet nur. Log-Emoji: 🌙 Nacht/Welle.

## Erlaubte Dateien

- `native/k3c-game/**`

## Nicht-Ziele

Ton, neue Papierschnitt-Assets, Logik, Partikelsysteme.

## Schritte

1. Tests zuerst: Zustand mit aktivem Event → Ankündigung; Schleier nur in Tiefe 0; unbekanntes Event; zwei Events.
2. Ankündigung an der Stelle der Mond-Events, Name aus den Daten.
3. Schleier als eine halbtransparente Fläche mit laufendem Muster je Bildhälfte.
4. Hinweis beim Eintreffen des Überfalls; Review `k3c-hud` bei Risiko.

## Fertig, wenn

- [ ] AC-04: B-995/AC-01, AC-02 als Tests grün.
- [ ] B-995/AC-03 (Sichtprüfung) ist an B-996 verwiesen.
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
