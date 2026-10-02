# B-117 · Der Client zeigt Wartezeit, Lagerstand, Hub-Stufe, Adern und Plantage

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Client zeichnet Bauplätze und Truppen; er zeigt weder Material-Maximum noch Hub-Stufe, Wartegründe oder Adern (`src/scenes/worldRenderer.ts`, `HudScene.ts`).

## Ziel

Spieler sehen, warum ein Bauplatz wartet (Bauer, Material), den Lagerstand mit Maximum je Rohstoff, die Hub-Stufe mit Ausbaukosten sowie Adern und Plantage. Nutzen: Bau und Versorgung sind ohne Rätselraten bedienbar.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 testet am Gerät.

## Anforderungen

- Wartegrund am Bauplatz (wartet auf Bauer, wartet auf Material).
- HUD: Lagerstand je Rohstoff mit Maximum (gemeinsam je Insel), Hub-Stufe und Kosten der nächsten Stufe.
- Darstellung von Adern (Abbau läuft), Plantage (Bäume wachsen) und Lager; Grafiken kommen aus B-010, bis dahin Platzhalter.
- Kein Spiel-Logik-Code in `src/scenes`; Daten kommen aus dem Snapshot (B-104).

## Nicht-Ziele

Grafiken und Sound (B-010, B-011), Simulation (B-112 bis B-116).

## Regeln und Einschränkungen

`CLAUDE.md` (Client rechnet nichts, B-Taste frei, 70 px oben frei), `docs/rules/materialien-gebaeude.md` § 4. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Bauplatz bezahlt, kein Stein im Vorrat → Hinweis „wartet auf Material (Stein)“ am Bauplatz; HUD zeigt Stein 40/300.

## Ausnahme- und Fehlerfälle

Snapshot ohne die neuen Felder (alter Server) → Anzeige leer, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Reine Funktion für Wartegrund und Lagerstand-Text ist getestet.
- **AC-02** HUD und Bauplätze zeigen Wartegrund, Lagerstand, Hub-Stufe und Kosten.
- **AC-03** 🧑 hat die Anzeige am Gerät abgenommen.

## Offene Fragen

keine

## Notizen

Aus R2.2 und R2.3. Abhängig von B-104, B-112, B-113, B-114.
