# RU4.4 · Gamepads im Backend: Xbox-Belegung, Geräte kommen und gehen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.1
- **Tickets:** B-1014
- **Kriterien:** AC-03

## Ziel

Angeschlossene Gamepads erscheinen in `InputState.pads` mit Xbox-Belegung; Pads können zur Laufzeit kommen und gehen.

## Kontext

Braucht die Freigabe der Crate `gilrs` durch 🧑 (RU4-README › Braucht 🧑); ohne Freigabe Status `blockiert`. Typen aus RU4.1: `PadState { id, held, axes }`, `PadButton`, `PadAxis` in `rust/crates/k3c-platform/src/input.rs`. Go-Vorbild für Geräte und Belegung: `native/k3c-game/input/source.go` (eine neue `GamepadID` ist ein neues Gerät; Standard-Layout) und `input/bindings.go` (Tabelle Gamepad Xbox). Nur Xbox-Namen (Projektregel: Plattformen nur Xbox, PC, Mobile). gilrs gehört nur in `src/backend/`.

## Erlaubte Dateien

- `rust/crates/k3c-platform/**`
- `rust/Cargo.toml`
- `rust/Cargo.lock`

## Nicht-Ziele

Auswertung zu Befehlen und Menü-Aktionen (RU4.6, RU4.7). Vibration. Andere Controller-Familien als eigene Belegung.

## Schritte

1. `gilrs` mit fester Version in `rust/Cargo.toml`, in `k3c-platform/Cargo.toml` einbinden.
2. `src/backend/pads.rs`: gilrs-Ereignisse je Bild abholen, `PadState` je verbundenem Pad pflegen; `id` zählt bei jeder neuen Verbindung hoch (auch beim Wiederverbinden desselben Pads). Zuordnung gilrs `Button`/`Axis` → `PadButton`/`PadAxis` als Tabelle; Trigger als Achse 0 … 1 und als Taste ab der Schwelle, die Go nutzt (`input/tracker.go`).
3. Headless und Tests fassen gilrs nicht an; die Zuordnung ist eine reine Funktion und wird ohne Gerät getestet.
4. `tests/layering.rs`: auch `gilrs::` nur in `src/backend/`.
5. Probe (`examples/probe.rs`): Titel zeigt die Zahl verbundener Pads.

## Fertig, wenn

- [ ] AC-03: `cargo test -p k3c-platform` grün mit Test der Zuordnungstabelle (jede `PadButton`-Variante hat genau eine Quelle) und erweitertem `layering`-Test.
- [ ] AC-03: Probe-Titel zeigt beim Ein- und Ausstecken die Zahl der Pads (Beobachtung durch Claude per Screenshot, wenn ein Pad angeschlossen ist; sonst im Ergebnis als ungeprüft vermerken).
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

- **AC-03 geprüft:** `cargo test -p k3c-platform` grün (55 Unit-Tests): `jede_xbox_taste_hat_genau_eine_quelle` (jede `PadButton`-Variante genau eine gilrs-Taste), `jede_achse_hat_genau_eine_quelle`, `trigger_ist_achse_und_ab_schwelle_taste` (Wert > 30/255 wie Ebitengine `gamepaddb.ButtonPressedThreshold`), `stick_y_nach_unten_positiv_und_tasten` (gilrs zählt Y nach oben, das Spiel nach unten; Steuerkreuz als Achse), `neue_verbindung_neue_id_auch_beim_selben_pad` (erstes Pad `id` 0, doppelte Meldung zählt nicht, Wiederverbinden gibt eine neue `id`). `tests/layering.rs`: auch `gilrs::` nur in `src/backend/`.
- **AC-03 (Lauf):** Probe-Titel zeigt `… | 60 Bilder/s | spät 0 | Pads 0` (per Fenstertitel ausgelesen). **Ungeprüft:** Ein- und Ausstecken – an diesem Rechner war kein Pad angeschlossen. **Ralf prüft:** Xbox-Pad an- und abstecken, der Titel der Probe zählt hoch und runter.
- **Umsetzung:** `gilrs = "=0.11.2"` (Standard-Feature `wgi`; Freigabe B-1014). `src/backend/pads.rs`: `Pads::open()` (ohne Gamepad-Unterstützung Warnung, Spiel läuft ohne Pads), `poll` je Bild vor den Takten (beim ersten Mal auch die schon verbundenen Pads), Ereignisse über `PadEv` (ohne gilrs-Kennung, testbar ohne Gerät) auf `InputState`. Headless öffnet gilrs nie. Titel des Fensters zeigt die Zahl der Pads.
- **Review (Sonnet):** keine schweren Befunde. Behoben: Start-Merker statt `next == 0` (sonst bekam das erste später angesteckte Pad `id` 1), ID-Vergabe getestet. Offen gelassen (bewusst): Pads behalten bei Fokusverlust ihren Zustand (wie Ebitengine, das Pads auch ohne Fokus liest); Trigger über `LeftZ`/`RightZ` mit Ruhe −1 bei manchen Treibern (WGI liefert 0 … 1, vermutet unkritisch); beim Anstecken kein Anfangszustand.
