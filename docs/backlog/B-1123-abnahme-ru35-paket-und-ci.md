# B-1123 · Abnahme RU35: Rust-Paket, CI-Lauf und schwere Läufe auf dem Second Worker

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU64
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU35 stellt `task build`/`buildall` und die CI auf das Rust-Spiel um. Auf dem Hauptrechner sind Sim-, Balance-, Gen-Läufe und `check:race` gesperrt (Beschluss 2026-11-11); sie laufen hier. Lane: zweit.

## Ziel

Der Sprint RU35 ist ohne gesperrte Läufe abgenommen: Sim, Balance, Gen und Race grün, Paket und CI-Lauf bestätigt.

## Beteiligte und Zielgruppen

Second Worker führt aus, 🧑 Ralf nimmt das Paket auf dem Zielgerät ab.

## Anforderungen

- Läufe wie `docs/arbeitsweise.md` › Sim, Balance und Gen; keine Regel- oder Wertänderung.

## Nicht-Ziele

Neue Funktionen; Lizenzdatei und Pfad-Nachzug (B-1121, B-1122).

## Regeln und Einschränkungen

Abnahme-Ticket nach B-742.

- **Bereich:** –
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Abnahme-Ticket.

## Ausnahme- und Fehlerfälle

nicht relevant: rote Läufe werden eigene Tickets.

## Akzeptanzkriterien

- **AC-01** Sim: `sim_test` ohne Befund (nur wenn 🧑 die Sperre aufgehoben hat, `K3C_SIM_ENTSPERRT=1`; sonst mit Grund als nicht gefahren vermerkt).
- **AC-02** Balance: Balancing-Lauf ohne Befund (gleiche Einschränkung wie AC-01).
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Golden grün.
- **AC-04** Race: `check_run task:check:race` grün.
- **AC-05** `task buildall` auf sauberem Stand legt `dist/k3c-game/` (Rust-EXE, data, Grafik) an; Rust-EXE startet. Das Go-Spiel baut seit RU58 nicht mehr: `dist/k3c-game-go/` entfällt. Der CI-Job `rust` des Branches (Paketprüfung EXE ≤ 40 MB) ist grün.
- **AC-06** 🧑 Ralf: `dist/k3c-game/k3c-game.exe` startet am Zielgerät mit Grafik und Ton (B-1141).

## Offene Fragen

keine

## Notizen

Lane: zweit (Second Worker). Hauptrechner hat Build, Start beider Pakete und den CI-Lauf geprüft, keine Sim-/Balance-/Gen-/Race-Läufe.
**Go-Referenz (RU58, 2026-11-19):** Das Go-Spiel baut nicht mehr; Go-Vergleichswerte liegen in `docs/messung/go-referenz.json`, Go-Bilder in `testdata/rust/e2e/go/`, Bauquelle ist der Tag `go-referenz-final`. Umgesetzt in Sprint RU64 (Session RU64.2), Ergebnis `docs/messung/abnahme-b-1123.md`; AC-06 bleibt bei Ralf (B-1141).
