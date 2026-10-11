# B-337 · Jede HUD-Anzeige ist ein eigenes Element mit optionalem Hintergrund und Rahmen, und HUD-Elemente überlagern sich nicht

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** U6
- **Projekt:** BED
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-11, 🧑 im Chat, Revision 2 (3–4 Spieler, Touch und Handy entfallen)

## Ausgangslage

Die HUD-Anzeigen (`src/scenes/HudScene.ts` und Helfer: Spielerzeile mit Gold/HP/Stufe, Raum-Zeile, Vorräte, Tag/Phase, Hinweise, Aktionsleiste, Radar) und die HTML-Elemente darüber (Home-Button der Shell, „☰ Optionen“ aus `pauseButton.ts`, Touch-Overlay, Debug-Overlay) werden je für sich an feste Positionen gesetzt. Nichts verhindert Überlagerungen. Bei den Abnahmen am 2026-10-07 verdeckte „☰ Optionen“ die Raum-Zeile, das Touch-Overlay die Optionen-Szene (B-336), das Debug-Overlay das HUD (B-191); Texte ohne Hintergrund sind über hellem Himmel schlecht lesbar.

## Ziel

Das HUD besteht aus klar abgegrenzten Elementen, die sich in jedem Layout (1–4 Spieler, Touch, Debug) nicht überlagern und auf jedem Hintergrund lesbar sind.

## Beteiligte und Zielgruppen

Spieler am TV, PC und Handy; 🧑 entscheidet über Aussehen (Hintergrund, Rahmen) und nimmt ab.

## Anforderungen

- Jede HUD-Anzeige ist ein eigenes Element mit Inhalt, Ankerplatz (z. B. oben links, oben rechts, unten Mitte) und optionalem Hintergrund und Rahmen (einheitlicher Stil).
- Eine reine, getestete Layout-Funktion ordnet die Elemente je Zelle bzw. Bildschirm an und verhindert Überlagerungen: Elemente am selben Anker stapeln sich, freizuhaltende Bereiche (Home-Button oben mittig ca. 70 px, „☰ Optionen“, Debug-Overlay) werden ausgespart.
- Gilt für Split-Screen mit 1–2 Spielern (je Zelle) und hält die Mindest-Schriftgröße aus `docs/rules/bedienung.md` § 2 (B-136). Geprüft werden PC (Tastatur) und Xbox (Controller); 3–4 Spieler, Touch und Handy sind seit 2026-10-11 ausgenommen.
- Passt ein Element nicht, wird es gekürzt oder ausgeblendet nach fester Rangfolge, nie überdeckt.

## Nicht-Ziele

