# B-121 · Bauern haben Berufe (Bergmann, Baumeister, Handwerker), und ein Händler tauscht Material gegen Gold

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ein Bauer kann alles; Handwerker und Händler gibt es nicht (`docs/rules/ist-monarch-buerger.md` § 3).

## Ziel

Bauern lassen sich zu Bergmann, Baumeister und Handwerker ausbilden; ein Händler kommt zu Besuch und tauscht Material gegen Gold. Nutzen: Adern, Bau und Truppenaufbau werden steuerbar (`docs/rules/buerger.md` §§ 1–2).

## Beteiligte und Zielgruppen

Spieler und Bauern.

## Anforderungen

- Berufe: Bergmann (+50 % Abbaurate an Adern und Fels, 20 Gold), Baumeister (+50 % Bautempo und Reparatur, 20 Gold), Handwerker (30 Gold, in Werkstatt, Schmiede oder Rüstkammer, 1–2 je Gebäude, +50 % Herstellungstempo); Umschulung kostet erneut.
- Händler: kommt alle 3 Tage (häufiger mit Taverne), bleibt einen Tag, tauscht Material gegen Gold und umgekehrt (10 Material = 5 Gold).
- Werte in `data/troops.json` und `data/economy.json`; Ausbildung am Werkstatt-Platz.

## Nicht-Ziele

Elite, Rüstung, Limit (B-122), Darstellung (B-126), Protokoll (B-123).

## Regeln und Einschränkungen

`docs/rules/buerger.md`; Werte nur in `data/`. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Ein Bauer wird Bergmann und arbeitet an der Stein-Ader mit +50 % Rate; Händler tauscht 100 Holz gegen 50 Gold.

## Ausnahme- und Fehlerfälle

Kein Platz im Gebäude → Ausbildung nicht möglich. Händler ohne Interesse an einem Material → Tausch abgelehnt.

## Akzeptanzkriterien

- **AC-01** Test: Bergmann und Baumeister erhöhen Abbau- und Bautempo um 50 %; Umschulung kostet Gold.
- **AC-02** Test: Handwerker beschleunigt die Herstellung; höchstens 2 je Gebäude.
- **AC-03** Test: Händler erscheint nach Regel, tauscht korrekt und verschwindet nach einem Tag.
- **AC-04** `task check:go` grün.

## Offene Fragen

Händler: Erscheinung, Ort, Preise je Material (mit B-099).

## Notizen

Aus R3.3. Abhängig von B-114 (Adern), B-112 (Hub-Ausbau), B-116 (Gebäude).
