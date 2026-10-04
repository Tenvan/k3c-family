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

Tor, Farm, Kaserne haben keinen Bauplatz und keine Wirkung; Taverne, Heilplatz, Schmiede, Rüstkammer und Zaubertum gibt es nicht; die Werkstatt liefert nur Bögen; es gibt kein Truppen-Limit (`docs/rules/archiv/ist-material-gebaeude.md`).

## Ziel

Die Gebäude aus `docs/rules/materialien-gebaeude.md` § 3 haben feste Bauplätze und ihre beschlossene Wirkung. Nutzen: Die Hub-Stufen haben Inhalt (Verteidigung, Truppen, Heilung, Upgrades).

## Beteiligte und Zielgruppen

Spieler; Elite- und Rüstungswerte kommen aus Regelwerk III.

## Anforderungen

- Bauplätze je Gebäude in `data/hub.json`. Der Hub **wächst mit dem Ausbau** (Hub-Stufe und entsprechender Mauerausbau) auf eine Breite, die den freigeschalteten Gebäuden Platz gibt; Breiten und Offsets sind Startwerte der Planung (W1), 🧑 bestätigt sie bei der Spec-Freigabe. Fehlt der Platz, zeigt der Client einen Hinweis (B-207). Bauzeiten: Taverne 12 s, Heilplatz 12 s, Schmiede 16 s, Rüstkammer 16 s (Beschluss Q26, 2026-10-04).
- Tor: je Seite ein Tor **außen** vor der äußersten Mauer (auf Hub-Stufe 1: ±48); eigene Truppen und Spieler passieren, für Gegner wirkt es wie eine Mauer (Hindernis und Angriffsziel), nicht aber für `outerWall` (Beschluss Q27, 2026-10-04).
- Kaserne: Truppen-Limit +10 (Basis 10); es zählen nur Kämpfer (in W3: Bogenschützen); geprüft an genau einer Stelle beim Waffe-Holen, bei vollem Limit bleibt die Waffe im Regal (Beschlüsse Q29 und Q40, 2026-10-04).
- Taverne: bei jedem `dawn` ein Landstreicher an der Taverne, solange dort weniger als 2 stehen; Wanderradius 6; eigene Werte in `data/` (Beschluss Q30, 2026-10-04).
- Heilplatz: heilt Truppen und Spieler in Reichweite, **immer** (auch im Kampf), 5 HP/s, Radius 6 um den Platz (Startwerte; Beschluss Q32, 2026-10-04). W3 baut Limit und Heilplatz vollständig (Beschluss Q40, 2026-10-04).
- Werkstatt: Schwert (Krieger, B-014) neben Bogen, je bis 3 im Waffenregal.
- Schmiede (Elite-Upgrades) und Rüstkammer (Rüstung/Waffen) als Gebäude mit Platzhalterwirkung bis Regelwerk III.
- Zaubertum (Turm-Stufe 5): eigener Schuss statt Bogen, Startwerte 40 Schaden, Radius 3, Reichweite 13, alle 1,5 s; die Schützen steigen beim Ausbau ab und zählen weiter als Kämpfer (Beschluss Q31, 2026-10-04).
- Startwerte für Kosten und HP laut `materialien-gebaeude.md` § 3.2.

## Nicht-Ziele

Elite- und Rüstungswerte (Regelwerk III), Krieger/Elite-Truppen (B-014), Grafiken (B-010), Anzeige (B-117).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 3.2; Werte nur in `data/`; mit 2+ Spielern. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Tor gebaut: Monarch und Bauern laufen hindurch, ein Goblin bleibt davor stehen und greift das Tor an.

## Ausnahme- und Fehlerfälle

Truppen-Limit erreicht → kein Bauer holt eine Waffe, die Waffe bleibt im Regal, Hinweis (B-117); Landstreicher werden weiter zu Bauern (Beschluss Q29, 2026-10-04). Kein Platz im Hub für ein Gebäude → Hinweis „Kein Platz für <Gebäude>“ über dem Hub (B-207). Gebäude zerstört → Wirkung endet.

## Akzeptanzkriterien

- **AC-01** Test je Gebäude: Wirkung wie beschrieben (Tor, Kaserne-Limit, Taverne, Heilplatz, Zaubertum-Schaden).
- **AC-02** Test: Kosten, HP und Platz kommen aus den Daten, nicht aus dem Code.
- **AC-03** Schmiede und Rüstkammer bauen und zerstören funktioniert; ihre Wirkung ist als offen markiert (Regelwerk III).
- **AC-04** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Schmiede/Rüstkammer-Wirkung (Regelwerk III, B-122). Hub-Breiten und Offsets je Stufe bestätigt 🧑 bei der Spec-Freigabe (Q26).

## Notizen

Aus R2.2. Abhängig von B-112. Verwandt mit B-014.
