# NG1.6 · Review Fundament und Glossar

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng1/6-review
- **Abhängig von:** NG1.5
- **Tickets:** B-792, B-793, B-794
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07, AC-08, AC-09, AC-10, AC-11

## Ziel

Sprint NG1 ist abgenommen, der Vertrag `view` gilt als eingefroren, die neuen Begriffe stehen im Glossar und der PR des Sprints ist offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer`. Schwerpunkte: Vertrag vollständig für NG2–NG8 (Eingabe, Screen, Session, Ereignisse), Importregel, Nebenläufigkeit (Simulation und Zeichnen in einer Goroutine), Spielstände, keine Änderung unter `engine/`. Glossar ist Sammeldatei: im letzten Commit nach `git merge origin/develop` eintragen: „Natives Spiel“, „Frame“ (nativ, nicht Snapshot), „Zelle“, „Zeichner“. Abnahme-Ticket des Sprints nach B-742 anlegen.

## Erlaubte Dateien

- `native/k3c-game/**`
- `tools/checkdiff/**`
- `.github/workflows/ci.yml`
- `native/Taskfile.yml`
- Planung NG1, B-792, B-793, B-794 (über MCP)

## Nicht-Ziele

Neue Funktionen; Umbau des Vertrags über schwere Befunde hinaus; Sim-, Balance- und Lasttest-Läufe.

## Schritte

1. `task check` und `task check:go`, dann `go test ./...` in `native/k3c-game`.
2. Diff `origin/develop...origin/sprint/ng1` lesen, nur schwere Befunde beheben.
3. Glossar im letzten Commit.
4. Abnahme in der Sprint-README, Abnahme-Ticket, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01 bis AC-09: je Kriterium Nachweis aus den Session-Ergebnissen geprüft.
- [ ] AC-10: Glossar enthält die vier Begriffe.
- [ ] PR des Sprints offen, Auto-Merge an.

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

Review (Agent `k3c-reviewer`, Opus) mit Schwerpunkt Vertrag: 5 schwere, 3 mittlere, 3 leichte Befunde, alle behoben (Agent, Opus): Kamera-Regel `view.CameraRule` aus `world`; Grafik je Level über `view.AssetSource` und `art.Library.SetLevels` (mehrere Level gleichzeitig); `view.Settings` mit Speicherung in `local`; `Session.Learn`/`Respec` und `view.Overlay` für Menüs im Spiel; `Session.Rebind` und Zuordnungs-Regel bei Pad-Verlust; `ActionHome`; unbesetzte Geräte und Geräte-Cues im Frame; `art.M`, `art.Point`, `art.RigView`; `EventsOf` und `local` ohne Allokation je Takt.
AC-01 bis AC-09, AC-11 bestätigt aus NG1.1 bis NG1.5 und erneut geprüft: `go vet`, `go build`, `go test`, `golangci-lint` im Modul grün (0 Befunde, Modul jetzt in `lintMods`); Aufnahme-Modus bildgleich vor und nach den Änderungen (9 PNG byte-gleich). AC-04: Laden und erneutes Speichern ergibt dieselbe Datei; neu `TestLoadDeterminismLikeRoom` (120 Takte nach dem Laden).
AC-10 umgesetzt: Glossar „Natives Spiel“, „Frame (nativ)“, „Zelle“, „Zeichner“.
`task check` und `task check:go` nicht gefahren: Der Sprint ändert weder Client noch `engine/`; `check:diff` deckt Modul und `tools/checkdiff`. Offen: CI-Job lief noch nie; Fenster nie gestartet; Schrift fehlt. Neues Ticket B-803.
