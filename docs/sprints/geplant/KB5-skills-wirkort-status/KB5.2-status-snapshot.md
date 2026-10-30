# KB5.2 · Snapshot trägt Kontrolleffekte, Schild, Auren und Stürme (B-675)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb5/2-status-snapshot
- **Abhängig von:** KB5.1
- **Tickets:** B-675
- **Kriterien:** B-675 AC-01, AC-02

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

- [ ] B-675 AC-01: Go-Test Snapshot mit Kontrolleffekten, Schild und Stürmen.
- [ ] B-675 AC-02: Protokoll und Golden nachgezogen.

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
