# B-411 · Lokale Spieler können beitreten, aber nicht einzeln austreten

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BD4
- **Projekt:** BED
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-13, Chat (Ralf), Revision 2

## Ausgangslage

Teilweise umgesetzt (Commit 883123a1, #274): Das Spielmenü hat „Spieler abmelden“ (`OptionId` `removePlayer`, `src/scenes/optionsLogic.ts`), das über `LocalSlots.removeLast` (`src/scenes/localSlots.ts`) immer den **höchsten** Slot per `removeSlot` entfernt; bei nur einem Platz passiert nichts. Das Menü weiß nicht, wer es geöffnet hat: `GameScene.wantsOptions` (`src/scenes/GameScene.ts`) prüft `pads.some(…)`, `keyboards[0]` und die Touch-Schaltfläche, ohne sich die auslösende Eingabe zu merken. Außerdem geht `removeSlot` beim Trennen eines Controllers raus (`LocalSlots.lose`). Der Server kann `removeSlot` schon.

## Ziel

Jeder lokale Spieler kann das Spiel für sich allein verlassen, ohne Controller abzuziehen und ohne die anderen am Gerät zu stören.

## Beteiligte und Zielgruppen

Spielende am selben Gerät (Couch, Split-Screen 2–4); Agent in CLI; 🧑 nimmt am Gerät ab.

## Anforderungen

- Beim Öffnen des Spielmenüs merkt sich `GameScene` die auslösende Eingabe (Pad mit Menu kurz, Tastatur 1 mit Esc, Touch-Schaltfläche) und gibt deren Slot an das Menü weiter (Öffner).
- Tastatur 2 (Pfeile, Enter, Ziffernblock) hat keine eigene Menü-Taste. Spielen zwei Tastatur-Spieler und öffnet Esc das Menü, fragt „Spieler abmelden“ zuerst, welcher der beiden sich abmeldet (Auswahl mit links/rechts, Bestätigen); andere Öffner treffen ohne Auswahl ihren eigenen Slot.
- „Spieler abmelden“ entfernt genau den gewählten Slot: `removeSlot`, Eingabe frei, Feld weg, der Split-Screen ordnet sich neu; der Monarch verlässt den Raum nach Server-Regel. `removeLast` entfällt zugunsten einer Funktion mit Slot.
- Gehört die auslösende Eingabe zu keinem Slot (z. B. Pad ohne Beitritt), ist der Eintrag ohne Wirkung und ausgegraut.
- Wiederbeitritt wie heute per Bestätigen.
- Ist der gewählte Spieler der letzte lokale Spieler, verhält sich „Spieler abmelden“ genau wie „Spiel verlassen“ (S8): ohne Rückfrage `leave` und zurück zur Lobby.
- Logik ohne Phaser in `localSlots.ts`/`optionsLogic.ts` mit Vitest; Texte zentral (`src/core/texts.*.ts`).

## Nicht-Ziele

Neue Halte-Geste; Platz reservieren für späteren Wiederbeitritt; Server- oder Protokolländerungen (`removeSlot` existiert).

## Regeln und Einschränkungen

Domäne CLI; Client rechnet nichts (`noSim.test.ts`); B nicht belegen, View + Menu reserviert; 2+ Spieler gleichzeitig; Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

3 Spieler an der Xbox, Spieler 2 drückt Menu kurz → „Spieler abmelden“ → sein Feld verschwindet, Spieler 1 und 3 spielen im 2er-Layout weiter. Später drückt er A → tritt wieder bei. Zwei Spieler an einer Tastatur, Esc → „Spieler abmelden“ → Auswahl „Spieler 2“ → nur sein Feld verschwindet. Allein am PC, Esc → „Spieler abmelden“ → sofort Lobby.

## Ausnahme- und Fehlerfälle

Server lehnt `removeSlot` ab (`bad_request`) → Feld bleibt, Warnung im Client-Log 🚫; die Eingabe kann per Bestätigen wieder beitreten. Verbindung weg während des Abmeldens → Wiederverbinden ohne diesen Slot. Menü von einer Eingabe ohne Slot geöffnet → Eintrag ausgegraut. Zwei Pads drücken im selben Frame Menu → der erste Pad in der Liste gilt als Öffner. Controller des Öffners wird getrennt, während das Menü offen ist → `LocalSlots.lose` meldet ihn ab, der Eintrag ist danach ohne Wirkung.

## Akzeptanzkriterien

- **AC-01** Vitest: Öffnet Spieler 2 von 3 das Menü und meldet sich ab, geht `removeSlot` nur für seinen Slot raus und seine Eingabe wird frei; Spieler 1 und 3 bleiben gebunden (nicht der höchste Slot).
- **AC-02** Vitest: Abmelden des letzten lokalen Spielers führt ohne Rückfrage zu `leave` statt `removeSlot`.
- **AC-03** Vitest: Pad, Tastatur 1 und Touch werden jeweils als Öffner erkannt; eine Eingabe ohne Slot graut den Eintrag aus.
- **AC-04** Vitest: Mit zwei Tastatur-Spielern fragt „Spieler abmelden“ nach dem Spieler und entfernt nur den gewählten Slot.
- **AC-05** Am Gerät (🧑): Ein Spieler meldet sich über das Spielmenü ab, sein Feld verschwindet, die anderen spielen weiter; Wiederbeitritt per A klappt.

## Offene Fragen

keine (Revision 1: Auslöser Spielmenü mit Öffner, Feld weg; Revision 2: Tastatur 2 per Auswahl nach Esc, letzter Spieler ohne Rückfrage wie „Spiel verlassen“; 🧑 2026-10-13, Chat).

## Notizen

Eingeplant in BD4 (BD4.3). Teilstand aus #274 (Commit 883123a1): Eintrag „Spieler abmelden“, `removeLast` (höchster Slot); wird in BD4.3 durch den Öffner ersetzt.

R2 (2026-10-13): Vollständigkeitsprüfung fand zwei Lücken: Tastatur 2 hat keine Menü-Taste (`KEYBOARD_2`, `src/input/keyboardLayouts.ts`), und „Spiel verlassen“ hat keine Rückfrage (`applyOption` › `leave`). Neues AC-04 (Auswahl bei zwei Tastatur-Spielern), Gerät jetzt AC-05.
