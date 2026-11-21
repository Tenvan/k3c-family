# LA2.2 · Wolken am Himmel und ruhigere Wellenstriche (B-1183)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la2
- **Abhängig von:** LA2.1
- **Tickets:** B-1183
- **Kriterien:** AC-02, AC-03

## Ziel

Der Himmel hat langsam ziehende Wolken, und die Wellenstriche im Wasser sind abgedunkelt und nachts nicht mehr zu hell.

## Kontext

Himmel: `rust/crates/k3c-game/src/world/sky.rs` (draw_sky, Sterne) und `data/look.json` (sky); Werte in `rust/crates/k3c-data/src/look_sky.rs`. Das eingebaute Profil muss der Datei entsprechen (Test `eingebaute_werte_sind_die_der_datei`); das Go-Profil bleibt unverändert (Golden-Tests). Wellenstriche liegen im Wasser-Code (`world/water*`). Ticket B-1183 (Wolken, Wellenstriche). Wolken zunächst prozedural, kein neues Asset.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/world/sky.rs`
- `rust/crates/k3c-game/src/world/sky_tests.rs`
- `rust/crates/k3c-game/src/world/water*.rs`
- `rust/crates/k3c-data/src/look_sky.rs`
- `rust/crates/k3c-data/src/look.rs`
- `data/look.json`

## Nicht-Ziele

Keine Wolken-Assets, kein Monumentalbild (LA2.4), kein Eingriff ins Go-Profil.

## Schritte

1. Wolkenebene im k2c-Profil: Dichte, Tempo, Tönung je Tageszeit, schaltbar über den fx-Schalter `sky`.
2. Wellenstriche abdunkeln (Wert in look.json, Nacht stärker).
3. Tests und Profil-Gleichheit prüfen.

## Fertig, wenn

- [ ] AC-02: Wolken ziehen im k2c-Profil sichtbar am Himmel, im Go-Profil unverändert.
- [ ] AC-03: Wellenstriche sind dunkler, Golden-Tests des Go-Profils unverändert grün.
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
