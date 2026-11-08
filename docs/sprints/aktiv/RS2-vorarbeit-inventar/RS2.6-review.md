# RS2.6 · Review RS2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/rs2
- **Abhängig von:** RS2.1, RS2.2, RS2.3, RS2.4
- **Tickets:** B-736, B-726, B-731
- **Kriterien:** alle

## Ziel

Sprint RS2 wird abgenommen: alle Kriterien geprüft, Befunde behoben, Folgesprint RS3 (Orakel und Skelett) nach Ralfs Entscheidung zu G1 bereit gemacht oder Projekt RST pausiert.

## Kontext

Review-Session nach `docs/arbeitsweise.md` › Review-Session. Hardware-Session RS2.5 ist keine Abhängigkeit (entkoppelt): Das Review führt AC-05 als `angenommen, Validierung offen (RS2.5)`, der Sprint bleibt `aktiv`, bis RS2.5 fertig ist. Zu prüfen: B-726, B-731, 007-Entwurf, Zahlen in 006.

## Erlaubte Dateien

- `docs/**`
- Behebungen kleiner Befunde (< ~30 Zeilen) in den vom Sprint geänderten Dateien

## Nicht-Ziele

Kein Rust-Code, keine Freigabe von Folgesprints (Ralf).

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander (race lokal ohne gcc: als übersprungen nennen).
2. Je Sprint-Kriterium Nachweis aus den Session-Ergebnissen prüfen.
3. Unabhängiger Reviewer-Agent über den Sprint-Diff (SRV/DEV, Protokollnahe).
4. Abnahme-Abschnitt der Sprint-README füllen, PR nach `develop` öffnen (erst nach vollständigem Push).
5. RS3 (Orakel und Skelett) aus B-736 als Sprint anlegen und bereit machen, wenn Ralf an G1 »weiter« entschieden hat.

## Fertig, wenn

- [ ] Alle Sprint-Kriterien (AC-01 bis AC-05) mit Nachweis; AC-05 ggf. `angenommen, Validierung offen (RS2.5)`.
- [ ] CI-Voraussetzung: `task check`, `task check:go` grün.

## Prüfen

```bash
task check
task check:go
task check:race
```

Nacheinander (Review-Session).

## Ergebnis

2026-11-08, autonom (Claude). `check_run task:check` exit 0 (5 min 41 s), danach `task:check:go` exit 0 (3 min 46 s); `check:race`, Sim, Balance, Gen nach B-742 im Abnahme-Ticket B-745.

- **AC-01 geprüft:** `go list -deps ./tools/k3c-workbench/...` → `engine/sim|room|net|level` nur über `internal/studioview` (erlaubte Ausnahme) und das Hauptpaket, das es einbindet; depguard `workbench-ohne-engine` in `.golangci.yml`. Gleiche Ergebnisse und Dauer −8 % (Meilenstein 1 Seed) laut RS2.2; Volllauf offen in B-745/AC-02.
- **AC-02 geprüft:** `BenchmarkTickDevices4` 4,0–4,2×, allocs 21,8 % (006 › RS2.3); online perf Tick p99 2,5 ms (run-28).
- **AC-03 geprüft:** `TestStateGleichOrakel*`, `TestStateReflection*` in `check:go` grün; Golden und `docs/protocol.md` im Diff unverändert.
- **AC-04 geprüft:** `docs/decisions/007-rust-server.md` Status Entwurf (nicht freigegeben), Inventar Anhang A–G, Crate-Tabelle.
- **AC-05 angenommen, Validierung offen (RS2.5):** Pi-Messung und Gate G1 bei Ralf; Sprint bleibt aktiv.

Reviewer (Sonnet) über den Sprint-Diff: keine schweren Befunde; mittel als B-743 (Bot-Pool erholt sich nicht) und B-744 (Replay-Pfad aus Seed, Segment-Kopie, Version in `protocol.md`). Leicht: `state.go` 398/400 Zeilen. Abweichung: RS3 nicht angelegt (wartet auf G1).
