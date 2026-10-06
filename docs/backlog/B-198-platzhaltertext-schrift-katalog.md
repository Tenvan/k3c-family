# B-198 · Der Platzhaltertext einer ungeladenen Stufe liest seine Schrift aus dem Katalog

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GR7
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`src/scenes/stageView.ts` setzt den Platzhaltertext „Stufe n wird geladen“ mit fester Größe von 40 px im Szenenraum (`PLACEHOLDER_TEXT`). Im Streifen und Viertel (Zoom 0,5) erscheint er mit 20 px, unter der Nebeninfo-Mindestgröße von 24 px (2 Spieler). S4.2 durfte `stageView.ts` nicht ändern, deshalb steht die Rolle nicht im Katalog `src/scenes/fontRules.ts` und der Test prüft sie nicht.

## Ziel

Der Platzhalter ist eine Rolle in `FONTS` (Nebeninfo, `world`) und erreicht die Mindestgröße in allen Layouts.

## Beteiligte und Zielgruppen

Entwickler (CLI).

## Anforderungen

- Rolle `placeholder` in `FONTS`, `stageView.ts` nutzt `fontStyle('placeholder')`, `fontRules.test.ts` deckt sie ab.

## Nicht-Ziele

Zeichnen der Stufen (S4.1).

## Regeln und Einschränkungen

`CLAUDE.md`, Datei ≤ 400 Zeilen, Q03.

## Beispiele

nicht relevant: kleine Schuld.

## Ausnahme- und Fehlerfälle

nicht relevant: kleine Schuld.

## Akzeptanzkriterien

- **AC-01** `PLACEHOLDER_TEXT` in `stageView.ts` hat keine lose Größe mehr; `task check` mit der neuen Rolle grün.

## Offene Fragen

keine

## Notizen

Entstanden in S4.2. `worldRenderer.ts` hat nach S4.2 395 Zeilen: weitere Texte dort brauchen vorher eine Aufteilung.
