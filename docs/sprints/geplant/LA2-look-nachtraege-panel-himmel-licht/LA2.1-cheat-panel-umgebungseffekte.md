# LA2.1 · Cheat-Fenster: Panel für die Umgebungseffekte (B-1179)

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
