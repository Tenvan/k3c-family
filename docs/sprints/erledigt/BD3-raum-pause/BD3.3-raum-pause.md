# BD3.3 · Raum: Pause im Couch-Raum und Schutz 60 s online

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** sprint/bd3
- **Abhängig von:** BD3.1, BD3.2
- **Tickets:** B-214
- **Kriterien:** AC-01, AC-02

## Ziel

Der Raum wirkt die Pause aus: Ist das pausierte Gerät das einzige verbundene (Couch-Raum), rechnet der Tick keinen Schritt; sonst stehen seine Monarchen still und sind höchstens 60 s (1800 Ticks) geschützt.

## Kontext

`engine/room/actions.go` (`Tick`, `Drop`), `engine/room/dev.go` (`steps`, Dev-Pause `r.paused`), `engine/room/pause.go` aus BD3.1. Frist in Ticks (deterministisch), `WaitFor`/`TickHz` in `engine/room/room.go`. Schutz über `sim.Player.Protected` aus BD3.2. Regel `docs/rules/bedienung.md` § 1 Pause.

## Erlaubte Dateien

- `engine/room/pause.go`, `engine/room/pause_test.go`, `engine/room/actions.go`, `engine/room/dev.go`, `engine/room/room.go`

## Nicht-Ziele

Client, Optionen-Szene, Verbindungsverlust (§ 3 bleibt wie er ist).

## Schritte

1. Tests zuerst: Couch-Raum mit 2 Spielern, Zustand steht während der Pause und läuft danach; zwei Geräte, A pausiert: A steht still und ist 1800 Ticks geschützt, danach nicht, B läuft weiter; zweites `pause` verlängert nicht; `Drop` beendet die Pause.
2. `steps()` 0 auch bei Couch-Pause; Eingaben pausierter Geräte auf Stillstand; Schutz-Zähler je Gerät im Tick.

## Fertig, wenn

- [ ] AC-01: `go test ./engine/room -run Pause` (Couch) grün.
- [ ] AC-02: `go test ./engine/room -run Pause` (online, 60 s) grün.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 umgesetzt: Couch-Raum (einziges verbundenes Gerät pausiert) rechnet keinen Schritt; geprüft: `TestPauseCouchRaumSteht` (2 Spieler, Zustand gleich, nach resume weiter), `TestPauseZweiSpielerAmGeraet` (online → Couch).
- AC-02 umgesetzt: online stehen die Monarchen des Geräts still und sind 1800 Ticks geschützt, danach verwundbar; geprüft: `TestPauseOnlineSchutz60s` (Tick 1800 geschützt, 1801 nicht, B läuft ungeschützt, zweites pause verlängert nicht), `TestPauseEndetBeiTrennung`; Schaden: BD3.2.
- Review (Sonnet): kein schwerer Befund; behoben: Eingaben verfallen auch bei resume, Frist steht in der Dev-Pause. Frist ab Couch → online beginnt voll (Default, B-528).
