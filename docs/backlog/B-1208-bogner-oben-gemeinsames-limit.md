# B-1208 · Bogner steht nur ganz oben, das Truppen-Limit gilt für die ganze Insel

- **Domäne:** SIM
- **Typ:** Problem
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

Seit ECO1.6 steht Produktion nur in Stufe 0 (`topOnly`), der Bogner darf unten stehen. Das Truppen-Limit gilt je Stufe, Besorger zählen mit. Entscheidung Ralf 2026-11-21: Bogner auch nur oben, Limit gemeinsam für die Insel.

## Ziel

Der Bogner ist `topOnly`; das Truppen-Limit zählt alle Kämpfer der ganzen Insel.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet und nimmt ab.

## Anforderungen

- **AC-01** `data/hub.json`: `topOnly` am Bogner, Taverne bleibt Produktion.
- **AC-02** Das Limit zählt Kämpfer aller Stufen einer Insel zusammen.
- **AC-03** Ohne native Regeln unverändert (Go-Parität).

## Nicht-Ziele

Balancing der Limit-Zahl.

## Regeln und Einschränkungen

Neue Mechanik nur in Rust; `testdata/rust/data-tree.json.gz` bei Datenwechsel neu schreiben; deterministisch, 2 Spieler.

- **Bereich:** `data/hub.json`, `rust/crates/k3c-sim/src/world/**`, `rust/crates/k3c-sim/src/state/links.rs`, `rust/crates/k3c-sim/src/island/**`, `testdata/rust/**`
- **Abhängig von:** B-1206
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

Zwei Stufen mit je drei Kriegern bei Limit sechs: kein weiterer Kampfbefehl, in keiner Stufe.

## Ausnahme- und Fehlerfälle

Eine Stufe: wie bisher.

## Akzeptanzkriterien

- **AC-01** Test: `world_site_specs` zeigt den Bogner nur in Tiefe 0.
- **AC-02** Test: Limit zählt über Stufen.
- **AC-03** Go-Golden grün.

## Offene Fragen

keine

## Notizen

Quelle: B-1203, ECO1.6, ECO1.7.
