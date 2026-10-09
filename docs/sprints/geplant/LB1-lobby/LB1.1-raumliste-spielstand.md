# LB1.1 · Raumliste, Beitritt und Spielstand wählen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** lb1/1-raumliste-spielstand
- **Abhängig von:** –
- **Tickets:** B-037
- **Kriterien:** AC-01

## Ziel

Die Lobby zeigt neben „Spielen“ und den offenen Räumen die gespeicherten Spielstände des Servers; ein Spielstand lässt sich per Controller, Tastatur oder Tippen wählen und startet oder betritt seinen Raum, ohne einen Raumcode oder Namen einzutippen.

## Kontext

- **Stand (SP08, B-037 › Notizen):** B-037/AC-01 und AC-02 sind umgesetzt, AC-03 weitgehend, AC-04 fehlt.
  - `src/scenes/LobbyScene.ts` (`LobbyScene`, 136 Zeilen) zeichnet nur: Zeilen ab `TOP = 150`, `ROW_H = 64`, Hinweis bei `GAME_HEIGHT - 90`; Bedienung Pfeile/W/S/Stick/D-Pad und A/Enter/Leertaste, Tippen über `window`-`pointerdown` und `rowAt`. B bleibt frei (`PAD_A = 0`, `PAD_UP = 12`, `PAD_DOWN = 13`).
  - `src/scenes/lobbyLogic.ts` (155 Zeilen, Tests `lobbyLogic.test.ts`): `lobbyEntries(rooms, status)` liefert `play` + je Raum `room`, ohne Verbindung `retry`/`reload`; `entryLabel`; `LobbyFlow.choose(entry)` macht aus `play` ein `create` mit `params.save` (Standard `DEFAULT_SAVE = 'familie'`, sonst `?save=NAME`) und aus `room` ein `join`; `save_not_found` → zweiter Versuch mit `fresh: true`.
  - Raumliste kommt per WebSocket-Nachricht `rooms` (`docs/protocol.md`: `code`, `name`, `depth`, `grade`, `taken`, `free`, `running`), Client-Feld `RoomClient.rooms` in `src/online/clientConnection.ts`.
- **Spielstände:** `GET /api/saves` (`docs/protocol.md` › HTTP: Spielstände) liefert ein Array mit `name`, `savedAt`, `version`, `day`, `phase`, `depths`, unlesbare Stände mit `"error": "ungültig"`. Der Client fragt das heute nirgends ab. Die HTTP-Helfer für Spielstände liegen in `src/core/saveStore.ts` (CLI, relative URL `api/…`, `AbortSignal.timeout(TIMEOUT_MS)`); `task dev` leitet `/api` an den Go-Server weiter.
- **Server-Regel** (`docs/protocol.md` › Beitreten): `create` mit `fresh: false` und einem Spielstand, der schon in einem Raum offen ist, tritt diesem Raum bei; ein Spielstand läuft nie doppelt. Der Raumname ist der Name des Spielstands.
- **„Neues Spiel“** der Landingpage (`src/landing/pages.ts`) startet `game.html?fresh=1&save=neu-<zeit>`; das bleibt so.
- **Platz:** Zwischen `TOP` und dem Hinweis passen 13 Zeilen. Anzahl und Reihenfolge der Spielstand-Zeilen: Sprint-README › Offene Fragen (Vorschlag gilt mit der Freigabe).
- Texte über `t()` in `src/core/texts.de.ts` und `src/core/texts.en.ts` (gleiche Schlüssel in beiden, `texts.test.ts`).
- Vitest ohne DOM; `fetch` im Test als Funktion übergeben oder `vi.stubGlobal` nutzen, keine neue Abhängigkeit.

## Erlaubte Dateien

- `src/scenes/LobbyScene.ts`, `src/scenes/lobbyLogic.ts`, `src/scenes/lobbyLogic.test.ts`
- `src/core/saveStore.ts` (Liste der Spielstände lesen) und ein Test dazu
- `src/core/texts.de.ts`, `src/core/texts.en.ts`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Anlegen-Dialog mit Raum-Optionen wie Grad, Ziel, Niederlage-Modus (K5); Namen eintippen; Spielstände löschen oder umbenennen; Änderungen am Server, an `GET /api/saves` oder am WebSocket-Protokoll; Landingpage (PLAT).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Sprint-README › Offene Fragen lesen; die dortigen Vorschläge gelten, sofern 🧑 nichts anderes entschieden hat.
2. In `saveStore.ts` eine Funktion, die `GET api/saves` liest und eine geprüfte Liste (`name`, `savedAt`, `day`, `depths`) liefert; Fehler, Zeitüberschreitung und Einträge mit `error` ergeben keine Einträge. Test.
3. `lobbyLogic.ts`: neuer Eintrag `{ kind: 'save'; save: … }` in `lobbyEntries` (Reihenfolge und Anzahl laut Offener Frage), ohne Spielstände, deren Name schon als offener Raum in der Liste steht; `entryLabel` mit Name, Tag und Stufe; `LobbyFlow.choose` macht daraus `create` mit diesem Namen und `fresh: false`. Tests: Liste mit und ohne Räume, Doppelte entfallen, Auswahl sendet den richtigen Befehl, `lost`/`ended` zeigen weiter nur `retry`/`reload`.
4. `LobbyScene.ts`: Liste beim Öffnen und bei Rückkehr in den Zustand `lobby` laden (nicht jedes Bild), Einträge zeichnen; Bedienung unverändert (B frei).
5. Texte in beiden Sprachen ergänzen.
6. Nachweis je B-037-Kriterium im Ergebnis: AC-01/AC-02 aus den bestehenden Tests von `LobbyFlow` (SP08), AC-03 aus `lobbyEntries`-Tests für `room`, AC-04 aus den neuen Tests.
7. `task check` grün, Ergebnis eintragen, committen.

## Fertig, wenn

- [ ] AC-01: Tests belegen Raum erstellen (B-037/AC-01), Beitreten aus der Liste ohne Code (B-037/AC-02), offene Räume als Einträge (B-037/AC-03) und Spielstand wählen, der `create` mit diesem Namen und `fresh: false` sendet (B-037/AC-04); ohne erreichbares `api/saves` bleibt die Lobby wie bisher bedienbar.
- [ ] `task check` grün; keine Datei über 400, keine Funktion über 60 Zeilen.

## Prüfen

```bash
task check
task test -- lobby
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
