# B-1023 · Go-math.Exp rechnet auf CPUs ohne AVX/FMA andere Bits als die Golden

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU22
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Gefunden in RU3.2. Go rechnet `math.Exp` auf amd64 im Assembler (`$(go env GOROOT)/src/math/exp_amd64.s`) und wählt zur Laufzeit: mit AVX und FMA (`math.useFMA`) die Befehle `VFNMADD231SD`/`VFMADD213SD`, sonst einen SSE-Pfad. Gemessen auf 100 000 Eingaben in [−50, 50]: Der SSE-Pfad weicht bei 9 307 (≈ 9 %) in der letzten Stelle ab. `math.Pow` mit gebrochenem Exponenten ruft `Exp` und erbt das. Die Sim ruft `math.Exp` in jedem Schritt (`engine/sim/economy.go:61`). Folge: Das Go-Spiel und `go test` mit Golden liefern auf CPUs ohne AVX/FMA (z. B. Mini-PCs mit Celeron/Pentium Silver) andere Bits als die Golden. `GODEBUG=cpu.fma=off` wirkt unter Windows nicht; der SSE-Pfad lässt sich hier nicht mit Go nachstellen.

Rust (`k3c_det::exp`, RU3.2) rechnet immer den FMA-Pfad über `mul_add` (einmal gerundet, auf jeder CPU gleich) und ist damit bitgleich zu Go auf Rechnern mit FMA, also zu allen Golden.

## Ziel

🧑 entscheidet, ob die Rust-Sim auf jeder CPU den FMA-Pfad rechnet (Stand RU3.2, deterministisch über Rechner hinweg) oder Gos Laufzeit-Wahl nachbaut.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Entscheidung festhalten (Entscheidung 011 oder Notiz in `docs/standards/rust-port.md` § 5).
- Bleibt es beim FMA-Pfad: im Abgleich RU22 vermerken, dass Golden nur auf Rechnern mit AVX/FMA geschrieben werden (`engine/internal/detvec` prüft das mit einem Probe-Wert).

## Nicht-Ziele

Go-Code ändern (Go-Stopp).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Entscheidung steht in `docs/standards/rust-port.md` § 5 oder einer Entscheidung unter `docs/decisions/`; `k3c_det::exp` folgt ihr und `cargo test -p k3c-det` ist grün.

## Offene Fragen

- Soll Rust auf jeder CPU den FMA-Pfad rechnen? Default (Stand RU3.2): ja, weil die Golden so entstanden sind und das Spiel damit über Rechner hinweg gleich rechnet.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
