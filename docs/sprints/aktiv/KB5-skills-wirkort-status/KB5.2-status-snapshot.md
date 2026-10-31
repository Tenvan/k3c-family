# KB5.2 · Snapshot trägt Kontrolleffekte, Schild, Auren und Stürme (B-675)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb5/2-status-snapshot
- **Abhängig von:** KB5.1
- **Tickets:** B-675
- **Kriterien:** AC-02

## Ziel

Der Snapshot trägt laufende Kontrolleffekte der Gegner, Schild und Buffs der Spieler, Auren und Stürme mit Restdauer.

## Kontext

Spec: `docs/backlog/B-675-status-effekte-sichtbar.md`. Simulation hat die Werte schon (`Enemy.Slow/SlowFor`, Stun, Taunt, `Player.Shield/ShieldFor`, `w.Storms`, Auren in `engine/sim/skills*.go`). Snapshot und Protokoll: `engine/net`, `docs/protocol.md`, `src/model/types.ts` (heute nur `stealthFor`).

## Erlaubte Dateien

- `engine/sim/**`, `engine/net/**`
- `docs/protocol.md`, `testdata/golden/**` (nur mit `task go:golden`)
- `src/model/**`, `src/online/clientProtocol.ts`

## Nicht-Ziele

- Darstellung im Client (KB5.3).

## Schritte

1. Felder im Snapshot ergänzen (nur vorhandene Werte), Go-Test.
2. Protokoll, Version, Golden.
3. Typen im Client.
4. `task check:diff`, Review (Protokoll).

## Fertig, wenn

- [x] B-675 AC-01: Go-Test Snapshot mit Kontrolleffekten, Schild und Stürmen.
- [x] B-675 AC-02: Protokoll und Golden nachgezogen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- B-675 AC-01 geprüft: `TestStatusEffekteImZustand` und `TestStatusAurenNurLebend` (`engine/net/status_snapshot_test.go`): Zustand mit `enemies[].stun/slow/slowFor/tauntId/tauntFor`, `players[].shield/shieldFor/lastStandFor/auras` und `storms` mit Restdauer, Schlüssel wie im Beispiel; zweiter Spieler ohne Effekte ohne Felder.
- Befund: Kontrolleffekte, Schild, Last Stand und Stürme standen schon im Zustand (JSON der Welt), nur undokumentiert und ohne Client-Typ. Neu ist allein `players[].auras` (`sim.AurasOf`, `engine/sim/skills_status.go`, nur lesen): Auren, guardian, regeneration, holyGround (`castle`), frostArmor (Radius 0); Werte-Passive ohne Wirkort fehlen, tote Monarchen ohne.
- B-675 AC-02 geprüft: Protokoll v11 (`docs/protocol.md` › Status-Effekte, Beispiel `s2c-snapshot-status.json`), Client-Typen `Player`, `Enemy`, `Aura`, `Storm`, `World.storms` (`src/model/types.ts`). `task go:golden` ohne Änderung (`auras` entsteht erst im Protokoll, nicht im Sim-JSON).
- Abweichung: Version 11 braucht wie in KB5.1 `src/online/clientSkills.test.ts` und `testdata/protocol/c2s-hello.json`, `s2c-welcome.json`.
- Prüfung: go test sim/net/room grün, `golangci-lint` engine 0 Befunde, `tsc` grün, Vitest `src/online`, `src/model`, `src/scenes`, `tests/planning` grün; `task check:diff` rot nur durch fremde Studio-Chat-Änderungen (worker-2) und `tools/cleanup` (Git-sh).