# B-215 · Die Texte von Touch-Overlay, Shell und Werkzeug-Seiten kommen aus den zentralen Textdateien

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** PL1
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

S5.3 (B-172) hat alle Spieltexte von `src/scenes` und `src/online` in `src/core/texts.de.ts` und `src/core/texts.en.ts` verlegt (`t(key, params?)`). Drei Bereiche gehören zu einer anderen Domäne und blieben deutsch im Code: `src/input/touchInput.ts` (`aria-label` der Touch-Tasten: „Vollbild“, „Sprinten“, „Münzen geben / beitreten“), `src/core/shell.ts` (Home-Button, `title`: „Zurück zur Startseite …“) und die Seiten unter `src/tools/` (Testseiten, Entwickler-Texte).

## Ziel

Die Texte von Touch-Overlay und Shell stehen in den Textdateien und folgen der gewählten Sprache; für die Werkzeug-Seiten ist entschieden, ob sie deutsch bleiben (Entwickler-Texte) oder übersetzt werden.

## Beteiligte und Zielgruppen

Spieler am Touch-Gerät und auf der Xbox (Home-Button); Entwickler (Werkzeug-Seiten). Entscheidung über die Werkzeug-Seiten: 🧑.

## Anforderungen

- Touch-Overlay und Shell holen ihre Texte über `t()`; `src/core` importiert dabei nichts aus `src/scenes`.
- Der Rückfall auf Deutsch bleibt (fehlender Text, unbekannte Sprache).

## Nicht-Ziele

Weitere Sprachen, Dokumentation, Texte in `src/scenes` und `src/online` (erledigt in S5.3).

## Regeln und Einschränkungen

`CLAUDE.md` › Seiten & Navigation (`installPageChrome()`), Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit. Die Regel `src/scenes/textRule.test.ts` ließe sich auf `src/input` und `src/core/shell.ts` ausdehnen.

## Beispiele

Sprache English → der Home-Button zeigt „Back to start (View + Menu, Home)“, die Touch-Taste „Coins / join“ als `aria-label`.

## Ausnahme- und Fehlerfälle

Die Shell läuft auf Seiten ohne Spiel (Landingpage): `t()` liest die Sprache dann aus `loadSettings()`, ohne Speicher gilt Deutsch.

## Akzeptanzkriterien

- **AC-01** `src/input/touchInput.ts` und `src/core/shell.ts` enthalten kein deutsches Text-Literal mehr (Test wie `textRule.test.ts`); die Texte stehen in `texts.de.ts` und `texts.en.ts`.
- **AC-02** Nach Sprachwechsel in den Optionen zeigen Home-Button und Touch-Tasten nach dem Neuladen die gewählte Sprache (🧑 am Gerät).

## Offene Fragen

Bleiben die Werkzeug-Seiten (`src/tools/`) deutsch? Entscheidet 🧑.

## Notizen

–
