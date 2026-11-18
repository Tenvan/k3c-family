# B-1080 · Abnahme RU6: Level-Generator in Rust (Gen, Race, manuell)

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU6
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU6 portiert `engine/level` nach `rust/crates/k3c-level`. Sim, Balance, Gen und `check:race` liefen im Sprint nicht (B-742); der Level-Vergleich lief nur als `cargo test` gegen die Golden-Dateien.

## Ziel

Die Abnahme nach dem Sprint: Gen-Lauf (`gen:paper` samt Grafik-Golden), `check:race` auf einem Rechner mit C-Compiler und ein Level-Vergleich Go gegen Rust über mehr Seeds, soweit der Beschluss zu Simulationen (gesperrt bis auf Widerruf) das wieder erlaubt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `task check:race` und `task gen:paper` einmal grün.
- Level-Vergleich Go gegen Rust über viele Seeds je Biom (`level_generate`), sobald Simulationen freigegeben sind.

## Nicht-Ziele

Neue Level-Regeln.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-level/**`
- **Abhängig von:** RU6
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task check:race` und `task gen:paper` laufen grün.
- **AC-02** Level-Vergleich Go gegen Rust über mindestens 1000 Seeds je Biom ohne Abweichung (oder Befund als Ticket).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
