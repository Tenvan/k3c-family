# B-1045 · Die Ansicht zeigt etwa 5 Monarchenhöhen Welt über der Gehlinie

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU55
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` § 2: In K2C sind über der Gehlinie ≈ 5 M Welt zu sehen, ≈ 15 M in der Breite. Bei uns ≈ 3,5 M Höhe und ≈ 9 M Breite (`ViewUnits` 420/32 über 2/3 der Zelle). Unser Bild ist damit enger und die Figuren größer.

## Ziel

Die Welt wirkt weit, der König klein; man sieht mehr vom Hub und kommt Gegnern früher auf die Spur.

## Beteiligte und Zielgruppen

Spieler am TV auf dem Sofa (Lesbarkeit!). Umsetzung: Claude (CLI, Rust-Client). Entscheidung und Abnahme: 🧑.

## Anforderungen

- Sichtbare Welt über der Gehlinie auf ≈ 5 M (≈ 19 Units bei M = 121,5 px und `UNIT_PX` 32), Wert `viewUnits` im Profil `k2c` von `data/look.json` statt als Konstante `VIEW_UNITS = 420/32` in `game/layout.rs`, `replay/layout.rs`, `still/state.rs`, `audio/mix.rs` und `props/decor.rs` (`VIEW_TOP`).
- Der Maßstab wird in Monarchenhöhen (M) angegeben: `worldHeightM` 5,0; Bildpunkte je Unit folgen aus Zellenhöhe × `worldShare` ÷ `viewUnits`.
- Split-Screen: M in jeder Hälfte ≈ 0,7-fach der vollen Ansicht, nicht halb (Faktor `splitScale` 0,7 im Profil); Trenner schmal (≈ 2 % der Höhe).
- Lesbarkeit von Preis-Anzeigen, Münzen und Marken bleibt erhalten (Größe in Bildschirm-px statt Welt-Maßstab, wo nötig); Mindestgröße der Figuren in px.
- Kamera (`world/camera.rs`, `cull.rs`): Totzone, Mitführung und Culling rechnen mit der neuen Breite; Parallax-Ebenen füllen die breitere Welt ohne Kanten.

## Nicht-Ziele

Größen der Assets selbst (BB1).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/world/camera.rs`, `rust/crates/k3c-game/src/world/cull.rs`, `rust/crates/k3c-game/src/game/layout.rs`, `rust/crates/k3c-game/src/replay/layout.rs`, `rust/crates/k3c-game/src/still/state.rs`, `rust/crates/k3c-game/src/audio/mix.rs`, `rust/crates/k3c-game/src/props/decor.rs`, `rust/crates/k3c-game/src/hud/layout.rs`, `data/look.json`
- **Abhängig von:** B-1043
- **Braucht 🧑:** Entscheidung, Abnahme
- **Größe:** M
- **Sperre:** –
- Entschieden mit Default (autonom, Claude, 2026-11-19; 🧑 kann am TV widersprechen): 5 M, ein Wert in `data/look.json`. Die sichtbare Breite beeinflusst die Wahrnehmung (Wellen früher sichtbar), nicht die Regeln; Sim und Golden unberührt (Profil `go` hält den alten Maßstab für die Bildvergleiche).

## Beispiele

1080p, eine Ansicht: Monarch ≈ 130 px hoch statt ≈ 210 px.

## Ausnahme- und Fehlerfälle

Kleine Fenster: Mindestgröße der Figuren in px, darunter weniger Weite.

## Akzeptanzkriterien

- **AC-01** Maßstab aus `data/`, Test: M ≈ 12 % der Ansicht bei einer, ≈ 16 % je Hälfte bei zwei Ansichten.
- **AC-02** Am TV (🧑): Figuren, Preise und Münzen vom Sofa lesbar; 🧑 wählt den endgültigen Wert.

## Offene Fragen

Wie weit zurücknehmen (4, 4,5 oder 5 M)? 🧑 entscheidet am TV; Default 5 M.

## Notizen

Gedächtnis `reference_k2c-bildauswertung`.
