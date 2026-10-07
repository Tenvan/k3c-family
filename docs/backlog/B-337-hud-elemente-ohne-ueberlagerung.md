# B-337 · Jede HUD-Anzeige ist ein eigenes Element mit optionalem Hintergrund und Rahmen, und HUD-Elemente überlagern sich nicht

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die HUD-Anzeigen (`src/scenes/HudScene.ts` und Helfer: Spielerzeile mit Gold/HP/Stufe, Raum-Zeile, Vorräte, Tag/Phase, Hinweise, Aktionsleiste, Radar) und die HTML-Elemente darüber (Home-Button der Shell, „☰ Optionen“ aus `pauseButton.ts`, Touch-Overlay, Debug-Overlay) werden je für sich an feste Positionen gesetzt. Nichts verhindert Überlagerungen. Bei den Abnahmen am 2026-10-07 verdeckte „☰ Optionen“ die Raum-Zeile, das Touch-Overlay die Optionen-Szene (B-336), das Debug-Overlay das HUD (B-191); Texte ohne Hintergrund sind über hellem Himmel schlecht lesbar.

## Ziel

Das HUD besteht aus klar abgegrenzten Elementen, die sich in jedem Layout (1–4 Spieler, Touch, Debug) nicht überlagern und auf jedem Hintergrund lesbar sind.

## Beteiligte und Zielgruppen

Spieler am TV, PC und Handy; 🧑 entscheidet über Aussehen (Hintergrund, Rahmen) und nimmt ab.

## Anforderungen

- Jede HUD-Anzeige ist ein eigenes Element mit Inhalt, Ankerplatz (z. B. oben links, oben rechts, unten Mitte) und optionalem Hintergrund und Rahmen (einheitlicher Stil).
- Eine reine, getestete Layout-Funktion ordnet die Elemente je Zelle bzw. Bildschirm an und verhindert Überlagerungen: Elemente am selben Anker stapeln sich, freizuhaltende Bereiche (Home-Button oben mittig ca. 70 px, „☰ Optionen“, Touch-Overlay, Debug-Overlay) werden ausgespart.
- Gilt für Split-Screen 1–4 (je Zelle) und hält die Mindest-Schriftgröße aus `docs/rules/bedienung.md` § 2 (B-136).
- Passt ein Element nicht, wird es gekürzt oder ausgeblendet nach fester Rangfolge, nie überdeckt.

## Nicht-Ziele

Neue HUD-Inhalte; Grafikstil-Pipeline (B-331); Inhalte der Optionen-Szene (B-336, löst dort die Touch-Überlagerung zuerst); Spielwelt-Beschriftungen wie Preisschilder an Bauplätzen.

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/`); Freiflächen für Shell-Elemente (`src/core/shell.ts`) und Touch-Overlay (`src/input/`, PLAT) werden nur gelesen, nicht geändert. Nur zeichnen, keine Spiel-Logik (ADR 001). Layout-Logik Phaser-frei mit Vitest-Tests. Texte über `t()`. Datei ≤ 400, Funktion ≤ 60 Zeilen. Keine neue Abhängigkeit.

## Beispiele

- Touch an, 1 Spieler: „☰ Optionen“ oben links → Spielerzeile und Raum-Zeile rücken daneben bzw. darunter, nichts verdeckt.
- 4 Spieler (2×2): je Viertel Spielerzeile oben links, Vorräte oben rechts, jeweils mit halbtransparentem Hintergrund; ist die Zelle zu schmal, wird die Raum-Zeile ausgeblendet.
- Debug-Overlay an: es bekommt einen eigenen Bereich, das HUD weicht aus (B-191).

## Ausnahme- und Fehlerfälle

Sehr kleine Zelle (Handy, 4 Spieler) → Rangfolge blendet unwichtige Elemente aus, Pflichtanzeigen (Gold, HP) bleiben. Fensteränderung → Layout neu berechnen, kein Springen über mehrere Frames.

## Akzeptanzkriterien

- **AC-01** Die Layout-Funktion liefert für Layouts 1–4, mit und ohne Touch und Debug, Rechtecke ohne Überschneidung untereinander und mit den freizuhaltenden Bereichen (Test).
- **AC-02** Alle HUD-Anzeigen laufen über diese Elemente; Hintergrund und Rahmen sind je Element schaltbar (Test oder Code-Nachweis).
- **AC-03** Mindest-Schriftgröße je Layout bleibt eingehalten (`fontRules.test.ts` grün).
- **AC-04** 🧑 hat am PC (1 und 4 Spieler, `?touch=1`) keine Überlagerung im HUD gesehen.

## Offene Fragen

- Stil von Hintergrund und Rahmen (Deckkraft, Farbe, Ecken; passend zu B-331): 🧑.
- Rangfolge beim Ausblenden (welche Anzeigen sind Pflicht): 🧑.
- B-191 in dieses Ticket aufnehmen oder davor/danach umsetzen? Beim Einplanen klären.

## Notizen

Vorschlag 🧑 (2026-10-07, nach S5.4/B-336): „Alle HUD-Anzeigen sollten eigene Elemente (mit optional Hintergrund und Rahmen) sein, bei denen Überlagerungen vermieden werden.“ Verwandt: B-191, B-336 (AC-04), B-090 (Radar), B-136 (Mindest-Schrift).
