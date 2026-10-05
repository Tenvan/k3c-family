# B-121 · Bauern haben Berufe (Bergmann, Baumeister, Handwerker), und ein Händler tauscht Material gegen Gold

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W4
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, mit Sprint W4

## Ausgangslage

Ein Bauer kann alles; Handwerker und Händler gibt es nicht (`docs/rules/archiv/ist-monarch-buerger.md` § 3).

## Ziel

Bauern lassen sich zu Bergmann, Baumeister und Handwerker ausbilden; ein Händler kommt zu Besuch und tauscht Material gegen Gold. Nutzen: Adern, Bau und Truppenaufbau werden steuerbar (`docs/rules/buerger.md` §§ 1–2).

## Beteiligte und Zielgruppen

Spieler und Bauern.

## Anforderungen

- Berufe: Bergmann (+50 % Abbaurate an Adern, Fels und Kupfererz, 20 Gold; zählt als einer der höchstens 2 Bauern an einer Ader), Baumeister (+50 % Bautempo und Reparatur, 20 Gold), Handwerker (30 Gold, in Werkstatt, Schmiede oder Rüstkammer, höchstens 2 je Gebäude, +50 % Herstellungstempo je Handwerker); Umschulung kostet erneut (Bergmann: Beschluss Q35, 2026-10-04).
- Herstellungszeiten für alles (Startwerte): Bogen und Schwert 10 s, Elite-Upgrade 20 s, Rüstungsstufe 30 s; Golden ändert sich (Bogen nicht mehr sofort), Begründung im Commit (Beschluss Q35, 2026-10-04).
- Auswahl am Platz: je Angebot ein eigenes Zahlziel als Anhang mit festem `dx` am Gebäude (`data/buildings.json`), entsteht mit dem Bau (A halten, keine neue Taste, kein Bau-Menü); ausgebildet wird der nächste freie Bauer (Beschlüsse Q34, Q52, 2026-10-04).
- Händler: ein Händler je Insel im Hub der Tiefe 0 auf den Hub-Plätzen +8/+12 (Beschluss Q55, 2026-10-04); Zahlziele „Kaufen“ und „Verkaufen“, Kurs 10 Material = 5 Gold in beide Richtungen; ein Material je Besuch, gewählt per `w.rng` aus den freigeschalteten; Ankunft bei `dawn` alle 3 Tage, mit Taverne auf der Insel alle 2 Tage; bleibt einen Tag (Beschluss Q36, 2026-10-04).
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

keine (Zeiten, Kurs und Rhythmus sind Startwerte, Feintuning B-099; Darstellung des Händlers B-126).

## Notizen

Aus R3.3. Abhängig von B-114 (Adern), B-112 (Hub-Ausbau), B-116 (Gebäude).
