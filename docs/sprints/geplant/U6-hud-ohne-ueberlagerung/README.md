# U6 · CLI · HUD ohne Überlagerung, Optionen per Touch

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-336, B-337
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-09, 🧑 im Chat, Revision 1, Vorschläge unter Offene Fragen übernommen

## Ausgangslage

HUD-Anzeigen und HTML-Elemente (Home-Button, „☰ Optionen“, Touch-Overlay, Debug-Overlay) sitzen je für sich an festen Positionen, nichts verhindert Überlagerungen. Mit `?touch=1` verdeckt das Touch-Overlay die Optionen-Szene, die sich per Touch nicht schließen lässt (B-336), und „☰ Optionen“ verdeckt die Raum-Zeile (B-337). Seit 2026-10-07 ruhen alle Anzeige-, Touch- und Tasten-Abnahmen bis zur Gesamtabnahme (B-337/AC-05).

## Ziel

Das HUD besteht aus abgegrenzten Elementen, die sich in keinem Layout überlagern, und Optionen und Pause sind per Touch vollständig bedienbar. Am Ende sichtbar: Am PC mit `?touch=1` öffnet, bedient und schließt man die Optionen per Tippen, und bei 1 bis 4 Spielern verdeckt nichts im HUD ein anderes Element. Danach steht die Gesamtabnahme Anzeige und Bedienung.

## Beteiligte und Zielgruppen

Spieler am TV, PC und Handy. 🧑 entscheidet über den Stil (Hintergrund, Rahmen) und die Rangfolge beim Ausblenden und macht die Gesamtabnahme.

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

- **AC-01** Bei offener Optionen-Szene verdeckt das Touch-Overlay keinen Eintrag (`B-336/AC-01`).
- **AC-02** Die Szene schließt per Touch über „Weiter“ (`B-336/AC-02`, Browser-Pane mit `?touch=1`).
- **AC-03** Der Knopf „☰ Optionen“ verdeckt keinen HUD-Text (`B-336/AC-04`).
- **AC-04** Die Layout-Funktion liefert für 1–4 Spieler, mit und ohne Touch und Debug, Rechtecke ohne Überschneidung untereinander und mit den Freiflächen (`B-337/AC-01`, Test).
- **AC-05** Alle HUD-Anzeigen laufen über diese Elemente, Hintergrund und Rahmen sind je Element schaltbar (`B-337/AC-02`).
- **AC-06** Die Mindest-Schriftgröße bleibt je Layout eingehalten (`B-337/AC-03`, `fontRules.test.ts` grün).
- **AC-07** 🧑 hat Optionen per Touch abgenommen und am PC (1 und 4 Spieler, `?touch=1`) keine Überlagerung gesehen (`B-336/AC-03`, `B-337/AC-04`).
- **AC-08** 🧑 hat die Prüfliste der Gesamtabnahme durchlaufen, Mängel als Tickets (`B-337/AC-05`).

## Offene Fragen

- Stil von Hintergrund und Rahmen (Deckkraft, Farbe, Ecken; passend zu B-331): 🧑, blockiert U6.2 (bei der Freigabe 2026-10-09 ohne Vorschlag, weiter offen).
- Rangfolge beim Ausblenden (Pflichtanzeigen): 🧑, blockiert U6.2 (bei der Freigabe 2026-10-09 ohne Vorschlag, weiter offen).
- Soll der ☰-Knopf die Szene auch schließen (Umschalter)? 🧑, nicht blockierend.
- Gerät für die Gesamtabnahme (AC-08): je Durchgang vorher erfragen; Xbox und Controller erst nach B-314.
- Grenzfall Touch-Overlay (U6.1): Vorschlag: Die Optionen-Szene blendet `.k3c-zone` und `.k3c-touch` über ihre Klassen per `style.display` aus (CLI), `src/input/touchInput.ts` bleibt unverändert; Alternative wäre eine PLAT-Session mit `TouchInput.show()`. Übernommen mit der Freigabe 2026-10-09.
- Browser-Pane für U6.1 (AC-01 bis AC-03 verlangen Nachweise mit `?touch=1`): 🧑 gibt sie je Lauf frei, sonst ist U6.1 blockiert.
- Nur bildschirmfeste Anzeigen werden HUD-Elemente; Aktionen-Overlay, geführte Hinweise und Preisschilder bleiben weltgebunden (Vorschlag nach B-337 › Nicht-Ziele). Übernommen mit der Freigabe 2026-10-09.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| U6.1 | `U6.1-optionen-per-touch.md` | Umsetzung | autonom | offen |
| U6.2 | `U6.2-hud-elemente-layout.md` | Umsetzung | autonom | offen |
| U6.3 | `U6.3-review.md` | Review | autonom | offen |
| U6.4 | `U6.4-abnahme-gesamt.md` | Workshop | Mensch | offen |

## Abnahme

–
