# 013 · Neue Mechaniken nur noch in Rust, die Go-Engine wird eingefroren

- **Status:** Entwurf (Claude, autonom, 2026-11-19; Annahme durch Ralf offen)
- **Datum:** 2026-11-19
- **Projekt:** RSU · Tickets: B-1143 (Umplanung), B-1014 (Vorschlag 011)
- **Vorgänger:** 008 (natives Spiel in Go), 012 (Port-Orakel: gleiches Verhalten)

## Anlass

Ralf hat am 2026-11-19 entschieden, das Go-Spiel (`native/k3c-game`, `native/k3c-rig`) jetzt zu entfernen (RU58 Referenz, RU37 Löschen); die Go-Engine bleibt nur bis RU40/RU41 für Workbench-Läufe. Offene Sprints und Tickets (13 Sprints, rund 60 Tickets) nannten aber `native/k3c-game` und `engine/sim` als Zielort. Ohne Regel entstünde neue Spiellogik weiter in Go und müsste danach ein zweites Mal portiert werden.

## Entscheidung

1. **Neue Mechaniken, Regeländerungen und Fehlerbehebungen in der Spiellogik entstehen nur in Rust:** `rust/crates/k3c-sim` (Sim), `k3c-level` (Level-Generator), `k3c-room` (Raum und Spielstand-Speicher), `k3c-balance`, `k3c-data` (Daten). Das Spiel ist `rust/crates/k3c-game`; es zeichnet nur.
2. **Die Go-Engine ist eingefroren:** `engine/**` und `cmd/k3c-sim` bekommen keine neue Mechanik und keine Fehlerbehebung; erlaubt sind nur Änderungen für den Abbau (RU40, RU41, RU59) und für die Workbench-Läufe bis dahin.
3. **Orakel (Ergänzung zu 012):** Bei Abweichung zwischen Rust und Go gilt Go nur für **bereits portierte** Regeln (Stand Port RU10 bis RU22, Tag `go-referenz-final`). Für neue Mechaniken gibt es kein Go-Orakel; ihre Tests liegen in Rust. Der Golden-Abgleich (`rust/crates/k3c-sim/tests/golden_*.rs`) bekommt für jede neue Mechanik, die Go nicht kennt, einen benannten Ausnahmeeintrag, bis RU41 die Go-Golden durch Rust-Golden ersetzt (ungeprüft: Umfang der Ausnahmen).
4. **Offene Sprints und Tickets** nennen als Bereich nur noch Rust-Pfade. Die Zuordnung (Port-Leitfaden § 2) lautet: `native/k3c-game/<x>/` zu `rust/crates/k3c-game/src/<x>/`, `native/k3c-game/e2e/` zu `rust/crates/k3c-game/tests/`, `native/k3c-rig/` zu `rust/crates/k3c-rig` und `public/rig/`, `engine/sim/` zu `rust/crates/k3c-sim/src/`, `engine/level/` zu `k3c-level`, `engine/room/` und `engine/store/` zu `k3c-room`, `engine/balance/` zu `k3c-balance`.
5. **Go-Vorlagen** zu Spiel und Engine sind nur noch im Tag `go-referenz-final` lesbar (`git show go-referenz-final:<pfad>`).

## Folgen

- B-1143 hängt alle offenen Feature-Sprints und Tickets um (RU60 Parität, RU61 Nachträge, RU62 Welt-Features, GS3, GS4, WE6, WE8).
- Hinweis für RU37.3 (Doku): `CLAUDE.md` (Regel „Neue Mechanik: Logik + Test in `engine/sim/`“, Abschnitt Spielstruktur, Befehle), `codemap.md` und `docs/arbeitsweise.md` (Sim-Code im Worktree B-925, `task go:golden`) auf diese Entscheidung zeigen lassen; diese Datei ändert `CLAUDE.md` selbst nicht.
- Entscheidung 011 (Vorschlag in B-1014) bleibt unberührt und wird mit RU41 angenommen oder eingetragen.
