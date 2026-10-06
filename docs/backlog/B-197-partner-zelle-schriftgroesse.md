# B-197 · Die Schriftregel nennt eine Mindestgröße für die Mitspieler-Zelle

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** GR7
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei einem lokalen Spieler und einem Mitspieler eines anderen Geräts zeigt `computeLayout(1, true)` oben eine Zelle `partner` (1920 × 360) und unten die eigene Zelle (1920 × 720). `docs/rules/bedienung.md` § 2 nennt nur die eigene Ansicht (28 px Pflicht-Info, 24 px Nebeninfo), keine Zahl für die Mitspieler-Zelle. Bei Zoom 1/3 wären Welt-Texte dort mit dem heutigen Katalog (`src/scenes/fontRules.ts`) nur 18,7 px groß. `minFontPx` liefert für `partner` deshalb `null`, `src/scenes/fontRules.test.ts` prüft sie nicht.

## Ziel

Entscheiden, welche Mindestgröße in der Mitspieler-Zelle gilt oder ob dort keine Welt-Texte nötig sind.

## Beteiligte und Zielgruppen

🧑 entscheidet; Spieler mit Partner an einem anderen Gerät.

## Anforderungen

- Eine Zahl je Art (Pflicht-Info, Nebeninfo) oder die Aussage „Mitspieler-Zelle ohne Mindestgröße, Texte dort ausgeblendet“.

## Nicht-Ziele

Die Größen der übrigen Layouts (beschlossen, S4.2).

## Regeln und Einschränkungen

Q03 (kürzen statt verkleinern), `docs/rules/bedienung.md` § 2.

## Beispiele

nicht relevant: Entscheidungsfrage.

## Ausnahme- und Fehlerfälle

nicht relevant: Entscheidungsfrage.

## Akzeptanzkriterien

- **AC-01** `docs/rules/bedienung.md` nennt die Regel für die Mitspieler-Zelle; `minFontPx` und der Test in `fontRules.test.ts` folgen ihr.

## Offene Fragen

Welche Mindestgröße (oder Ausblenden) gilt in der Mitspieler-Zelle? Entscheidet 🧑.

## Notizen

Entstanden in S4.2.
