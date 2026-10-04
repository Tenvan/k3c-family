# B-116 · Tor, Kaserne, Taverne, Heilplatz, Schmiede, Rüstkammer und Zaubertum wirken im Spiel

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W3
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint W3

## Ausgangslage

Tor, Farm, Kaserne haben keinen Bauplatz und keine Wirkung; Taverne, Heilplatz, Schmiede, Rüstkammer und Zaubertum gibt es nicht; die Werkstatt liefert nur Bögen; es gibt kein Truppen-Limit (`docs/rules/archiv/ist-material-gebaeude.md`).

## Ziel

Die Gebäude aus `docs/rules/materialien-gebaeude.md` § 3 haben feste Bauplätze und ihre beschlossene Wirkung. Nutzen: Die Hub-Stufen haben Inhalt (Verteidigung, Truppen, Heilung, Upgrades).

## Beteiligte und Zielgruppen

Spieler; Elite- und Rüstungswerte kommen aus Regelwerk III.

## Anforderungen

- Feste Hub-Plätze je Gebäude in `data/hub.json` aus dem Layout von B-206 (W0); kein Platz bewegt sich, Hub-Plätze dürfen zwischen Linie 1 und 2 liegen (Beschlüsse Q43, Q51, 2026-10-04). Bauzeiten: Taverne 12 s, Heilplatz 12 s, Schmiede 16 s, Rüstkammer 16 s (Beschluss Q26, gilt nach Q43).
- Tor: je Mauerlinie ein fester Tor-Platz 4 Units außen (Linie 1: ±48), bezahlbar nur an der äußersten gebauten Linie (Beschluss Q47, 2026-10-04); eigene Truppen und Spieler passieren, für Gegner wirkt es wie eine Mauer (Hindernis und Angriffsziel), nicht aber für `outerWall` (Beschluss Q27, 2026-10-04). Wird außen eine neue Linie gebaut, bleiben Turm und Tor innen stehen und wirken weiter (Beschluss Q54, 2026-10-04).
- Kaserne: Truppen-Limit +10 (Basis 10); es zählen nur Kämpfer (in W3: Bogenschützen); geprüft an genau einer Stelle beim Waffe-Holen, bei vollem Limit bleibt die Waffe im Regal (Beschlüsse Q29 und Q40, 2026-10-04).
- Taverne: bei jedem `dawn` ein Landstreicher an der Taverne, solange dort weniger als 2 stehen; Wanderradius 6; eigene Werte in `data/` (Beschluss Q30, 2026-10-04).
- Heilplatz: heilt Bürger und Spieler in Reichweite (Q63), **immer** (auch im Kampf), 5 HP/s, Radius 6 um den Platz (Startwerte; Beschluss Q32, 2026-10-04). W3 baut Limit und Heilplatz vollständig (Beschluss Q40, 2026-10-04).
- Werkstatt: Schwert (Krieger, B-014) gehört nicht zu W3, siehe Nicht-Ziele (kommt mit W4).
- Schmiede (Elite-Upgrades) und Rüstkammer (Rüstung/Waffen) als Gebäude mit Platzhalterwirkung bis Regelwerk III.
- Zaubertum (Turm-Stufe 5): eigener Schuss statt Bogen, Startwerte 40 Schaden, Radius 3, Reichweite 13, alle 1,5 s; die Schützen steigen beim Ausbau ab und zählen weiter als Kämpfer (Beschluss Q31, 2026-10-04).
- Startwerte für Kosten und HP laut `materialien-gebaeude.md` § 3.2.

## Nicht-Ziele

Elite- und Rüstungswerte (Regelwerk III), Krieger/Elite-Truppen und Schwerter in der Werkstatt (B-014, W4), Grafiken (B-010), Anzeige (B-117).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 3.2; Werte nur in `data/`; mit 2+ Spielern. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Tor gebaut: Monarch und Bauern laufen hindurch, ein Goblin bleibt davor stehen und greift das Tor an.

## Ausnahme- und Fehlerfälle

Truppen-Limit erreicht → kein Bauer holt eine Waffe, die Waffe bleibt im Regal, Hinweis (B-117); Landstreicher werden weiter zu Bauern (Beschluss Q29, 2026-10-04). Tor an einer inneren Linie → nicht bezahlbar (Q47); welcher Platz warum gesperrt ist, zeigt B-207. Gebäude zerstört → Wirkung endet.

## Akzeptanzkriterien

- **AC-01** Test je Gebäude: Wirkung wie beschrieben (Tor, Kaserne-Limit, Taverne, Heilplatz, Zaubertum-Schaden).
- **AC-02** Test: Kosten, HP und Platz kommen aus den Daten, nicht aus dem Code.
- **AC-03** Schmiede und Rüstkammer bauen und zerstören funktioniert; ihre Wirkung ist als offen markiert (Regelwerk III).
- **AC-04** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Schmiede/Rüstkammer-Wirkung (Regelwerk III, B-122). Plätze kommen aus B-206 (W0); Hub-Breiten aus Q26 entfallen (Q43).

## Notizen

Aus R2.2. Abhängig von B-112. Verwandt mit B-014.
