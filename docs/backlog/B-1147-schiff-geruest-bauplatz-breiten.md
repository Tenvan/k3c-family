# B-1147 · Schiff-Baustelle mit eigenem Gerüst, Bauplatz-Breiten für Schiff und Burg

- **Domäne:** CLI
- **Typ:** Problem
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

Die Baustelle des Schiffs nutzt `geruest-6` (200 x 132 px, 1,09 M), geteilt mit Werkstatt, Bogner, Lager, Kaserne, Taverne, Heiler, Schmiede und Rüstkammer (`props/buildings.rs`, `scaffold_class`). Beim Fertigstellen springt das Bild auf 2,97 M (BB1.1). Bauplatz Schiff ist 190 px breit (`props/plan_sites.rs`), das Schiff 440 px; die Burg (4 M, rund 433 px) ragt je Seite rund 85 px über ihr altes Maß (BB1.2). Beides überlappt vermutlich Nachbarplätze (ungeprüft im Spiel).

## Ziel

Baustelle und fertiges Schiff haben dieselbe Größenordnung, Schiff und Burg überdecken keine Nachbarplätze.

## Beteiligte und Zielgruppen

Spieler am TV; Umsetzung Claude (CLI), Spec `geruest-schiff` zeichnet PAP; Abnahme: Ralf.

## Anforderungen

- Eigene Gerüst-Klasse `geruest-schiff` (Spec in `gebaeude-paper`, Zuordnung in `props/buildings.rs`); die anderen Gebäude behalten `geruest-6`.
- Bauplatz-Breiten für Schiff und Burg passend zu den gezeichneten Breiten (`props/plan_sites.rs`), Abstände zu Nachbarplätzen ohne Überlappung.
- Mit 2 Spielern gleichzeitig unverändert nutzbar.

## Nicht-Ziele

Schiffsrumpf verlängern (B-1148).

## Regeln und Einschränkungen

Assets nur über `assets_*` (Entscheidung 005), Gen im Abnahme-Ticket (B-742). Planungsangaben:
- **Bereich:** `rust/crates/k3c-game/src/props/buildings.rs`, `rust/crates/k3c-game/src/props/plan_sites.rs`, `public/grafik/gebaeude-paper/**`, `data/paper.json`
- **Abhängig von:** BB1
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

Schiff-Baustelle 3 M hoch, Schiff 3 M: kein Größensprung. Burg und Mauer daneben: keine Überlappung.

## Ausnahme- und Fehlerfälle

Hafen ohne Platz: Bauplatz bleibt auswählbar, Breite nie kleiner als das Gerüst.

## Akzeptanzkriterien

- **AC-01** Rust-Test: `scaffold_class` liefert für das Schiff `geruest-schiff`, für die anderen Gebäude weiter `geruest-6`.
- **AC-02** Test (Vitest oder Rust): Gerüst-Höhe Schiff 2,5 bis 3,2 M.
- **AC-03** Rust-Test: Bauplatz-Breiten von Schiff und Burg ≥ gezeichnete Breite, keine Überlappung mit Nachbarplätzen.
- **AC-04** Abnahme am TV (Ralf): Hafen und Hub ohne Sprung und Überdeckung.

## Offene Fragen

Bauplatz oder Sprite anpassen (Breite des Schiffs)? Entscheidet Claude mit Blick auf B-1148.

## Notizen

Von BB1.5 angelegt (Befunde BB1.1 und BB1.2).
