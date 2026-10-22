# GEB11 · REG, SIM, DEV, CLI · Hub wächst kleinteilig vom Lagerfeuer bis zur Burg

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** REG, SIM, DEV, CLI
- **Reife:** bereit
- **Tickets:** B-458
- **Start-Commit:** b1ad27a0
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22 (Auftrag von Ralf: GEB komplett autonom)

## Ausgangslage

Der Hub hat fünf Stufen (GEB4: Burg und Hub-Stufen 1–5). 🧑 wünscht (2026-10-17), dass er kleinteiliger wächst wie in Kingdom Two Crowns: Lagerfeuer mit Grill, dann ein Haus und so weiter bis zur Burg, je Stufe mit freigeschalteten Gebäuden und Schilden (B-458). GEB9 (Ausbaustufen der Hub-Gebäude) klärt in GEB9.1 die Abgrenzung zu B-458.

## Ziel

Der Hub wächst in mehr, kleineren Stufen sichtbar vom Lagerfeuer bis zur Burg. Am Ende sichtbar: Wachstumsreihe in der Vorschau und im Spiel, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spielende am TV; 🧑 legt Stufenfolge und Freischaltungen fest; Agent in REG, SIM, DEV und CLI nacheinander.

## Anforderungen

B-458 › Akzeptanzkriterien (Spec noch Entwurf). Sprint-eigen: Regel- und Server-Anteil (B-458/AC-04) läuft hier als eigene Sessions statt als neues Ticket, falls der Workshop `data/` oder `engine/` betrifft.

## Nicht-Ziele

Ausbaustufen der übrigen Hub-Gebäude (GEB9); Werte-Runde zu Kosten der Stufen (Projekt BAL); Migration alter Spielstände (B-288).

## Regeln und Einschränkungen

Feature-Kette REG → SIM → DEV → CLI. **Abhängigkeit:** nach GEB9 (GEB9.1 legt die Abgrenzung fest). Neue Felder in `data/` legt SIM an, Werte REG; Golden nur mit Begründung. Grafik nach `docs/standards/gebaeude-grafik.md` § 6. Deterministisch, 2 Spieler.

## Beispiele

Neue Insel → Lagerfeuer mit Grill; erste Stufe bezahlt → Haus, Bauplatz Werkstatt frei; letzte Stufe → Burg mit Schildern.

## Ausnahme- und Fehlerfälle

Workshop entscheidet „nur Grafik“ → SIM-Session entfällt (`verworfen` mit Grund). Stufe ohne Grafik → Grafik der vorigen Stufe.

## Akzeptanzkriterien

- **AC-01** Stufenfolge und Freischaltungen je Stufe sind als Entscheidung von 🧑 festgelegt und in `docs/rules/` nachgezogen (B-458/AC-01).
- **AC-02** Simulation und Daten kennen die neue Stufenfolge, falls der Workshop Regeln ändert; `task check:go` und `task balance` grün (B-458/AC-04).
- **AC-03** Hub-Grafik je Stufe als Daten-JSON und PNG, die Vorschau zeigt die Wachstumsreihe (B-458/AC-02).
- **AC-04** Das Spiel zeichnet den Hub nach der Stufe; 🧑 nimmt Vorschau und Spiel ab (B-458/AC-03).

## Offene Fragen

Beantwortet von Claude (autonom, 2026-10-22, Default bis 🧑 widerspricht):

1. **Reine Grafik oder neue Regeln?** Nur Grafik und Client: keine Änderung an `engine/`, `data/`, Protokoll oder Spielstand. Hub-Stufen 1–5 und ihre Freischaltungen bleiben.
2. **Anzahl Stufen je Level:** sichtbar mehr Looks ohne mehr Hub-Stufen. Der Client leitet sie aus `hubLevel` und `hubUpgrade` (bezahltes Gold, Wartegrund) ab: Hub-Stufe 1 zeigt Lagerfeuer mit Grill (nichts bezahlt), Zelt (Gold fließt), Hütte (ab der Hälfte des Golds), Holzburg (Gold voll); während ein Bauer den Ausbau baut, steht ein Gerüst vor der Burg; Stufen 2–5 sind die Burgen aus GEB4 (`materialien-gebaeude.md` § 2.1).
3. **Reihenfolge der Gebäude und Schilde:** unverändert (§ 3, erzeugter Block aus `data/hub.json`). Schilde sind schon da (Preisschild, Wartegrund).
4. **Abgrenzung:** GEB4 bleibt die Burg je Hub-Stufe, GEB9 die Hub-Gebäude je Hub-Stufe; GEB11 ergänzt nur die Looks vor der Holzburg und das Gerüst.

**Vorlage für 🧑 (kritische Entscheidung, eigener Sprint): echte feinere Stufenfolge im Server**

- *A – so lassen (nur Grafik, Stand GEB11):* kein Risiko, aber die Zwischenlooks schalten nichts frei und hängen an der Bezahlung der nächsten Stufe.
- *B – Unterstufen nur in Hub-Stufe 1 (z. B. 1a Lagerfeuer, 1b Zelt, 1c Haus, je kleine Goldkosten):* `data/hub.json` › `levels` würde 7 statt 5 Einträge; `engine/sim/world_lines.go` „Linie k ab Hub-Stufe k“, `gateHubLevel`, Mauer- und Turm-Stufen (`data/buildings.json` › `levels`) und alle `hubLevel` der Plätze müssten von „Stufe“ auf „Material-Stufe“ umgestellt werden; Golden neu, Balancing (Werkstatt und Farm erst ab 1b/1c?) mit `sim_test`.
- *C – neues Feld „Ausbau-Schritte je Stufe“ (z. B. `levels[n].steps`, je Schritt Gold-Teilbetrag und Freischaltung):* Hub-Stufe bleibt Material-Stufe, Schritte schalten Plätze frei (`sites[].hubStep`); Protokoll bekommt `hubStep`, Spielstand speichert ihn, Gold-Teilzahlungen werden echte Meilensteine. Mehr Aufwand (SIM, SRV-Protokoll, CLI, Studio-Planer WZ9), aber sauber getrennt.
- **Empfehlung: C**, falls die Freischaltung je Unterstufe gewollt ist (K2C-Gefühl: das erste Gebäude erst nach dem ersten Schritt); sonst A. B vermeiden: es vermischt Material-Stufe und Wachstum und bricht die Regel „Linie k ab Hub-Stufe k“.
- Betroffen bei B/C: `engine/sim/hub_level.go`, `world_lines.go`, `economy_view.go`, `data/hub.json`, `docs/protocol.md`, `testdata/golden/`, Gebäude-Planer (WZ9), Regeltabelle (`task gen:rules`), Balancing (BAL).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB11.1 | `GEB11.1-wachstumsreihe-festlegen.md` | Workshop | autonom | fertig |
| GEB11.2 | `GEB11.2-stufenfolge-simulation.md` | Umsetzung | autonom | verworfen |
| GEB11.3 | `GEB11.3-grafik-wachstumsreihe.md` | Umsetzung | autonom | offen |
| GEB11.4 | `GEB11.4-spiel-zeichnet-wachstum.md` | Umsetzung | autonom | offen |
| GEB11.5 | `GEB11.5-review.md` | Review | autonom | offen |
| GEB11.6 | `GEB11.6-abnahme.md` | Review | Mensch | offen |

GEB11.1 ersetzt den Workshop durch eine Festlegung von Claude (Auftrag „GEB komplett autonom“); GEB11.2 entfällt bei „nur Grafik“. Kette REG → DEV → CLI, Review, Abnahme durch 🧑.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
