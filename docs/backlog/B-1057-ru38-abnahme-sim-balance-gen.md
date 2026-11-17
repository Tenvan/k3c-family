# B-1057 · RU38 Abnahme: Fenster-Messung der Wiedergabe, Sim, Balance und Gen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU38
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU38 (Aufzeichnung aus Go, Wiedergabe `k3c-game replay`) ist per Test abgenommen, aber die Sicht im Fenster und die gesperrten Läufe stehen aus (Sim-Sperre, B-742). Offene Punkte aus dem Review: `replay/pending.rs` zeichnet Rechtecke je Einheit, bis RU13 vorliegt; `replay/blend.rs` gleitet nur Stufe 0 (Aufzeichnung hat eine Welt je Takt); der Prüfwert hängt von `--measure` ab (nur bei gleicher Optionsmenge vergleichbar).

## Ziel

RU38 ist am Rechner und im Fenster abgenommen; Sim-, Balance- und Gen-Läufe des Sprints sind gelaufen oder als nicht betroffen genannt.

## Beteiligte und Zielgruppen

🧑 Ralf prüft das Fenster; Claude fährt die gesperrten Läufe nach Aufhebung der Sperre.

## Anforderungen

- 🧑: `cargo run --release -p k3c-game -- replay testdata/rust/trace/nacht-300.jsonl.gz --window --measure 300` im Ordner `rust` starten; Nacht mit Gegnern in zwei Bildhälften, Glätte der Zwischenbilder und Kamera ansehen, Bericht unter `reports/` prüfen.
- `task check:race` (lokal ohne gcc übersprungen; CI nennen).
- Go-Test `go test ./engine/internal/trace/ -write` einmal fahren, `.work/trace/` bleibt ungeprüft eingecheckt-frei; keine Golden-Änderung.
- Sim, Balance, Gen: von RU38 nicht berührt (nur neues Paket `engine/internal/trace`), kein Lauf nötig.
- Zur Kenntnis: `flate2 = "=1.1.11"` in `rust/crates/k3c-game/Cargo.toml` (gleiche Version wie `k3c-det`, kein neuer Crate im Lockfile, außerhalb der erlaubten Dateien von RU38.2); Blend nur Stufe 0 und Rechtecke aus `pending.rs` bis RU13.

## Nicht-Ziele

Neue Funktionen der Wiedergabe.

## Regeln und Einschränkungen

Sim-Sperre vom 2026-11-11, Golden nie neu schreiben.

- **Bereich:** `rust/crates/k3c-game/src/replay/**`
- **Abhängig von:** RU38
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Abnahme-Ticket.

## Ausnahme- und Fehlerfälle

nicht relevant: Abnahme-Ticket.

## Akzeptanzkriterien

- **AC-01** Der Fenster-Lauf zeigt die Nacht in zwei Bildhälften ohne sichtbares Ruckeln; der Bericht enthält alle Felder des Go-Berichts.
- **AC-02** `task check:race` (CI) grün; `testdata/golden` unverändert.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
