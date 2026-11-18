# B-1097 · Platzhalter add_stock_capped in k3c-sim pending.rs ist ein Rest aus RU10

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU18
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`rust/crates/k3c-sim/src/pending.rs` hat noch den RU10-Platzhalter `add_stock_capped(w, isl, resource, amount) -> Option<i64>` (`unimplemented!("RU10: ...")`). RU10 hat die echte Funktion in `island/storage.rs` geliefert (andere Signatur: `v`, Ergebnis `(i64, bool)`). `boss/mod.rs` Zeile 173 ruft noch den Platzhalter; der Boss-Belohnungs-Pfad panikt dort.

## Ziel

Kein Platzhalter mehr: `boss/mod.rs` nutzt `island::storage::add_stock_capped`, der Platzhalter ist gelöscht.

## Beteiligte und Zielgruppen

Entwickler (Rust-Port RU22 prüft, dass kein Platzhalter übrig ist).

## Anforderungen

- Aufruf in `boss/mod.rs` auf die echte Funktion umstellen (`v` und `isl` stehen dort bereit), Platzhalter löschen, Boss-Tests bleiben grün.

## Nicht-Ziele

Weitere Platzhalter.

## Regeln und Einschränkungen

Port-Leitfaden `docs/standards/rust-port.md` § 7.

- **Bereich:** `rust/crates/k3c-sim/src/pending.rs`, `rust/crates/k3c-sim/src/boss/mod.rs`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Aufräumen eines Platzhalters.

## Ausnahme- und Fehlerfälle

nicht relevant: Verhalten bleibt das von Go.

## Akzeptanzkriterien

- **AC-01** `grep -rn "RU10: add_stock_capped" rust/` findet nichts; `cargo test -p k3c-sim boss` grün.

## Offene Fragen

keine

## Notizen

Gefunden in RU18.2.
