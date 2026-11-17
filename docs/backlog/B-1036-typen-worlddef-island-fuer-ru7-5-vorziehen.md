# B-1036 · Typen WorldDef und Island für RU7.5 vorziehen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU10
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU8.2 hat `Island`, `WorldDef` und das BossData-Verhalten nicht portiert (Abweichung 5, `World` hat kein Feld `island`); sie sind für RU10 vorgesehen. RU7.5 (view Teil 2, Frame-Vertrag) braucht die Typen bereits fuer die Ansicht, die Tests `command_sim_clamps` und `crown_states_match_protocol` in `rust/crates/k3c-game` warten auf die Sim-Typen.

## Ziel

`WorldDef` und `Island` stehen als reine Datentypen (ohne Verhalten) in `k3c-sim`, bevor RU7.5 sie braucht, sodass RU7.5 nicht auf RU10 wartet.

## Beteiligte und Zielgruppen

Entwickler (Claude) der Rust-Umstellung; keine Spieler-Wirkung.

## Anforderungen

- Felder und serde-Namen wie Go (`engine/sim`), JSON-Rundlauf gegen die Golden.
- Kein HashMap, Rc, RefCell, unsafe, keine Lebenszeit-Parameter.
- Verhalten (Takt, Material-Vorrat) bleibt RU10.

## Nicht-Ziele

Insel-Takt, Stufenwechsel, Material-Vorrat (RU10).

## Regeln und Einschränkungen

Port-Leitfaden, `docs/standards/rust-sim-zustand.md`.

## Beispiele

nicht relevant: reine Typ-Vorarbeit.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Typ-Vorarbeit.

## Akzeptanzkriterien

- **AC-01** `cargo test -p k3c-sim` zeigt einen Rundlauf `WorldDef` und `Island` gegen ein Golden.
- **AC-02** RU7.5 nutzt die Typen ohne eigene Kopie.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
