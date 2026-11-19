# B-1152 · Rig-Export unter public/rig weicht außerhalb der Bosse von den Specs ab, gen:rig:check ist rot

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`task gen:rig:check` (`bun tools/rigexport/export.ts --pruefen`, byte-genau) ist rot, auch nach dem Boss-Export (RU61.6, B-885). Abweichung oder fehlend außerhalb der Bosse, geprüft 2026-11-19: `figuren/priesterin` (13 Dateien), `figuren/schwertschmied` (13), `gegner/mumie` (11), `gegner/ninja` (12), `gegner/roemischer-spaeher` (15), `gegner/samurai` (12), `gegner/alphawolf` (12), `gegner/wolf` (12), `reittiere/kamel` (17), `reittiere/wolf` (12), kleinere Abweichungen bei `gegner/goblin`, `goblin-bogenschuetze`, `hoehlentroll`, `kristallwaechter` (4), `skelett` (2), `truppen/bauer`, dazu `soll/**` (23 Dateien). Etwa 160 Dateien. Mehrere Gruppen (priesterin, schwertschmied, mumie, ninja, roemischer-spaeher, samurai, kamel) fehlen im Repo ganz, der Export legt sie neu an.

## Ziel

`task gen:rig:check` ist grün; der eingecheckte Export unter `public/rig/**` passt zu den Specs unter `public/grafik/**`.

## Beteiligte und Zielgruppen

Agent PAP (Agent `k3c-grafik` prüft); Rust-Client `rust/crates/k3c-rig`.

## Anforderungen

- `task gen:rig` einmal laufen lassen und den Diff übernehmen (neue Gruppen, geänderte Teile, `soll/**`).
- Danach `task rust:test -- -p k3c-rig` grün; schlagen die `*_gegen_go`-Tests für die geänderten Figuren an, ist das ein Befund (Entscheidung 012: Spec ist die Wahrheit), kein stilles Anpassen.
- Prüfen, ob die neuen Dateien (PNG, Größe) das Paketbudget des Spiels berühren.

## Nicht-Ziele

`gen:rig:check` in die CI aufnehmen; Änderungen an den Specs; Bosse (B-885).

## Regeln und Einschränkungen

Gen-Lauf am Feature (B-742): Dieses Ticket ist das Feature. Der Export liest Specs und Stil nur.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `public/rig/**`, `testdata/rust/rig/**`, `rust/crates/k3c-rig/src/**`
- **Abhängig von:** B-885
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`task gen:rig:check` vor dem Lauf rot (rund 160 Abweichungen), nach `task gen:rig` und Übernahme grün.

## Ausnahme- und Fehlerfälle

Der Export scheitert an einer Spec (Teil ohne Pivot) → nicht an der Spec drehen, Ticket anlegen.

## Akzeptanzkriterien

- **AC-01** `task gen:rig:check` grün.
- **AC-02** `task rust:test -- -p k3c-rig` grün.

## Offene Fragen

keine

## Notizen

Gefunden in RU61.6 (2026-11-19). Der Boss-Export wurde dort getrennt übernommen, die übrigen Gruppen blieben bewusst unverändert.
