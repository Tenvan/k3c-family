# B-374 · Das Preisschild nennt die Taste zum Bezahlen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** BD1
- **Projekt:** BED
- **Erstellt:** 2026-10-09
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-14, Chat 🧑 (Revision 2)

## Ausgangslage

Seit S9.1 (B-319) zeigt das Aktionen-Overlay im Bereich eines Preisschilds keinen Hinweis mehr, „das Preisschild trägt die Aktion“. Das Preisschild (`src/scenes/siteView.ts` › `drawPrice`) zeigt aber nur Name, Material und Münzkreise, keine Taste. Das Beispiel in B-319 („nur das Preisschild mit ‚Leertaste halten‘“) setzt voraus, dass es die Taste nennt. Außerhalb der geführten ersten Nacht (Hinweis `guide.pay`) sieht ein neuer Spieler am Bauplatz also nicht mehr, welche Taste er halten muss.

## Ziel

Das Preisschild an Bauplatz und Bogen-Regal bleibt bei Name, Material und Münzkreisen; es nennt keine Taste. Das Ticket hält diese Entscheidung fest.

## Beteiligte und Zielgruppen

Neue Spielende; 🧑 entscheidet (Glyph im Preisschild oder so lassen); Agent in CLI.

## Anforderungen

- Das Preisschild (`src/scenes/siteView.ts` › `drawPrice`) bleibt unverändert; keine Bestätigen-Glyph im Preisschild.
- Je Weltposition weiter nur ein Element (B-319).
- Welche Taste bezahlt, zeigt weiter der geführte Hinweis `guide.pay` in der ersten Nacht.

## Nicht-Ziele

Den Aktionen-Hinweis am Preisschild zurückholen (widerspricht B-319/AC-02); eine Taste oder Glyph im Preisschild; Änderungen an `drawPrice`.

## Regeln und Einschränkungen

CLI zeichnet nur; Glyphen aus `src/scenes/glyphs.ts`; zwei Spieler mit verschiedenen Geräten am selben Bauplatz beachten.

## Beispiele

Spieler an der Tastatur steht am unbezahlten Mauer-Bauplatz → Preisschild zeigt „Mauer“ mit Münzkreisen, ohne Tastenangabe.

## Ausnahme- und Fehlerfälle

nicht relevant: Das Preisschild ändert sich nicht; zwei Spieler mit verschiedenen Geräten am selben Preisschild sehen dasselbe.

## Akzeptanzkriterien

- **AC-01** Die Entscheidung „keine Taste im Preisschild“ ist festgehalten (🧑, 2026-10-14); `drawPrice` bleibt unverändert, ein bestehender Test belegt, dass das Preisschild Name, Material und Münzkreise zeigt.

## Offene Fragen

keine. Entschieden 2026-10-14 (🧑, Chat): Preis und Münzkreise genügen, keine Taste im Preisschild.

## Notizen

Aus S9.1. Nummer von Hand B-372 (wegen Doppelvergabe 2026-10-09 umnummeriert auf B-374), weil B-370 (`sprint/s8`) und B-371 (`sprint/pl1`) auf offenen Sprint-Branches vergeben sind.
