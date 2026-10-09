# B-372 · Das Preisschild nennt die Taste zum Bezahlen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit S9.1 (B-319) zeigt das Aktionen-Overlay im Bereich eines Preisschilds keinen Hinweis mehr, „das Preisschild trägt die Aktion“. Das Preisschild (`src/scenes/siteView.ts` › `drawPrice`) zeigt aber nur Name, Material und Münzkreise, keine Taste. Das Beispiel in B-319 („nur das Preisschild mit ‚Leertaste halten‘“) setzt voraus, dass es die Taste nennt. Außerhalb der geführten ersten Nacht (Hinweis `guide.pay`) sieht ein neuer Spieler am Bauplatz also nicht mehr, welche Taste er halten muss.

## Ziel

Am Bauplatz und am Bogen-Regal ist erkennbar, welche Taste bezahlt, ohne zweites Element an derselben Weltposition.

## Beteiligte und Zielgruppen

Neue Spielende; 🧑 entscheidet (Glyph im Preisschild oder so lassen); Agent in CLI.

## Anforderungen

- Das Preisschild zeigt die Bestätigen-Glyph des Geräts des nächsten Spielers (z. B. „Leertaste halten“), oder 🧑 entscheidet, dass Preis und Münzkreise genügen.
- Je Weltposition weiter nur ein Element (B-319).

## Nicht-Ziele

Den Aktionen-Hinweis am Preisschild zurückholen (widerspricht B-319/AC-02).

## Regeln und Einschränkungen

CLI zeichnet nur; Glyphen aus `src/scenes/glyphs.ts`; zwei Spieler mit verschiedenen Geräten am selben Bauplatz beachten.

## Beispiele

Spieler an der Tastatur steht am unbezahlten Mauer-Bauplatz → Preisschild „Mauer · Leertaste halten“ mit Münzkreisen.

## Ausnahme- und Fehlerfälle

Zwei Spieler mit verschiedenen Geräten am selben Preisschild → Glyph des näheren Spielers (Vorschlag, 🧑 entscheidet).

## Akzeptanzkriterien

- **AC-01** 🧑 hat entschieden (Glyph im Preisschild oder nicht); bei Glyph: Test belegt die Taste je Gerät im Preisschild.

## Offene Fragen

Braucht das Preisschild die Taste, oder genügen Preis und Münzkreise (🧑)? Blockierend; Beobachtung in S9.5 (B-319/AC-04) hilft bei der Entscheidung.

## Notizen

Aus S9.1. Nummer von Hand B-372, weil B-370 (`sprint/s8`) und B-371 (`sprint/pl1`) auf offenen Sprint-Branches vergeben sind.
