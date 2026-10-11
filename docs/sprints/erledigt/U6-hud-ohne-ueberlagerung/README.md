# U6 · CLI · HUD ohne Überlagerung, Optionen per Touch

- **Status:** erledigt
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-337
- **Start-Commit:** f77ff97a
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-11, 🧑 im Chat, Revision 2 (Touch, Handy und 3–4 Spieler entfallen)

## Ausgangslage

HUD-Anzeigen und HTML-Elemente (Home-Button, „☰ Optionen“, Touch-Overlay, Debug-Overlay) sitzen je für sich an festen Positionen, nichts verhindert Überlagerungen. Mit `?touch=1` verdeckt das Touch-Overlay die Optionen-Szene, die sich per Touch nicht schließen lässt (B-336), und „☰ Optionen“ verdeckt die Raum-Zeile (B-337). Seit 2026-10-07 ruhen alle Anzeige-, Touch- und Tasten-Abnahmen bis zur Gesamtabnahme (B-337/AC-05).

## Ziel

Das HUD besteht aus abgegrenzten Elementen, die sich in keinem Layout überlagern. Am Ende sichtbar: Bei 1 bis 2 Spielern am PC verdeckt nichts im HUD ein anderes Element. Danach steht die Gesamtabnahme Anzeige und Bedienung für PC und Xbox. (Optionen per Touch und 3–4 Spieler entfallen seit 2026-10-11, B-336 ist verworfen; der umgesetzte Code bleibt.)

## Beteiligte und Zielgruppen

Spieler am PC (Tastatur) und an der Xbox (Controller), höchstens 2 Spieler. 🧑 entscheidet über den Stil (Hintergrund, Rahmen) und die Rangfolge beim Ausblenden und macht die Gesamtabnahme.

## Anforderungen

B-336 › Anforderungen; B-337 › Anforderungen. Reihenfolge: zuerst B-336 (Optionen per Touch), danach führt B-337 die Layout-Funktion ein und übernimmt die Freifläche für „☰ Optionen“.

## Nicht-Ziele

Neue HUD-Inhalte oder Optionen; Grafikstil-Pipeline (B-331); Spielwelt-Beschriftungen; Controller-Bedienung (B-314); Debug-Overlay bedienbar machen (B-191, U5): U6 hält dem Debug-Overlay nur einen Bereich frei.

## Regeln und Einschränkungen

- Domäne CLI (`src/scenes/`). `src/core/shell.ts` und `src/input/touchInput.ts` (PLAT) werden nur gelesen. Muss B-336 das Overlay selbst ändern (Ausblenden bei offener Szene), klärt U6.1 den Grenzfall vor dem Umsetzen und hält ihn im Session-Ergebnis fest.
- Nur zeichnen, keine Spiel-Logik (ADR 001). Layout-Logik Phaser-frei mit Vitest-Tests, Texte über `t()`.
- Mindest-Schriftgröße aus `docs/rules/bedienung.md` § 2 (B-136). B unbelegt, View + Menu reserviert.
- Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit.

## Beispiele

B-336 › Beispiele; B-337 › Beispiele.

## Ausnahme- und Fehlerfälle

B-336 › Ausnahme- und Fehlerfälle; B-337 › Ausnahme- und Fehlerfälle (sehr kleine Zelle: Rangfolge blendet aus, Gold und HP bleiben; Fensteränderung: neu berechnen, kein Springen).

## Akzeptanzkriterien

- **AC-01** Bei offener Optionen-Szene verdeckt das Touch-Overlay keinen Eintrag (`B-336/AC-01`). Umgesetzt und getestet in U6.1; die Abnahme per Touch entfällt (2026-10-11).
- **AC-02** Die Szene schließt per Touch über „Weiter“ (`B-336/AC-02`). Umgesetzt in U6.1; die Abnahme per Touch entfällt.
- **AC-03** Der Knopf „☰ Optionen“ verdeckt keinen HUD-Text (`B-336/AC-04`). Umgesetzt in U6.1; die Abnahme entfällt.
- **AC-04** Die Layout-Funktion liefert für die Layouts mit 1–2 Spielern, mit und ohne Debug, Rechtecke ohne Überschneidung untereinander und mit den Freiflächen (`B-337/AC-01`, Test). Die Layouts für 3–4 Spieler und Touch sind umgesetzt, werden aber nicht abgenommen.
- **AC-05** Alle HUD-Anzeigen laufen über diese Elemente, Hintergrund und Rahmen sind je Element schaltbar (`B-337/AC-02`).
- **AC-06** Die Mindest-Schriftgröße bleibt je Layout eingehalten (`B-337/AC-03`, `fontRules.test.ts` grün).
- **AC-07** 🧑 hat am PC (1 und 2 Spieler) keine Überlagerung gesehen (`B-337/AC-04`).
- **AC-08** 🧑 hat die Prüfliste der Gesamtabnahme für PC und Xbox durchlaufen, Mängel als Tickets (`B-337/AC-05`).

