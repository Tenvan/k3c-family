# B-1043 · Gehlinie liegt bei 58 % der Ansicht, das Ufer ist 7 % hoch

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** BB2
- **Projekt:** PAP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` § 1: In K2C liegt die Gehlinie bei 57–59 % der Ansicht, das Ufer ist ≈ 7 %, der Fluss ≈ 35 %. Bei uns liegt die Gehlinie bei 66,7 % (`view.WorldShare`), das Ufer bei ≈ 3 % (`world.bankUnits` 20 px), der Fluss bei ≈ 30 %. Das Spiel wird nach Rust portiert (Go-Stopp).

## Ziel

Das Bild hat die drei Bänder Welt, Ufer, Fluss im Verhältnis 58 : 7 : 35, in jeder Ansicht und jeder Split-Screen-Hälfte.

## Beteiligte und Zielgruppen

Spieler am TV, 1 oder 2 Ansichten. Umsetzung: Claude (CLI, Rust-Client), Fachprüfung `k3c-hud`. Abnahme: 🧑.

## Anforderungen

- Gehlinie bei 58 % der Zellenhöhe, ein fester Wert für alle Biome (B-526 bleibt gültig).
- Ufer 7 % der Zellenhöhe, Material und Farbe je Biom bzw. Welt (Erde mit Rissen, Steinkai), Grasbüschel über der Kante.
- HUD, Seekarte und Marken rücken mit; nichts liegt im Fluss außer Spiegelung und Glanz.

## Nicht-Ziele

Spiegelung (B-1044), Maßstab der Welt (B-1045).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/**`
- **Abhängig von:** RU7
- **Braucht 🧑:** Freigabe, Abnahme
- **Größe:** S
- **Sperre:** –
- Kein Änderungsbedarf in der Engine; das Spiel zeichnet nur.

## Beispiele

Fenster 1920 × 1080, eine Ansicht: Gehlinie bei y ≈ 626, Ufer bis ≈ 702, darunter Fluss.

## Ausnahme- und Fehlerfälle

Sehr flache Fenster: Ufer höchstens ein Drittel des Flusses.

## Akzeptanzkriterien

- **AC-01** Test im Rust-Client: Gehlinie bei 58 % ± 1 px, Ufer 7 % in einer und zwei Ansichten.
- **AC-02** Am TV (🧑): HUD und Marken überdecken nichts Wichtiges.
- **AC-03** Prüfläufe des Rust-Clients grün.

## Offene Fragen

keine

## Notizen

Gedächtnis `reference_k2c-bildauswertung`.
