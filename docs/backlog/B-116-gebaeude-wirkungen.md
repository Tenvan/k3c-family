# B-116 · Tor, Kaserne, Taverne, Heilplatz, Schmiede, Rüstkammer und Zaubertum wirken im Spiel

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W3
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Tor, Farm, Kaserne haben keinen Bauplatz und keine Wirkung; Taverne, Heilplatz, Schmiede, Rüstkammer und Zaubertum gibt es nicht; die Werkstatt liefert nur Bögen; es gibt kein Truppen-Limit (`docs/rules/ist-material-gebaeude.md`).

## Ziel

Die Gebäude aus `docs/rules/materialien-gebaeude.md` § 3 haben feste Bauplätze und ihre beschlossene Wirkung. Nutzen: Die Hub-Stufen haben Inhalt (Verteidigung, Truppen, Heilung, Upgrades).

## Beteiligte und Zielgruppen

Spieler; Elite- und Rüstungswerte kommen aus Regelwerk III.

## Anforderungen

- Feste Bauplätze je Gebäude in `data/hub.json`.
- Tor: eigene Truppen und Spieler passieren, Gegner nicht. Kaserne: Truppen-Limit +10 (Basis 10). Taverne: 1 Landstreicher je Tag im Hub. Heilplatz: heilt Truppen und Spieler in Reichweite. Werkstatt: Schwert (Krieger, B-014) neben Bogen, je bis 3 im Waffenregal.
- Schmiede (Elite-Upgrades) und Rüstkammer (Rüstung/Waffen) als Gebäude mit Platzhalterwirkung bis Regelwerk III; Zaubertum (Turm-Stufe 5): Flächenschaden statt Bogen.
- Startwerte für Kosten und HP laut `materialien-gebaeude.md` § 3.2.

## Nicht-Ziele

Elite- und Rüstungswerte (Regelwerk III), Krieger/Elite-Truppen (B-014), Grafiken (B-010), Anzeige (B-117).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 3.2; Werte nur in `data/`; mit 2+ Spielern. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Tor gebaut: Monarch und Bauern laufen hindurch, ein Goblin bleibt davor stehen und greift das Tor an.

## Ausnahme- und Fehlerfälle

Truppen-Limit erreicht → Landstreicher werden nicht zu Bauern/Truppen, Hinweis (B-117). Gebäude zerstört → Wirkung endet.

## Akzeptanzkriterien

- **AC-01** Test je Gebäude: Wirkung wie beschrieben (Tor, Kaserne-Limit, Taverne, Heilplatz, Zaubertum-Schaden).
- **AC-02** Test: Kosten, HP und Platz kommen aus den Daten, nicht aus dem Code.
- **AC-03** Schmiede und Rüstkammer bauen und zerstören funktioniert; ihre Wirkung ist als offen markiert (Regelwerk III).
- **AC-04** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Zaubertum: Schaden und Reichweite (Regelwerk III); Schmiede/Rüstkammer-Wirkung.

## Notizen

Aus R2.2. Abhängig von B-112. Verwandt mit B-014.