## Offene Fragen

- Stil von Hintergrund und Rahmen (Deckkraft, Farbe, Ecken; passend zu B-331): 🧑, beantwortet 2026-10-10 im Chat: Schwarz mit 55 % Deckkraft, Ecken 8 px, Innenabstand 8/4 px, ohne Rahmen; Hintergrund und Rahmen je Element schaltbar.
- Rangfolge beim Ausblenden (Pflichtanzeigen): 🧑, beantwortet 2026-10-10 im Chat: Pflicht sind Spielerzeile (Gold, HP), Meldung (Banner) und Skill-Leiste; danach gemeinsamer Block (Vorräte, Uhr, Kampf) > Reise > Beitritt > Radar > Raum-Zeile > Steuerhinweis (fällt zuerst weg).
- Soll der ☰-Knopf die Szene auch schließen (Umschalter)? 🧑, nicht blockierend.
- Gerät für die Gesamtabnahme (AC-08): je Durchgang vorher erfragen; Xbox und Controller erst nach B-314.
- Grenzfall Touch-Overlay (U6.1): Vorschlag: Die Optionen-Szene blendet `.k3c-zone` und `.k3c-touch` über ihre Klassen per `style.display` aus (CLI), `src/input/touchInput.ts` bleibt unverändert; Alternative wäre eine PLAT-Session mit `TouchInput.show()`. Übernommen mit der Freigabe 2026-10-09.
- Browser-Pane für U6.1 (AC-01 bis AC-03 verlangen Nachweise mit `?touch=1`): 🧑 gibt sie je Lauf frei, sonst ist U6.1 blockiert.
- Nur bildschirmfeste Anzeigen werden HUD-Elemente; Aktionen-Overlay, geführte Hinweise und Preisschilder bleiben weltgebunden (Vorschlag nach B-337 › Nicht-Ziele). Übernommen mit der Freigabe 2026-10-09.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| U6.1 | `U6.1-optionen-per-touch.md` | Umsetzung | autonom | fertig |
| U6.2 | `U6.2-hud-elemente-layout.md` | Umsetzung | autonom | fertig |
| U6.3 | `U6.3-review.md` | Review | autonom | fertig |
| U6.4 | `U6.4-abnahme-gesamt.md` | Workshop | Mensch | fertig |

## Abnahme

2026-10-10 (U6.3): AC-01 bis AC-03 laut U6.1 › Ergebnis, AC-04 bis AC-06 laut U6.2 › Ergebnis (AC-05: Radar mit eigenem Hintergrund, Skill-Menü kein Element, offengelegt); Review ohne schwere Befunde.
2026-10-11 (U6.4, 🧑 am PC): AC-07 erfüllt (1 Spieler, 2 und 4 Mock-Spieler, zwei echte Spieler in verschiedenen Stufen, mit Debug-Overlay, keine Überlagerung). AC-08 erfüllt: Prüfliste B-337 für PC mit je Punkt Ergebnis; Mängel B-270 (Respec fehlt in `actions`), B-398 (Gold im HUD statt über dem Reiter), B-399 (Steuerhinweis verschwindet); Punkt 6a (Hub- und Materialstufen) nicht geprüft (B-117, B-342, B-315); Controller und Xbox (Punkt 7, 8, Teile von 1 und 2) nicht geprüft bis B-314.
Touch, Handy und 3–4 Spieler entfallen (Beschluss 🧑 2026-10-11, Revision 2); B-336 verworfen, B-337 erledigt.
Version: v0.19.0 vorgeschlagen (Minor: HUD-Layout im Spiel, Optionen per Touch; nach den offenen Vorschlägen bis v0.18.0); nicht gesetzt (wartet auf Bestätigung 🧑).
