# GEB16 · DEV · Bauschilder, dichter Wald und Baugerüst als Pixel-Grafik

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-584, B-585, B-586, B-587, B-588
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bauschilder statt grauer Rahmen (B-584) und ein dichterer Wald (B-585) sind am 2026-10-24 erstmals umgesetzt und warten auf Abnahme. Im Bau fehlt ein Gerüst (B-586). Entschieden (🧑, 2026-10-24): Der Client zeigt nur, was der Server meldet (B-587), der Wald wird noch dichter, Axt und Spitzhacke als Abbau-Symbole (B-588), Balancing kommt in einen eigenen Sprint (BAL7).

## Ziel

Bauplätze zeigen ein Schild nur dort, wo der Server Münzen annimmt, im Bau ein Gerüst in Gebäudeform als Pixel-Grafik, Abbau-Schilder tragen Axt bzw. Spitzhacke, Wälder stehen voller Bäume. Am Ende sichtbar: Spiel am Gerät mit Schildern, Gerüsten und dichtem Wald.

## Beteiligte und Zielgruppen

Spielende; Agent DEV und CLI; 🧑 nimmt ab.

## Anforderungen

B-584, B-585, B-586, B-587, B-588 › Anforderungen.

## Nicht-Ziele

Holz-Balancing (BAL7, B-589), Burg-Gerüst (GEB11), Höhlen-Wurzeln und Felsen (GEB15).

## Regeln und Einschränkungen

Grafik über das Asset-Studio; Client nur zeichnen (ADR 001); Werte in `data/`; Golden mit Begründung.

## Beispiele

Bezahlte Werkstatt → Gerüst in Werkstattgröße statt Schild; fertig → Gebäude.

## Ausnahme- und Fehlerfälle

Gerüst-Grafik fehlt → einfache gezeichnete Form.

## Akzeptanzkriterien

- **AC-01** Der Snapshot trägt `payable` je Bauplatz, das Schild erscheint nur dort, `buildableRules.ts` ist gelöscht (B-587/AC-01, B-587/AC-02, B-584/AC-01).
- **AC-02** Waldabschnitte sind dichter als zuvor, Golden grün (B-585/AC-01, B-585/AC-02).
- **AC-03** Im Bau steht ein Gerüst-Sprite in Gebäudeform, nie zugleich mit dem Schild (B-586/AC-01, B-586/AC-02).
- **AC-04** Axt und Spitzhacke liegen als Grafik im Katalog und erscheinen am Abbau-Schild (B-588/AC-01, B-588/AC-02).
- **AC-05** Ralf hat Schilder, Wald, Gerüst und Abbau-Symbole am Gerät abgenommen (B-584/AC-02, B-585/AC-03).

## Offene Fragen

Entschieden: Gerüst-Grafiken je Größenklasse. Offen: Ort des Abbau-Schilds (B-588, Default: an markierten Bäumen und Felsen); Endwert der Wald-Dichte (B-585, Default `[22, 30]`). Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB16.1 | – | Umsetzung | autonom | offen |
| GEB16.2 | – | Umsetzung | autonom | offen |
| GEB16.3 | – | Umsetzung | autonom | offen |
| GEB16.4 | – | Umsetzung | autonom | offen |
| GEB16.5 | – | Umsetzung | autonom | offen |
| GEB16.6 | – | Review | autonom | offen |
| GEB16.7 | – | Workshop | Mensch | offen |

- GEB16.1 (SRV) Feld `payable` je Bauplatz im Snapshot und in `docs/protocol.md` (AC-01).
- GEB16.2 (CLI) Client zeigt das Schild nur bei `payable`, `buildableRules.ts` löschen (AC-01).
- GEB16.3 (DEV) Gerüst-Grafiken je Größenklasse sowie Axt und Spitzhacke im Asset-Studio (AC-03, AC-04).
- GEB16.4 (CLI) Gerüst im Bau und Abbau-Schilder im Spiel (AC-03, AC-04).
- GEB16.5 (REG) Wald noch dichter, Golden und `campaign-ausbau.json` anpassen, ohne Balancing (AC-02).
- GEB16.6 Review: alle Kriterien prüfen.
- GEB16.7 (Mensch) Abnahme am Gerät (AC-05).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
