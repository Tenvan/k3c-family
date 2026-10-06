# B-293 · Das Spielmenü hat neben „Weiter“ einen Eintrag „Spiel verlassen“

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** S8
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Optionen-Szene (`src/scenes/OptionsScene.ts`, Logik `optionsLogic.ts`) ist zugleich das Pausenmenü und kennt nur „Weiter“. Ein Spiel verlassen geht nur indirekt (Controller trennen, Seite wechseln). `GameScene.leaveRoom()` (`src/scenes/GameScene.ts`) leistet das Verlassen schon: `client.leave()`, zurück in die Lobby; der Server speichert beim Verlassen.

## Ziel

Spielende verlassen das Spiel aus dem Spielmenü und landen in der Lobby, mit jedem Eingabegerät.

## Beteiligte und Zielgruppen

Spielende Familie (Tastatur, Controller, Touch); 🧑 nimmt ab.

## Anforderungen

- Neuer Eintrag „Spiel verlassen“ (`leave`) direkt unter „Weiter“, Text in `texts.de.ts` und `texts.en.ts`.
- Bestätigen (Enter, Leertaste, A, Tippen) verlässt den Raum für alle lokalen Spieler des Geräts und öffnet die Lobby; die Optionen-Szene schließt.
- Links/Rechts auf dem Eintrag tut nichts.
- Kein Protokoll- und Server-Eingriff.

## Nicht-Ziele

Rückfrage-Dialog vor dem Verlassen; einzelnen lokalen Spieler abmelden (`removeSlot`).

## Regeln und Einschränkungen

B bleibt unbelegt, View + Menu bleibt „zurück zur Landingpage“ (`CLAUDE.md`). Text nur über die Textdateien (S5.3). Logik ohne Phaser in `optionsLogic.ts`, die Szene zeichnet nur.

## Beispiele

Menu/Esc öffnet das Menü, Runter auf „Spiel verlassen“, Enter → Lobby mit dem Eintrag „Spielen“ und den offenen Räumen.

## Ausnahme- und Fehlerfälle

Ist die Verbindung schon weg, führt `GameScene` ohnehin in die Lobby (`leavesGame`); der Eintrag ändert daran nichts.

## Akzeptanzkriterien

- **AC-01** `task test -- optionsLogic`: `OPTION_IDS` endet mit `resume`, `leave`; `applyOption(…, 'leave', 'confirm').leave` ist `true`, bei `left`/`right` `false`, alle anderen Einträge setzen `leave` nie; `optionLabel` liefert für `leave` einen Text in beiden Sprachen.
- **AC-02** Im Browser: Menü öffnen, „Spiel verlassen“ bestätigen → Lobby; Server-Log zeigt den Austritt aus dem Raum.
- **AC-03** Alle Zeilen des Menüs passen auf 1080 px Höhe, über dem Hinweistext.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
