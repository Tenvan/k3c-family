# B-1209 · Besorger laufen sichtbar über die Treppe in die obere Stufe und zurück

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WK1
- **Projekt:** NAC
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf-Entscheidung 2026-11-21 (Dialog), Claude trägt ein
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Besorger (Bürger ohne Werkstatt in tieferer Stufe, ECO1.7) wechseln unsichtbar: Sie verschwinden an der Treppe und tauchen oben auf (`rust/crates/k3c-sim/src/island/errand.rs`). Entscheidung Ralf 2026-11-21: sichtbarer Weg.

## Ziel

Der Besorger läuft sichtbar zur Treppe, steigt hinauf, holt dort die Ausrüstung und kehrt auf demselben Weg zurück.

## Beteiligte und Zielgruppen

Ralf (🧑) nimmt am PC mit Pad ab.

## Anforderungen

- **AC-01** Der Bürger bleibt während des Wechsels als Figur sichtbar (Ein- und Ausstieg an der Treppe, Wechsel der Stufe mit Animation oder Ein- und Ausblenden).
- **AC-02** Der Wechsel kostet kein Frame-Budget über der Obergrenze.
- **AC-03** Deterministisch, 2 Spieler.

## Nicht-Ziele

Neue Grafik-Assets.

## Regeln und Einschränkungen

Neue Mechanik nur in Rust; 60 Hz, zwei Ansichten; Spielstand schreibt nichts Neues (Felder `serde(skip)`).

- **Bereich:** `rust/crates/k3c-sim/src/island/**`, `rust/crates/k3c-sim/src/state/**`, `rust/crates/k3c-game/src/actors/**`
- **Abhängig von:** B-1208
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Besorger läuft zur Treppe, ist kurz in der Treppe, erscheint oben, holt den Gürtel, kommt zurück.

## Ausnahme- und Fehlerfälle

Treppe zerstört: Besorger bricht ab und bleibt in seiner Stufe.

## Akzeptanzkriterien

- **AC-01** Test: Figur bleibt über den Wechsel erhalten (Ort, Art, Leben).
- **AC-02** Messung `alloc_je_bild` nicht schlechter.
- **AC-03** Test: gleiche Eingabe, gleicher Ablauf.

## Offene Fragen

Wie wird der Wechsel dargestellt (Ein- und Ausblenden oder Laufen durch die Treppe)? Default: Ein- und Ausblenden an der Treppe; entscheidet Ralf bei der Abnahme.

## Notizen

Quelle: B-1203, ECO1.7.
