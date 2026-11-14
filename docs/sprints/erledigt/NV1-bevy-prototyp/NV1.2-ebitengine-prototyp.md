# NV1.2 · Ebitengine-Prototyp bauen und gegen Bevy messen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** live
- **Branch:** sprint/nv1
- **Abhängig von:** NV1.1
- **Tickets:** B-777
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

`native/k3c-ebiten/` (Go, Ebitengine) zeigt dasselbe Level wie `native/k3c-proto`; `task native:measure` misst beide Engines in einem Bericht.

## Kontext

Vorlage ist der Bevy-Prototyp unter `native/k3c-proto/src/` (Level, Maße, Parallaxe, Kommandozeile, Berichtsfelder). Sheets und Manifeste wie in NV1.1 › Kontext. Ebitengine lädt Bilder erst beim ersten Zeichnen auf die GPU und fasst kleine Bilder in einem internen Atlas zusammen.

## Erlaubte Dateien

- `native/k3c-ebiten/**`
- `native/Taskfile.yml`, `native/measure.ps1`, `native/messen.cmd`
- `.gitignore` (nur Bau-Ausgaben)
- Planung B-777, NV1 (über MCP)

## Nicht-Ziele

Simulation, Netz, HUD; Änderungen an `src/`, `engine/`, `rust/`, `native/k3c-proto/`.

## Schritte

1. Go-Modul mit festgelegter Ebitengine-Version.
2. Manifeste und Sheets laden, Level wie in `level.rs`.
3. Split-Screen, zwei Gamepads, Tastatur.
4. Messmodus und Erweiterung des Messskripts.
5. Vergleichsmessung, Werte in B-777 › Notizen.

## Fertig, wenn

- [x] AC-01: `task native:ebiten:build` grün, `go vet` und `gofmt` sauber.
- [x] AC-02: Screenshot neben dem Bevy-Screenshot geprüft, gleiches Level im Split-Screen.
- [ ] AC-03: Echte Pads und Tastatur von Hand nicht geprüft, nur Auto-Fahrt (🧑 am Mini-PC).
- [x] AC-04: Bericht mit beiden Engines und beiden Lastfällen.
- [x] AC-05: Vergleichswerte in B-777 › Notizen.

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

`native/k3c-ebiten` (Go, Ebitengine 2.10.6, 1099 Zeilen, eigenes Modul, ohne cgo) zeigt dasselbe Level wie der Bevy-Prototyp. `task native:measure:beide` misst beide; `native:proto:paket` packt beide EXE. Entwicklungs-PC: Ebitengine 5238 / 5146 FPS (level / alle) bei 104 / 1330 MiB GPU, Bevy 399 / 364 FPS bei 224 / 996 MiB. Umgesetzt von einem Subagenten, Screenshot und Bericht nachgeprüft. `task check:diff` nicht gelaufen.
