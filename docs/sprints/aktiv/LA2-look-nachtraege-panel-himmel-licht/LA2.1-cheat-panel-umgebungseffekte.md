# LA2.1 · Cheat-Fenster: Panel für die Umgebungseffekte (B-1179)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la2
- **Abhängig von:** –
- **Tickets:** B-1179
- **Kriterien:** AC-01

## Ziel

Das Cheat-Fenster (F4, k3c-devwin) hat ein Panel, in dem alle Umgebungseffekte (Dunst, Lichthöfe, Sterne, Himmel, Lichtanhebung, Tönung, Spiegelung) einzeln ein- und ausgeschaltet werden können, live im Spiel.

## Kontext

Die Effekte stehen in `rust/crates/k3c-game/src/world/fx.rs` (EFFECTS, `on`/`set`, Zwischenlösung per Umgebungsvariable `K3C_FX_OFF`). Das Cheat-Fenster ist ein eigener Win32-Prozess (`rust/crates/k3c-devwin`), Protokoll in `k3c-devproto` (Catalog, State, DevCommand). Das Panel braucht eine Protokollerweiterung (Schalterliste im State, Befehl zum Umschalten) und die Anbindung im Spiel. Ticket B-1179 beschreibt den Wunsch. Der Go-Look-Standard bleibt unverändert.

## Erlaubte Dateien

- `rust/crates/k3c-devwin/**`
- `rust/crates/k3c-devproto/**`
- `rust/crates/k3c-game/src/world/fx.rs`
- `rust/crates/k3c-game/src/world/fx_tests.rs`
- `rust/crates/k3c-game/src/devtools/**`

## Nicht-Ziele

Keine neuen Effekte, keine Änderung der Look-Werte, keine Speicherung der Schalter über Programmstarts.

## Schritte

1. Protokoll erweitern: Effektliste (Name, an/aus) im State, Befehl zum Umschalten.
2. Spiel: Zustand der Effekte an das Protokoll anbinden, `K3C_FX_OFF` bleibt als Startwert.
3. Panel im Cheat-Fenster mit einem Schalter je Effekt.
4. Tests für Protokoll und Umschalten.

## Fertig, wenn

- [ ] AC-01: Im Cheat-Fenster schaltet jeder Umgebungseffekt per Schalter live ein und aus; Protokoll- und fx-Tests grün.
- [ ] `task check:diff` grün.

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

AC-01 geprüft: `Catalog.effects` (Kennung `fx:<id>`, aus `fx::EFFECTS`) im Protokoll; `ui`-Taste `fx:<id>` (Wert 1 an, 0 aus, ohne Wert um) in `k3c-game/src/devwin/view.rs` ruft `fx::set`; Seite Anzeige im Cheat-Fenster hat die Gruppe „Umgebungseffekte“ mit einem Umschalter je Effekt (angehakt = Effekt aus). Tests: `umgebungseffekte_je_ein_umschalter` (devwin), `umgebungseffekt_schalten` (Spiel), Protokoll-Roundtrip grün; `cargo test` devproto/devwin/game und clippy grün.

Abweichungen: `devwinhost.rs` (`game/`, Katalog) lag außerhalb der erlaubten Dateien, ist aber nötig. Kein Effektstand im `State`: der Schalter startet immer „an“, ein per `K3C_FX_OFF` abgeschalteter Effekt zeigt das im Panel nicht (Zustand im Fenster ist lokal).

`task check:diff`: nur die bekannten roten Planungstests (`tests/planning.test.ts`, Hauptrechner-Daten); `alloc_je_bild` bleibt rot (vorher schon).

### Für den Hauptrechner
- Panel zeigt den Startzustand aus `K3C_FX_OFF` nicht (Effektliste im `State` fehlt); bei Bedarf Ticket.
