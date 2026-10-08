# B-295 · Handwerker lassen sich auch für Schmiede und Rüstkammer ausbilden

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/rules/buerger.md` § 1 nennt Handwerker „in Werkstatt, Schmiede oder Rüstkammer (1–2 je Gebäude)“. In
`data/buildings.json` gibt es das Angebot `craftsman` aber nur an der Werkstatt (`dx −4`, W0.1); Schmiede (nur
`eliteWarrior`) und Rüstkammer (keine `offers`) haben keins. Seit W4.2 bindet die Ausbildung einen Handwerker an das
Gebäude, an dessen Angebot er bezahlt wurde (`engine/sim/professions.go`), deshalb beschleunigt heute kein Handwerker
Elite-Upgrade und Rüstung (W4.3b).

## Ziel

Schmiede und Rüstkammer bekommen einen Weg, Handwerker auszubilden, oder die Regel sagt ausdrücklich, dass Handwerker
nur in der Werkstatt arbeiten.

## Beteiligte und Zielgruppen

Spieler (Koop); 🧑 entscheidet über Plätze und `dx`, REG pflegt die Werte.

## Anforderungen

- Entscheidung: je ein Angebot `craftsman` mit festem `dx` an Schmiede und Rüstkammer (Abstand ≥ 4 Units zu anderen
  Anhängen, Datentest aus W0.1) oder Regel ändern.
- Kein neuer Code nötig, wenn nur `offers` in `data/buildings.json` ergänzt werden (die Mechanik liest sie schon).

## Nicht-Ziele

Neue Tasten oder Bau-Menü (Q34), Balancing der Kosten (B-099).

## Regeln und Einschränkungen

Angebote als Anhänge mit festem `dx` (Q52); Plätze und `dx` ändert nur W0 bzw. REG nach Beschluss.

## Beispiele

Schmiede gebaut, Spieler hält A am Handwerker-Angebot der Schmiede → nächster freier Bauer wird Handwerker der
Schmiede, das Elite-Upgrade dauert 20 s ÷ 1,5.

## Ausnahme- und Fehlerfälle

nicht relevant: offene Entscheidung, kein Verhalten festgelegt.

## Akzeptanzkriterien

- **AC-01** `docs/rules/buerger.md` § 1 und `data/buildings.json` stimmen überein (Angebot an Schmiede und Rüstkammer
  oder Regel nur Werkstatt); `task check:go` grün.

## Offene Fragen

Welche `dx` für das Handwerker-Angebot an Schmiede (+28, Anhang Elite-Krieger schon bei `dx +4`) und Rüstkammer (+40),
oder Handwerker nur in der Werkstatt? Entscheidet 🧑.

## Notizen

Gefunden in W4.2 (Sprint W4).
