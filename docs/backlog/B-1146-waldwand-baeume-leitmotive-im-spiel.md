# B-1146 · Waldwand, acht neue Bäume und Leitmotive im Rust-Spiel anbinden

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

BB1.3 und BB1.4 haben `waldwand`, acht neue Bäume (tanne-mittel/-hoch/-dunkel, pappel-hoch/-dunkel, birke-mittel/-hoch/-dunkel) und `leitmotive` (standard, sengoku, nil, britannien) in `data/paper.json` und `public/grafik/` angelegt. `rust/crates/k3c-data/src/paper.rs` liest die Schlüssel `waldwand` und `leitmotive` nicht, `props/nature.rs` kennt nur `TREE_KINDS` (tanne, pappel, birke). Die Assets sind damit ohne Verbraucher.

## Ziel

Wald, Plantage und Hub-Kulissen zeigen Baumhöhen-Varianten, die Waldwand und das Leitmotiv der Welt; Zusammenarbeit mit der Parallax-Ebene aus RU55.6.

## Beteiligte und Zielgruppen

Spieler am TV; Umsetzung Claude (CLI), Abnahme am TV: Ralf.

## Anforderungen

- `k3c-data` liest `waldwand` und `leitmotive` (`serde(default)`, alte Dateien bleiben lesbar).
- Baumwahl pro Platz deterministisch aus Seed und Position (Variantenwahl in drei Höhen, dunkle Variante nur in der Waldwand-Nähe).
- Leitmotiv je Welt als Schicht hinter der Welt, Platz aus `data/look.json` › `worlds.<level>.motif`.
- Frame-Budget und Texturbudget (B-1151) bleiben im Rahmen; Split-Screen unverändert.

## Nicht-Ziele

Neue Assets zeichnen (B-1148, B-1150), Tonung und Dunst (RU55.4 steht).

## Regeln und Einschränkungen

Neue Mechanik nur in `rust/crates/k3c-sim`, das Spiel zeichnet nur; Entscheidung 005; Test beweist Regel, Spaß prüft Ralf. Planungsangaben:
- **Bereich:** `rust/crates/k3c-data/src/paper.rs`, `rust/crates/k3c-game/src/props/nature.rs`, `rust/crates/k3c-game/src/art/**`, `data/look.json`
- **Abhängig von:** BB1
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Wald-Level: Tannen in drei Höhen, dahinter die Waldwand. Britannien: Steinkreis-Motiv hinter dem Hub.

## Ausnahme- und Fehlerfälle

Fehlt `waldwand` oder ein Leitmotiv in `paper.json`: keine Schicht, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `k3c-data`-Test: `paper.json` liefert `waldwand` und vier `leitmotive`; eine Datei ohne die Schlüssel lädt weiter.
- **AC-02** Rust-Test: Baumvarianten werden je Platz deterministisch gewählt, alle 12 Bäume kommen vor.
- **AC-03** Rust-Test: Leitmotiv-Zuordnung je Welt aus `look.json`.
- **AC-04** Abnahme am TV (Ralf): Wald, Plantage und Hub je Welt sehen stimmig aus.

## Offene Fragen

Zusammenlegen mit RU55.6-Nachträgen? Entscheidet Ralf.

## Notizen

Von BB1.5 angelegt (Befunde BB1.3 und BB1.4).
