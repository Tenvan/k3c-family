# GEB16 · DEV · Bauschilder, dichter Wald und Baugerüst als Pixel-Grafik

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-584, B-585, B-586
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bauschilder statt grauer Rahmen (B-584) und der dichtere Wald (B-585) sind am 2026-10-24 umgesetzt und warten auf Abnahme. Im Bau fehlt ein Gerüst (B-586).

## Ziel

Bauplätze zeigen Schild, im Bau ein Gerüst in Gebäudeform als Pixel-Grafik, Wälder stehen voller Bäume. Am Ende sichtbar: Spiel am Gerät mit Schildern, Gerüsten und dichtem Wald.

## Beteiligte und Zielgruppen

Spielende; Agent DEV und CLI; 🧑 nimmt ab.

## Anforderungen

B-584, B-585, B-586 › Anforderungen.

## Nicht-Ziele

Burg-Gerüst (GEB11), Holz-Balancing, Höhlen-Wurzeln und Felsen (GEB15).

## Regeln und Einschränkungen

Grafik über das Asset-Studio; Client nur zeichnen (ADR 001); Werte in `data/`; Golden mit Begründung.

## Beispiele

Bezahlte Werkstatt → Gerüst in Werkstattgröße statt Schild; fertig → Gebäude.

## Ausnahme- und Fehlerfälle

Gerüst-Grafik fehlt → einfache gezeichnete Form.

## Akzeptanzkriterien

- **AC-01** Bauschild nur an Plätzen, die gebaut werden dürfen (B-584/AC-01; umgesetzt, Test `buildableRules.test.ts`).
- **AC-02** Waldabschnitte haben 14 bis 20 Bäume (B-585/AC-01, B-585/AC-02; umgesetzt, `deco_test.go`).
- **AC-03** Im Bau steht ein Gerüst-Sprite in Gebäudeform, nie zugleich mit dem Schild (B-586/AC-01, B-586/AC-02).
- **AC-04** Ralf hat Schilder, Wald und Gerüst am Gerät abgenommen (B-584/AC-02, B-585/AC-03).

## Offene Fragen

Umfang der Gerüst-Grafiken (Größenklassen oder je Gebäudeart): Default Größenklassen; 🧑 bestätigt. Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB16.1 | – | Umsetzung | autonom | offen |
| GEB16.2 | – | Umsetzung | autonom | offen |
| GEB16.3 | – | Review | autonom | offen |
| GEB16.4 | – | Workshop | Mensch | offen |

- GEB16.1 (DEV) Gerüst-Grafiken im Asset-Studio (AC-03).
- GEB16.2 (CLI) Gerüst im Spiel statt Schild während des Baus (AC-03).
- GEB16.3 Review: alle Kriterien prüfen (AC-01, AC-02 schon umgesetzt).
- GEB16.4 (Mensch) Abnahme am Gerät (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
