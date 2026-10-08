# B-114 · Farm-Plantage lässt Holz nachwachsen, Adern liefern Stein bis Kristall unendlich mit Abbaurate

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** W2
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, Revision 1, mit Sprint W2

## Ausgangslage

Bäume, Felsen und Kupfererz sind endliche Objekte im Level; jeder Baum liefert einmalig 10 Holz (`data/economy.json` › `gatherables`); die Farm hat keine Wirkung. Die Level liefern nur etwa 390 Holz, 330 Stein und 34 Kupfer (B-111, gemessen).

## Ziel

Holz wächst über Farm-Plantagen nach; Stein, Kupfer, Eisen und Kristall kommen unendlich aus Adern, gesteuert über die Abbaurate. Nutzen: Die Kosten aus R2.2 sind erreichbar, die Zeit begrenzt statt der Fundmenge (`docs/rules/materialien-gebaeude.md` § 1).

## Beteiligte und Zielgruppen

Spieler und Bauern; Raten balanced B-099.

## Anforderungen

- **Plantage:** Farm mit 6 Plätzen, je Platz alle 30 s ein Baum (10 Holz) nachwachsend (≈ 12 Holz/min je Farm); Farm-Stufen erhöhen Plätze oder Tempo (Werte mit B-099); `data/buildings.json` › `farm` (`plantation`).
- **Adern:** neues Level-Objekt (`vein`) je Mine-Stufe, **2 Adern je Stufe**, unendlicher Vorrat; höchstens 2 Bauern gleichzeitig; Zielrate bei 2 Bauern: Stein 60/min, Kupfer 45/min, Eisen 35/min, Kristall 25/min; `data/economy.json` › `veins`, Biom-Daten (`data/biomes/*.json`).
- **Markierung:** Eine Ader wird **einmal markiert**, die Markierung bleibt (die Ader verschwindet nie); Kosten = `markCost` des Materials: Stein 1, Kupfer 2, Eisen 2, Kristall 2 Gold (Eisen und Kristall Startwert). Plantage-Bäume brauchen **keine Markierung** (Beschluss Q25, 2026-10-04).
- **Farm-Platz:** Die Farm ist ein fester Weltplatz je Seite zwischen Linie 1 und Linie 2, ungeschützt bis Linie 2 steht (Beschluss Q51, 2026-10-04); der Platz kommt aus dem Layout von B-206 (W0).
- Level-Generator (`engine/level`) setzt die Adern deterministisch; Golden-Level-Daten werden angepasst.
- Level-Objekte (Bäume, Felsen, Erz), Truhen und Drops bleiben als endlicher Startvorrat.
- Die Siegvariante „alles abbauen“ zählt nur endliche Objekte (`stufen.md` § 3).

## Nicht-Ziele

Neue Stufen Eisenstollen und Kristallhöhle (B-115), Darstellung (B-117, Grafik B-010), Lager (B-113).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 1; Werte nur in `data/`; Level-Generator deterministisch (`engine/rng`). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Höhle: 2 Bauern an einer Stein-Ader → etwa 60 Stein/min bis das Lager-Maximum erreicht ist. Eine Farm mit 6 Plätzen liefert nach 5 Minuten etwa 60 Holz.

## Ausnahme- und Fehlerfälle

Kein Bauer frei → Ader ruht. Lager voll → Bauer wartet (B-113). Alle Plantage-Plätze belegt → Baum wächst nicht nach.

## Akzeptanzkriterien

- **AC-01** Test: Der Generator erzeugt je Mine-Stufe genau 2 Adern; Seeds bleiben reproduzierbar (Golden-Level aktualisiert).
- **AC-02** Test: Eine Ader mit 2 Bauern liefert die Zielrate je Material (Toleranz ±10 %), nie mehr als 2 Bauern gleichzeitig.
- **AC-03** Test: Eine Farm lässt je Platz alle 30 s einen Baum nachwachsen, höchstens 6 Bäume.
- **AC-04** Test: Adern und Plantage ändern nichts am endlichen Startvorrat; „alles abbauen“ ignoriert sie.
- **AC-05** `task check` und `task check:go` grün.

## Offene Fragen

keine (Raten, Plantage-Werte und `markCost` von Eisen und Kristall sind Startwerte, Feintuning B-099).

## Notizen

Aus R2.3 (B-111). Verwandt mit B-012 (Mine). Protokoll/Client brauchen die neue Objektart (B-104, B-117).
