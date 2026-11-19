# B-1107 · Rust-Client: Anzeige rechnet ohne IslandRef (Lager-Maximum, Bau des Endbosses) und ohne Geschichte

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU63
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`props/simview.rs` (`LiveSimView`, RU48.3) ruft die Sim ohne `IslandRef`: `stock_max`/`purse_max` der Wirtschaft fehlen (Lager unbegrenzt angezeigt), `at_lair` ist immer false (kein Auslöser am Endboss-Bau), `site_gold_left`/`site_payable` rechnen ohne `cheap_build`, `stock_bonus` und `storages_elsewhere` (Anzeige „zahlbar“ kann von der Sim abweichen). `hud/pending.rs::WorldDef` und `Campaign::world` sind Platzhalter: die Geschichte (Einblendungen) erscheint im Rust-Spiel nicht.

## Ziel

Die Stufe reicht der Anzeige einen schreibgeschützten Insel-Bezug (Lager-Maximum, Lair-Zustand), `Campaign::world` trägt die `WorldDef` mit Geschichte; die Platzhalter in `hud/pending.rs` entfallen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Baut auf dem Spiegel aus B-1125 auf: `LiveSimView` (`props/simview.rs`) reicht `w.island_ref_view()` an `economy_of`, `site_gold_left`, `site_payable` und `at_lair` statt `None`. Die Geschichte (`WorldDef`-Platzhalter in `hud/pending.rs`, `Campaign::world`) gehört zu B-974 (RU60) und bleibt dort; sie ist nicht Teil dieses Tickets.

## Nicht-Ziele

Geschichte und Einblendungen (B-974); Spielregeln, Protokoll, Spielstand.

## Regeln und Einschränkungen

Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen; RU42; keine neue Abhängigkeit.

Planungsangaben:

- **Bereich:** `rust/crates/k3c-game/src/props/simview.rs`, `rust/crates/k3c-game/src/props/simview_tests.rs`
- **Abhängig von:** B-1125
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test: `LiveSimView::economy_of` zeigt mit gespiegeltem Lager das Maximum der Insel (`stock_max` > 0 bei Lager, mit `stock_bonus` erhöht).
- **AC-02** Test: `site_payable` und `site_gold_left` rechnen mit `cheap_build` der Insel.
- **AC-03** Test: `at_lair` ist false, sobald der Endboss besiegt oder im Kampf ist, sonst true am Bau.
- **AC-04** Ralf sieht am Endboss-Bau den Auslöser und die zahlbaren Preise wie in Go (manuelle Abnahme).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
