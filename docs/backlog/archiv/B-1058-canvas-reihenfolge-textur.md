# B-1058 · Canvas sortiert innerhalb einer Ebene nach Textur statt nach Aufrufreihenfolge

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** RU13
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Vermutet (aus dem Code, nicht im Fenster gemessen): `k3c_platform::Frame::finish` sortiert nach (Ebene, Ausschnitt, Textur, Aufruf). Zeichnen world und actors alle auf Ebene 0, liegen Figuren verschiedener Sheets nicht in Go-Aufrufreihenfolge (hinten nach vorn), sondern nach Textur-ID; überlappende Figuren können falsch verdeckt werden. Das Standbild (RU13.6) zeichnet bewusst in Aufrufreihenfolge und zeigt das darum nicht. Zu klären vor dem Rust-Fensterspiel: je Drawer eigene Ebenen oder Aufrufreihenfolge im Canvas.

## Ziel

Der Canvas zeichnet innerhalb einer Ebene und eines Ausschnitts in Aufrufreihenfolge wie Ebitengine (hinten nach vorn), auch über Texturen. Überlappende Figuren verschiedener Sheets werden nicht mehr nach Textur-ID verdeckt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `Frame::finish` ordnet nach (Ebene, Ausschnitt, Aufruf); aufeinanderfolgende Aufrufe mit gleicher Textur, Art und Ausschnitt bleiben ein Bündel.
- Keine neue Abhängigkeit, kein Fenster nötig (CPU-Seite `batch.rs`).
- Draw-Calls je Bild können steigen (Sprites verschiedener Sheets wechseln sich ab); die Messung am Fenster macht Ralf (G2, RU39.2).

## Nicht-Ziele

Eigene Ebenen je Drawer; Sortierung nach Tiefe oder y im Canvas; Messung der Draw-Calls im Fenster (RU39.2).

## Regeln und Einschränkungen

Port ohne Verhaltensänderung gegenüber Go (B-1014): Go zeichnet in Aufrufreihenfolge. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

- **Bereich:** `rust/crates/k3c-platform/src/batch.rs`, `rust/crates/k3c-platform/src/batch_tests.rs`, `rust/crates/k3c-platform/src/lib.rs`, `rust/crates/k3c-game/src/hud/paint.rs`, `rust/crates/k3c-game/src/still/raster.rs`
- **Abhängig von:** RU13
- **Braucht 🧑:** Gerät (Messung der Draw-Calls im Fenster, G2)
- **Größe:** S
- **Sperre:** –

## Beispiele

Aufrufe Sprite(Sheet B), Sprite(Sheet A), Sprite(Sheet B) auf Ebene 0 → drei Bündel in dieser Reihenfolge (vorher: A, B, B nach Textur).

## Ausnahme- und Fehlerfälle

Verschiedene Ebenen bleiben nach Ebene geordnet; verschiedene Ausschnitte (Split-Screen-Hälften, nicht überlappend) bleiben nach Ausschnitt gruppiert.

## Akzeptanzkriterien

- **AC-01** Test `behaelt_aufrufreihenfolge_auch_ueber_texturen` (k3c-platform): Instanzen und Bündel stehen in Aufrufreihenfolge, Vollfarbe nicht ans Ende sortiert.
- **AC-02** Gleiche Textur nacheinander bildet ein Bündel (`gleiche_textur_nacheinander_bildet_ein_buendel`); Ebene geht vor Aufruf; `cargo test -p k3c-platform -p k3c-game` und `task rust:lint` grün.

## Offene Fragen

keine

## Notizen

Die Vermutung im Ausgangstext bestätigte sich im Code (`finish` sortierte nach Textur-Schlüssel). Auswirkung auf Draw-Calls im Fenster ungemessen; Messung gehört zu Ralfs G2 (RU39.2), darum erst danach mergen.
