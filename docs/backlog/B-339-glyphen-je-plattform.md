# B-339 · Spiel und Seiten zeigen Tastensymbole passend zum gerade benutzten Controller (Xbox, PlayStation, weitere)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Glyphen im Spiel (`src/scenes/glyphs.ts`, `glyphView.ts`, B-149) kennen nur die Geräte-Arten `pad`, `keyboard`, `touch` (`src/input/slotBindings.ts` › `Device`); ein Pad wird immer mit Xbox-Beschriftung und -Farben gezeichnet (`GLYPH_COLORS` A grün, X blau, Y gelb; LB/RB/LT/RT, View/Menu). Die Seiten tun dasselbe: Landingpage-Fußleiste (`index.html`, `.glyph.a` „A“, „Y“, „View + Menu“), Home-Button der Shell („View+Menu“), Hinweise der Werkzeug-Seiten. Der Browser liefert je Pad `Gamepad.id` (z. B. „Xbox Wireless Controller“, „DualSense Wireless Controller (STANDARD GAMEPAD Vendor: 054c …)“); ausgewertet wird das nirgends. 🧑 (2026-10-07): „Kann man das nicht auf die gerade laufende Plattform anpassen? Später z. B. noch PlayStation oder was sonst noch so geht?“

## Ziel

Wer mit einem PlayStation-, Switch- oder anderen Controller spielt, sieht dessen Tastensymbole statt der Xbox-Symbole, im Spiel und auf den Seiten; Tastatur und Touch bleiben wie heute.

## Beteiligte und Zielgruppen

Spieler am PC oder TV mit verschiedenen Controllern; Xbox bleibt Zielplattform und Standard. 🧑 entscheidet die unterstützten Familien und nimmt ab (nach B-337 und B-314).

## Anforderungen

- Eine reine, getestete Funktion ordnet `Gamepad.id` (und ggf. Vendor-ID) einer Controller-Familie zu: `xbox` (Standard und Rückfall), `playstation`, weitere nach Beschluss (z. B. `switch`, `generic`).
- Je Familie ein Symbolsatz: Beschriftung und Farben der vier Haupttasten (Xbox A/B/X/Y; PlayStation ✕/○/□/△), Schultertasten (LB/RB/LT/RT ↔ L1/R1/L2/R2), View/Menu ↔ Share/Options bzw. Create/Options.
- Maßgeblich ist der zuletzt benutzte Controller **je Spieler** (Split-Screen mit gemischten Pads zeigt je Zelle die passenden Symbole); auf Seiten der zuletzt benutzte Controller des Geräts.
- Gilt für Glyphen im Spiel und für Hinweise auf Landingpage, Shell (Home-Button) und Werkzeug-Seiten.
- Belegung bleibt gleich (Standard-Mapping); nur die Darstellung ändert sich. B bzw. ○ bleibt unbelegt.

## Nicht-Ziele

Andere Tastenbelegung je Plattform; Controller ohne `mapping: 'standard'` unterstützen; Bilddateien aus Hersteller-Packs mit unklarer Lizenz (Symbole selbst zeichnen oder CC0).

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/glyphs*.ts`); Erkennung und Weitergabe der Familie in `src/input/` sowie die Seiten-Hinweise (`index.html`, `src/core/shell.ts`, `src/tools/`) sind PLAT und werden beim Einplanen als eigene Session oder eigenes Ticket geführt. Kontrast ≥ 4,5:1 (bestehender Test), Mindest-Schriftgröße (B-136). Erst nach B-337 (HUD-Elemente) und mit Controller-Abnahme nach B-314 sinnvoll prüfbar.

## Beispiele

- DualSense am PC: Hinweis „✕ halten: Werkstatt bauen“ statt „A halten …“; Landingpage-Fußleiste zeigt ✕ und „Create + Options“.
- Spieler 1 Xbox, Spieler 2 DualSense im Split-Screen: oben A/X/Y, unten ✕/□/△.
- Unbekannter Controller („USB Gamepad“) → Xbox-Symbole wie heute.

## Ausnahme- und Fehlerfälle

`Gamepad.id` leer oder unbekannt → Xbox (Rückfall). Wechsel des Controllers mitten im Spiel → Symbole wechseln mit der nächsten Eingabe, ohne Flackern. Edge auf der Xbox meldet Xbox-Pads → unverändert.

## Akzeptanzkriterien

- **AC-01** Die Zuordnung `Gamepad.id` → Familie ist getestet (Xbox-, DualShock/DualSense- und unbekannte IDs; Rückfall Xbox).
- **AC-02** Für jede Familie liefert die Glyph-Funktion Beschriftung und Farben aller Glyph-Tasten; Kontrast-Test grün.
- **AC-03** Im Spiel zeigt jede Zelle die Symbole des Controllers ihres Spielers (Test der Zuordnung Spieler → Familie; Nachweis im Browser-Pane mit gemocktem Pad).
- **AC-04** Landingpage, Shell und Werkzeug-Seiten zeigen die Symbole des zuletzt benutzten Controllers (Nachweis im Browser-Pane).
- **AC-05** 🧑 hat mit einem PlayStation-Controller am PC abgenommen (nach B-314).

## Offene Fragen

- Welche Familien außer Xbox und PlayStation (Switch Pro, 8BitDo, Steam Controller, generisch)? 🧑
- Symbole selbst zeichnen (Kreis + Zeichen, wie heute) oder ein CC0-Pack (z. B. Kenney Input Prompts) nutzen? 🧑
- Soll auch die Tastatur je Layout (DE/US) beschriftet werden? 🧑, nicht blockierend.

## Notizen

Erkennung über `Gamepad.id`: Chrome/Edge melden Text plus `Vendor: xxxx Product: yyyy` (Sony 054c, Microsoft 045e, Nintendo 057e). Verwandt: B-149 (Glyphen, erledigt), B-337 (HUD-Elemente), B-314 (Controller-Prüfungen).
