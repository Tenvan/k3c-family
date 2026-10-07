# B-336 · Die Optionen-Szene lässt sich per Touch vollständig bedienen und schließen, ohne vom Touch-Overlay verdeckt zu werden

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei der Abnahme S5.4 (2026-10-07, PC, Maus, `game.html?touch=1`) hat 🧑 festgestellt: Der Knopf „☰ Optionen“ öffnet die Optionen-Szene, sie lässt sich per Touch aber nicht schließen, und das Touch-Overlay liegt über dem Menü. Vermutete Ursache (ungeprüft): Das Overlay (`src/input/touchInput.ts`, `.k3c-zone` z-index 9 über den ganzen Bildschirm, `.k3c-touch` z-index 10 unten) bleibt bei offener Szene sichtbar und verdeckt die unteren Zeilen, darunter „Weiter“ (`resume`), die einzige Touch-Möglichkeit zum Schließen (`src/scenes/OptionsScene.ts`, `optionsLogic.ts`). Der Knopf „☰ Optionen“ (`src/scenes/pauseButton.ts`) schließt die Szene nicht.

## Ziel

Am Handy (Touch) lassen sich Optionen und Pause öffnen, alle Einträge bedienen und wieder schließen, ohne dass Spiel-Bedienelemente das Menü verdecken.

## Beteiligte und Zielgruppen

Spieler am Handy oder Tablet; 🧑 nimmt ab (S5.4, AC-05).

## Anforderungen

- Bei offener Optionen-Szene ist das Touch-Overlay ausgeblendet oder liegt unter dem Menü; kein Eintrag ist verdeckt.
- Jeder Eintrag einschließlich „Weiter“ und „Verlassen“ ist per Tippen erreichbar.
- Die Szene lässt sich per Touch schließen (über „Weiter“; zusätzlich darf der ☰-Knopf sie schließen).
- Nach dem Schließen ist das Overlay wieder da und bedienbar.
- Der Knopf „☰ Optionen“ verdeckt keinen HUD-Text (heute die Zeile „Raum … · n Spieler“).

## Nicht-Ziele

Neue Optionen; Gestaltung des Touch-Overlays außerhalb der Szene; Controller (B-314).

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/`); berührt die Lösung `src/input/touchInput.ts` (PLAT), wird das beim Einplanen als Grenzfall oder eigene Session geklärt. `docs/rules/bedienung.md` › Pause; B unbelegt, View + Menu reserviert. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Handy: „☰ Optionen“ tippen → Menü vollständig sichtbar, Overlay weg → Musik ändern → „Weiter“ tippen → Szene zu, Overlay wieder da.

## Ausnahme- und Fehlerfälle

Kleiner Bildschirm im Querformat → alle Zeilen bleiben erreichbar (ggf. kompakter). Wechsel auf Tastatur oder Controller bei offener Szene → Bedienung wie heute.

## Akzeptanzkriterien

- **AC-01** Bei offener Optionen-Szene verdeckt das Touch-Overlay keinen Eintrag (Test der Sichtbarkeitslogik oder Nachweis im Browser-Pane mit `?touch=1`).
- **AC-02** Die Szene schließt per Touch über „Weiter“ (Nachweis im Browser-Pane mit `?touch=1`).
- **AC-03** 🧑 hat Öffnen, Bedienen und Schließen per Touch abgenommen (PC mit `?touch=1` oder Handy).
- **AC-04** Der Knopf „☰ Optionen“ verdeckt im Spiel keinen HUD-Text (Nachweis im Browser-Pane mit `?touch=1`).

## Offene Fragen

- Soll der ☰-Knopf die Szene auch schließen (Umschalter)? 🧑, nicht blockierend.

## Notizen

Befund 🧑 bei S5.4 (2026-10-07): „Optionen öffnet nur, schließt aber nicht. Menü wird von Touch-Overlay überlagert.“

Screenshots 🧑 (2026-10-07, PC, `?touch=1`): Bei offener Szene liegen die Touch-Knöpfe (⛶, 1–4, », ★, Schlag, Münze) über den Zeilen „Weiter“ und „Spiel verlassen“ und über der Hinweiszeile; „Weiter“ ist nur halb sichtbar, „Spiel verlassen“ fast ganz verdeckt. Nebenbefund: Der Knopf „☰ Optionen“ (oben links) verdeckt im Spiel die HUD-Zeile „Raum … · … · n Spieler“; das gehört mit zu diesem Ticket (Anforderung: kein HUD-Text verdeckt).
