# GS2.3 · Spiel: Schiffsfahrt im Vollbild beim Inselwechsel (B-977)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs2
- **Abhängig von:** GS2.2
- **Tickets:** B-977
- **Kriterien:** AC-04

## Ziel

Beim Inselwechsel zeigt das Spiel die Schiffsfahrt über das Diorama im Vollbild statt des bloßen Ladebildschirms.

## Kontext

Spec: `docs/backlog/B-977-seekarte-schiffsfahrt.md` (R1–R6, AC-01 bis AC-04). Der Wechsel kommt von der Engine (Ereignis `islandSwitch`, Ziel `ChosenTarget`, `engine/sim/island_targets.go`, `island_switch.go`); A im Overlay wählt nur das Ziel. Spiel: Szenen und Laden in `native/k3c-game/game/` (`scene.go`, `stack.go`, `play.go`, `loading.go`, `loadscreen.go`, Fehler `fault.go`). Diorama-Zeichnen aus GS2.2 (`native/k3c-game/hud/drawsea_paper.go`) wird für das Vollbild wiederverwendet. Die Fahrt dauert mindestens die Fahrzeit (3 s, vorläufig) und endet erst, wenn die Zielinsel geladen ist; ohne Assets bleibt der bisherige Ladebildschirm. Laden eines Spielstands und neues Spiel bleiben unverändert. Die Geschichtszeile der Ankunft (GS1, falls schon in `main`) blendet erst nach der Fahrt ein.

## Erlaubte Dateien

- `native/k3c-game/game/**`
- `native/k3c-game/hud/**`
- `native/k3c-game/view/**`

## Nicht-Ziele

Regeländerung am Inselwechsel, Ton, Fahrt beim Laden eines Spielstands, Abbrechen der Fahrt.

## Schritte

1. Tests zuerst: Szene startet auf `islandSwitch` mit Start und Ziel; Ende nach Fahrzeit und Laden (beide Reihenfolgen); Eingaben wirkungslos; Rückfall ohne Assets; Position als reine Funktion der Zeit.
2. Fahrt-Szene in `game/` (neue Datei `voyage.go`), eingehängt an der Stelle des Ladebildschirms beim Inselwechsel.
3. Zeichnen im Vollbild: Diorama, Schiff auf Bogen, bewegte vordere Welle, Fahne auf der verlassenen Insel.
4. Fachprüfung `k3c-hud`, `check_run task:check:diff`.

## Fertig, wenn

- [ ] AC-04: Fahrt im Vollbild beim Inselwechsel, Ende nach Fahrzeit und Laden, keine Eingabe, Rückfall (B-977/AC-01 bis AC-04).
- [ ] B-977/AC-05 (TV) bleibt offen bei 🧑.

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
