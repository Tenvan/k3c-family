# BD3.2 · Simulation: geschützter Monarch (Protected) ohne Treffer

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** sprint/bd3
- **Abhängig von:** –
- **Tickets:** B-214
- **Kriterien:** AC-02

## Ziel

Die Simulation kennt einen geschützten Monarchen (`sim.Player.Protected`, nur zur Laufzeit vom Raum gesetzt): Gegner wählen ihn nicht als Ziel, Schaden trifft ihn nicht.

## Kontext

Schaden an Monarchen läuft über `damagePlayer` (`engine/sim/common.go`), Zielwahl der Gegner über `candidates` (`engine/sim/enemies.go`), Flächenschaden über `aoeTargets` (`engine/sim/enemies_traits.go`), Boss-Fähigkeiten überspringen schon `Free` (`boss_abilities.go`). Das Feld ist wie `Free` Sache des Raums, aber `json:"-"`: kein Spielstand-Feld, kein Protokollfeld. BD3.3 setzt es für pausierte Monarchen online höchstens 60 s.

## Erlaubte Dateien

- `engine/sim/types.go`, `engine/sim/common.go`, `engine/sim/enemies.go`, `engine/sim/enemies_traits.go`, `engine/sim/boss_abilities.go`
- `engine/sim/player_protected_test.go` (neu)

## Nicht-Ziele

Schutz wartender Monarchen nach Verbindungsverlust (eigenes Ticket); Spielstand und Protokoll.

## Schritte

1. Test zuerst: geschützter Monarch neben Gegner verliert keine HP, kein `hit`; ungeschützt verliert er HP.
2. Feld `Protected` in `Player`, `damagePlayer` kehrt sofort zurück, `candidates` und `aoeTargets` überspringen ihn.
3. Golden unverändert (Feld ist im Normalbetrieb false).

## Fertig, wenn

- [ ] AC-02 (Teil Simulation): `go test ./engine/sim -run Protected` grün.
- [ ] `task check:diff` grün, Golden unverändert.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-02 (Teil Simulation) umgesetzt: `sim.Player.Protected` (`json:"-"`), `damagePlayer` ignoriert ihn, `candidates` und `aoeTargets` wählen ihn nicht; geprüft: `TestProtectedOhneTreffer` (Nahkampf, Geschoss, Gegenprobe ohne Schutz), `TestProtectedKeinZiel`.
- Review (Sonnet): kein schwerer Befund; alle HP-Verluste laufen über `damagePlayer`. Gegner laufen weiter auf ihn zu (`nearestThreat`), ohne Schaden. Golden unverändert.
