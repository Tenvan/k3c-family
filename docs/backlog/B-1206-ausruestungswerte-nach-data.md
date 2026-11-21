# B-1206 · Preise, Regalgrößen und Herstellzeiten der Ausrüstung stehen in data/buildings.json

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** EQ1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf-Entscheidung 2026-11-21 (Dialog), Claude trägt ein
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Rüstungspreis (30 Eisen, 8 Gold), Regalgröße 3 und Herstellzeiten (Schwert 10 s, Rüstung 30 s) sowie das Schwert-Zahlziel stehen als Code-Konstanten in `rust/crates/k3c-sim/src/unit/hammer.rs` und `warrior.rs`. Das verstößt gegen „Werte gehören nach data/“ (Reviewer-Befund FG1.4).

## Ziel

Alle diese Werte stehen in `data/buildings.json`, mit den heutigen Zahlen als Startwerten.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet; Balancing im Abnahme-Ticket.

## Anforderungen

- **AC-01** Optionale Felder an Waffenschmied, Rüstungsschmied und Werkstatt; fehlt ein Feld, gilt der bisherige Wert.
- **AC-02** Die Sim liest die Werte aus den Daten, keine Zahlen mehr im Code.
- **AC-03** Go liest die Felder nur mit (Struct), das Go-Golden bleibt gleich.

## Nicht-Ziele

Zahlen ändern oder balancen.

## Regeln und Einschränkungen

Go-Parität: `data/buildings.json` wird mit Go geteilt, Felder optional; `testdata/rust/data-tree.json.gz` mit `go test ./internal/detvec/ -write` neu schreiben, Grund im Commit.

- **Bereich:** `rust/crates/k3c-sim/src/unit/**`, `rust/crates/k3c-data/**`, `data/buildings.json`, `engine/sim/data.go`, `testdata/rust/**`
- **Abhängig von:** B-1205
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

`armorCost` in der Datenänderung auf 40 Eisen: das Regal verlangt 40 Eisen.

## Ausnahme- und Fehlerfälle

Feld fehlt: bisheriger Wert.

## Akzeptanzkriterien

- **AC-01** Test: Wert aus den Daten wirkt.
- **AC-02** Test: ohne Feld gelten die alten Zahlen.
- **AC-03** Go-Tests und Golden unverändert grün.

## Offene Fragen

keine

## Notizen

Quelle: B-1203, FG1.4 Befund 3.