Neue HUD-Inhalte; Grafikstil-Pipeline (B-331); Inhalte der Optionen-Szene (B-336, löst dort die Touch-Überlagerung zuerst); Spielwelt-Beschriftungen wie Preisschilder an Bauplätzen.

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/`); Freiflächen für Shell-Elemente (`src/core/shell.ts`) und Touch-Overlay (`src/input/`, PLAT) werden nur gelesen, nicht geändert. Nur zeichnen, keine Spiel-Logik (ADR 001). Layout-Logik Phaser-frei mit Vitest-Tests. Texte über `t()`. Datei ≤ 400, Funktion ≤ 60 Zeilen. Keine neue Abhängigkeit.

## Beispiele

- 1 Spieler: Spielerzeile und Raum-Zeile stehen ohne Überdeckung neben Home-Button und „☰ Optionen“.
- 2 Spieler: je Zelle Spielerzeile oben links, Vorräte oben rechts, jeweils mit halbtransparentem Hintergrund; ist die Zelle zu schmal, wird die Raum-Zeile ausgeblendet.
- Debug-Overlay an: es bekommt einen eigenen Bereich, das HUD weicht aus (B-191).

## Ausnahme- und Fehlerfälle

Sehr kleine Zelle (2 Spieler, kleines Fenster) → Rangfolge blendet unwichtige Elemente aus, Pflichtanzeigen (Gold, HP) bleiben. Fensteränderung → Layout neu berechnen, kein Springen über mehrere Frames.

## Akzeptanzkriterien

- **AC-01** Die Layout-Funktion liefert für die Layouts mit 1–2 Spielern, mit und ohne Debug, Rechtecke ohne Überschneidung untereinander und mit den freizuhaltenden Bereichen (Test).
- **AC-02** Alle HUD-Anzeigen laufen über diese Elemente; Hintergrund und Rahmen sind je Element schaltbar (Test oder Code-Nachweis).
- **AC-03** Mindest-Schriftgröße je Layout bleibt eingehalten (`fontRules.test.ts` grün).
- **AC-04** 🧑 hat am PC (1 und 2 Spieler) keine Überlagerung im HUD gesehen.
- **AC-05** Gesamtabnahme Anzeige und Bedienung: 🧑 hat die Prüfliste unten (Notizen › Prüfliste Gesamtabnahme) für PC und Xbox vollständig durchlaufen; Ergebnis je Punkt mit Datum und Gerät, Mängel als Tickets. Ersetzt die am 2026-10-07 verworfenen Abnahmen S3.4, S4.3, S5.4, S6.4, S7.3, GR3.4, GR4.3, GR5.4.

## Offene Fragen

- Stil von Hintergrund und Rahmen sowie Rangfolge beim Ausblenden: entschieden 2026-10-10 (🧑, siehe Sprint U6).
- Entschieden 2026-10-11 (🧑, Chat): 3–4 Spieler, Touch und Handy entfallen aus Anforderungen und Abnahme; es zählen PC (Tastatur) und Xbox (Controller) mit höchstens 2 Spielern. Eigene Abnahmen dafür legt 🧑 später bei Bedarf neu an.

## Notizen

Vorschlag 🧑 (2026-10-07, nach S5.4/B-336): „Alle HUD-Anzeigen sollten eigene Elemente (mit optional Hintergrund und Rahmen) sein, bei denen Überlagerungen vermieden werden.“ Verwandt: B-191, B-090 (Radar), B-136 (Mindest-Schrift). B-336 (Touch) ist 2026-10-11 verworfen.

### Prüfliste Gesamtabnahme (AC-05)

Entscheidung 🧑 (2026-10-07): Bis B-337 umgesetzt ist, keine weiteren Anzeige- und Tasten-Abnahmen; alle bisherigen und offenen sind geschlossen, diese Liste ersetzt sie. Seit 2026-10-11 nur PC (Tastatur) und Xbox (Controller), höchstens 2 Spieler; Touch, Handy und 3–4 Spieler entfallen. Gerät je Durchgang vorher festhalten (PC mit Tastatur; Xbox/TV und Controller nach B-314). Je Punkt: grün, Mangel (Ticket) oder nicht geprüft.

1. **HUD und Layouts** (aus S4.3, B-106): 1–2 Spieler (Testseite mit Mock-Spielern); keine Überlagerung (AC-01 bis AC-04); zwei Spieler in verschiedenen Stufen: Stufe, Radar und HUD je Zelle, Nacht je Zelle; lesbar am TV aus 2–3 m.
2. **Optionen und Pause** (aus S5.4, B-146, B-172): öffnen und schließen mit Tastatur (Esc, Pos1) und Controller (Menu kurz, View + Menu, B ohne Wirkung); alle Einträge einstellen; Werte nach Neuladen erhalten; Englisch lesen, Sprache bleibt.
3. **Schlag, Skills, Skill-Menü, Aktionen-Overlay** (aus S3.4, B-124, B-125; Mängel B-318, B-319): je Gerät Schlag, Skill-Slots, Skill-Menü öffnen, Punkt lernen, Respec an der Burg; Overlay-Hinweise passend zum Gerät, ein Hinweis je Spieler, zwei Spieler lesbar.
4. **Onboarding und Glyphen** (aus S6.4, B-148, B-149): neuer Raum im Grad „leicht“, Hinweise zurückgesetzt, erste Nacht ohne Erklärung (am TV möglichst mit Kind); Hinweise verstanden, Glyphen lesbar, auch im Split-Screen.
5. **Monarch auf dem Reittier** (aus S7.3, B-173; Mangel B-320): stehen, laufen links/rechts, sprinten; Reiter bleibt im Sattel, Goldbeutel und HP über dem Reiter; zwei Spieler im Split-Screen; gefallener Monarch kommt beritten zurück.
6. **Grafik im Renderer** (aus GR3.4, B-010): Hub-Stufen, Mauer- und Turm-Materialstufen unterscheidbar; Parallax je Biom ohne Darstellungsfehler; zwei Spieler im Split-Screen.
7. **Juice** (aus GR5.4, B-164): Treffer, Kill, Münze aufheben und geben, Bau fertig, Tod sichtbar; Screenshake nur in der Kamera des Betroffenen; mit Screenshake und Blitz „aus“ ruhig; Vibration (Controller, B-314).
8. **Lade-Szene und Kaltstart** (aus GR4.3, B-163): Zeit vom Öffnen bis zum Menü dreimal messen (Cache geleert), Budget festlegen; auf der Xbox `MAX_TEXTURE_SIZE` ablesen (angenommen 4096).
9. **Pause-Anzeige** (B-333): angehaltener Raum zeigt eine Anzeige, Figuren-Animationen stehen. Grün am 2026-10-11 (PC, 2 Spieler, AZ1.4).
