# RU7.6 · art/rigs: Brücke zu k3c-rig, Rig-Teil der Bibliothek

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru7
- **Abhängig von:** RU7.2, RU5
- **Tickets:** B-1014
- **Kriterien:** AC-04

## Ziel

Rig-Figuren gehen über `art` (Brücke `art/rigs` auf `k3c-rig`), und die Bibliothek lädt die Rig-Teile wie in Go.

## Kontext

Startet erst, wenn RU5 erledigt ist (`rust/crates/k3c-rig` mit `rig` und `paint`). Quellen: `native/k3c-game/art/rigs.go` (83, Test `rigs_test.go`), Rig-Teile in `library.go` (`RigDir`, `paint.LoadAssets`, `RigMiB`) und `budget.go` (`AddBytes`). Platzhalter aus RU7.2: `rust/crates/k3c-game/src/art/pending.rs` (`RU7.6: rigs`). Standard-Ordner `native/k3c-rig/assets`. Fachmodule importieren `k3c-rig` nie direkt (`view/doc.go` › Importregel).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/art/**`

## Nicht-Ziele

Änderungen an `k3c-rig`; fehlt dort etwas: Ticket. Zeichnen der Akteure (RU13).

## Schritte

1. `rigs.go`, `rigs_test.go` und die Rig-Stellen in `library.go`/`budget.go` ganz lesen.
2. `art/rigs.rs` (Actor, Painter, M, Point, ActorFrame, RigView als Re-Export bzw. dünne Hülle wie Go).
3. Platzhalter in `pending.rs` durch den echten Aufruf ersetzen, `default_config` setzt `rig_dir`, `rig_mib` zählt; `pending.rs` löschen, wenn leer.
4. `rigs_tests.rs` portieren.

## Fertig, wenn

- [ ] AC-04: `cargo test -p k3c-game art` grün, inklusive `rigs_tests`; `Library::new(default_config())` lädt `native/k3c-rig/assets`.
- [ ] AC-04: `grep -rn "k3c_rig" rust/crates/k3c-game/src` trifft nur `src/art/`.
- [ ] `task rust:lint` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
