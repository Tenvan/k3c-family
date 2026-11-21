# LA2.3 · Licht-Assets: animierte Flammen je Welt, Halter-Varianten, Ankerpunkte (B-1181)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la2
- **Abhängig von:** –
- **Tickets:** B-1181
- **Kriterien:** AC-05

## Ziel

Je Welt gibt es Licht-Assets als Papierschnitt (Fackel, Laterne, Feuerschale) mit animierter Flamme, über `assets_*` angelegt und gerendert.

## Kontext

Asset-Regeln: Entscheidung 005, Anleitung über MCP `assets_guide`. Assets nur über `assets_*`, Zuordnung in `data/paper.json`. Heute zeichnet `world/lightdraw.rs` Fackeln prozedural (Stab plus drei Flammenebenen, Flackern über FLICKER_PATTERNS). Ticket B-1181. Lane zweit: Gen und Bildvergleich laufen hier (B-742). Die Anbindung im Spiel (Ersatz der prozeduralen Zeichnung) bleibt klein und folgt dem Flammen-Anker aus `light.rs` (`for_each_torch`).

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `data/paper.json`
- `rust/crates/k3c-game/src/world/lightdraw.rs`
- `rust/crates/k3c-game/src/world/light.rs`

## Nicht-Ziele

Kein Monumentalbild (LA2.4), keine Änderung der Lichtkarte oder Halo-Werte.

## Schritte

1. `assets_guide` und Styleguide lesen, Vorbild-Prüfung der vorhandenen Fackel.
2. Pro Welt Fackel, Laterne und Feuerschale als Spec anlegen, Flamme als Animation.
3. Rendern, im Review der Workbench prüfen.
4. Im Spiel an die Fackel-Anker binden, prozedurale Zeichnung als Rückfall belassen.

## Fertig, wenn

- [ ] AC-05: Mindestens drei Licht-Assets je Welt sind angelegt, gerendert und im Spiel an den Fackel-Ankern sichtbar mit animierter Flamme.
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
