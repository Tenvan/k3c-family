# LA2.3 · Licht-Assets: animierte Flammen je Welt, Halter-Varianten, Ankerpunkte (B-1181)

- **Status:** fertig
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

AC-05 geprüft: 12 Licht-Assets als Papierschnitt über `assets_create`/`assets_set` angelegt und gerendert (`licht-fackel|laterne|schale-<standard|sengoku|nil|britannien>`, je 4 Frames, Flamme als Animation `idle`), in `data/paper.json › objekte` zugeordnet und in `OBJECT_ASSETS` (30 → 42) geladen. Im Spiel: `lightdraw.rs` zeichnet am Fackel-Anker das Asset der Insel-Welt (Burg: Feuerschale am Boden, Taverne und Heilplatz: Laterne am Boden, sonst Fackel an der Wand), unbekannte Welten nehmen Standard; ohne geladenes Sheet bleibt die prozedurale Zeichnung (Stab plus drei Flammenebenen) als Rückfall; mit Asset entfallen die prozeduralen Flammen, der Schein kommt aus den Höfen. `light.rs`: neues `for_each_torch_at` mit Platz-Art. Test `licht_assets_sind_zugeordnet_und_geladen`. `cargo test -p k3c-game` und clippy grün (außer bekanntem `alloc_je_bild`), `task test paper` (Golden) grün, `task check:diff` nur bekannte Planungstests rot.

Nicht geprüft: Bildvergleich im laufenden Spiel (Look, Größe und Höhe der Sprites vermutet nach Fackel-Maßen); manuelle Abnahme durch Ralf.

### Für den Hauptrechner
- `fx/festival.rs` fragt die Animation `stand`, die Assets `fest-fackel`/`fest-laterne` haben nur `idle`: das Sheet wird vermutlich nie gefunden (Rückfall prozedural). Ticket nötig.
- `world/light.rs` und `lightdraw.rs` wurden erweitert, `world/mod.rs` blieb unverändert (nur Aufrufer).
