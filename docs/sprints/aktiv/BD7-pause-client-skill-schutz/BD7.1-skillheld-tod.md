# BD7.1 · Simulation: gehaltener Skill wird beim Tod vergessen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** sprint/bd7
- **Abhängig von:** –
- **Tickets:** B-524
- **Kriterien:** AC-01

## Ziel

Der Tod eines Monarchen setzt `Player.skillHeld` zurück: Nach dem Respawn meldet der erste neue Druck ohne Ziel wieder `castFailed`.

## Kontext

`stepSkills` (`engine/sim/skills.go`) merkt den gehaltenen Slot in `Player.skillHeld` (B-378, BD2) und meldet `castFailed` nur bei neuem Druck. In `stepPlayers` (`engine/sim/economy.go`) führt der Tod-Zweig (`!isAlive(p)`) zu `continue` vor `stepSkills`, das Feld bleibt stehen. Testhilfen: `quietWorld`, `AddPlayer`, `eventsOfType` (`engine/sim/feedback_test.go`), Tod per `p.HP = 0` und `p.RespawnIn`.

## Erlaubte Dateien

- `engine/sim/economy.go`
- `engine/sim/feedback_test.go`

## Nicht-Ziele

Andere Laufzeit-Felder beim Tod (eigenes Ticket bei Bedarf).

## Schritte

1. Test zuerst in `feedback_test.go`: Skill 1 halten → ein `castFailed`; Monarch stirbt (HP 0, `RespawnIn`), während des Todes weiter halten; Respawn über `stepPlayers`; dann erst loslassen-frei neu drücken → zweites `castFailed`. Zweiter Spieler unverändert.
2. Im Tod-Zweig von `stepPlayers` `p.skillHeld = 0`.
3. Golden unverändert.

## Fertig, wenn

- [ ] AC-01: `go test ./engine/sim -run CastFailed` grün, der neue Test scheitert ohne die Änderung.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
