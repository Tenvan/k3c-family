# B-1126 · RU53 Abnahme: Skills lernen und Inselwahl im Rust-Spiel (Sim, Gen, Race, Spieltest)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU53
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

**Lane: zweit** (Second Worker). Sprint RU53 (B-1106) hat Skills lernen, Respec und Zielinsel-Wahl im Rust-Spiel angebunden und schließt auf dem Hauptrechner ohne Sim-, Gen- und Race-Läufe ab (B-742).

## Ziel

Die gesperrten Prüfläufe und der Spieltest zu RU53 sind gelaufen und belegt.

## Beteiligte und Zielgruppen

Second Worker (Läufe), 🧑 (Spieltest mit zwei Pads am PC).

## Anforderungen

Keine Änderung an Regeln, Goldens oder `flow.json`; Befunde als Tickets.

## Nicht-Ziele

Neue Funktionen, Balancing-Änderungen.

## Regeln und Einschränkungen

Nur auf dem Second Worker (Sim/Race/Gen erlaubt, Beschluss 2026-11-18).

## Beispiele

Spieler 2 holt einen Skill-Punkt, lernt im Menü einen Skill; Respec am Tag an der Burg; nach Endboss-Sieg Zielinsel an der Seekarte wählen.

## Ausnahme- und Fehlerfälle

Abweichung gegen Go: Go gilt, als Ticket melden.

## Akzeptanzkriterien

- **AC-01** `task check:race` grün (Rust-Tests `cargo test --workspace` und Go).
- **AC-02** `cargo test -p k3c-game --test e2e_flow` grün ohne Ausnahmeliste, `testdata/` unverändert.
- **AC-03** Manuell (🧑): Skill-Menü zu zweit: nur der wählende Spieler lernt; Respec-Eintrag nur am Tag an der Burg; Seekarte nach Endboss-Sieg lässt die Zielinsel wählen, beide spielen auf der neuen Insel weiter.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Soft-Lock nach Inselsieg wurde in RU53.3 nicht gefunden (Test `choose_island_wechselt_und_spiel_laeuft_weiter`).
