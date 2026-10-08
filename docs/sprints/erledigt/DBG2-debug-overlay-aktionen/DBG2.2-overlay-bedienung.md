# DBG2.2 · Overlay-Ansicht mit Aktionen, Zeitfaktor und Bedienung

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** dbg2/2-overlay-bedienung
- **Abhängig von:** DBG2.1
- **Tickets:** B-179
- **Kriterien:** AC-03, AC-04

## Ziel

Das Debug-Overlay zeigt im Dev-Mode die Aktionsliste, löst Gold, Material und Zeitfaktor per Maus, Touch und Controller aus (Senden der `dev`-Nachricht) und zeigt den Zeitfaktor des Raums in seinen Textzeilen.

## Kontext

- Stand nach DBG2.1: `src/scenes/debugActions.ts` hat die Aktionsliste, `devMessage` und `actionsVisible`. Spezifikation: B-179 (`docs/backlog/B-179-debug-overlay-aktionen.md`); Protokoll: `docs/protocol.md` (von DBG1, Nachricht `dev`, Fehlercode `forbidden`, Zustandsfeld für den Zeitfaktor).
- Overlay heute: `src/scenes/debugOverlay.ts` (reine Zeilen-Funktion `debugLines`, `DebugWorld` ist eine schmale Auswahl der Welt-Felder) und `debugOverlayView.ts` (Klasse `DebugOverlay`, Phaser-Text oben links, `update(client, world)` pro Frame aus `HudScene.update`; Toggle: Ö bzw. Klick auf den linken Stick, `PAD_LS`). Erzeugt nur, wenn `debugEnabled(location.search)`.
- Verbindung: `src/online/clientConnection.ts` (`RoomClient`) sendet über das private `send(message)`; es gibt dort je Nachricht eine kleine Methode (`addSlot`, `removeSlot`, `sendInput`). `fail(code, message)` setzt für unbekannte Codes wie `forbidden` `notice` und `errorCode` und bleibt im Raum. `client.you` sind die lokalen Slots (`SlotSeat.slot`).
- **Fallstrick Touch:** Auf Touch-Geräten liegt eine bildschirmfüllende DOM-Fläche (`.k3c-zone`, z-index 9, `src/input/touchInput.ts`) und die Schaltflächen (`.k3c-touch`, z-index 10) über dem Canvas; Phaser-Pointer-Ereignisse erreichen den Canvas dort nicht. Vorschlag: Aktionen als DOM-Schaltflächen mit eigenem z-index über beiden (funktioniert für Maus und Touch); `touchInput.ts` (CSS, `pointerdown`) als Vorbild; `src/input/` selbst nur lesen (Domäne PLAT).
- **Fallstrick Controller:** Das D-Pad links/rechts bewegt den Spieler und A ist „bestätigen“ (beitreten, Münzen geben), `src/input/playerInput.ts` (`GamepadInput`). B (Zurück auf Edge), View + Menu (Landingpage) und X (interagieren) bleiben tabu. Vorschlag: Ein freier Knopf (RB, in `PAD_ACTIONS` ungenutzt) schaltet bei sichtbarer Aktionsliste den „Dev-Fokus“ ein; im Fokus wählt das D-Pad hoch/runter die Zeile, A löst aus, und `GameScene` sendet in dieser Zeit keine Bewegung und keine Bestätigung der Spieler. Ob RB passt, entscheidet sich beim Bau (Konflikt mit anderen Belegungen prüfen, z. B. in den Sprints S3 und S5 unter `docs/sprints/`); die gewählte Belegung steht im Ergebnis und in der Overlay-Beschriftung. Nicht belegen: B, View + Menu, X.
- Fehlerfall: Antwort `forbidden` → kurzer Hinweis im Overlay (Text aus `client.notice`, solange `client.errorCode === 'forbidden'`), keine Wiederholung. Raum ohne Dev-Mode → keine Liste, keine Meldung.
- Regeln: `src/scenes` zeichnet nur, rechnet nichts (`noSim.test.ts`). `HudScene.ts` hat die Komplexitäts-Grenze 17 (`.oxlintrc.json`, darf nicht steigen): dort höchstens eine Zeile ändern, Logik in `debugOverlayView.ts` oder der neuen Datei. Datei ≤ 400 Zeilen (`clientConnection.ts` ist nahe dran: nur eine kleine Methode), Funktion ≤ 60. Jede Mechanik muss mit 2 lokalen Spielern funktionieren: Gold fällt für den Slot, den die Auswahl bestimmt (Vorschlag: der erste lokale Slot, mit Umschalter für den zweiten, falls ein zweiter Spieler lokal sitzt).

## Erlaubte Dateien

- `src/scenes/debugOverlay.ts`, `src/scenes/debugOverlayView.ts`, `src/scenes/debugOverlay.test.ts`
- `src/scenes/debugActions.ts`, `src/scenes/debugActions.test.ts` (nur kleine Anpassungen aus DBG2.1)
- `src/scenes/debugOverlayPanel.ts` (neu, DOM-Schaltflächen und Fokus-Auswahl), `src/scenes/debugOverlayPanel.test.ts` (neu, nur reine Teile)
- `src/online/clientConnection.ts`, `src/online/clientConnection.test.ts`, `src/online/clientProtocol.ts` (nur Methode `sendDev`, Typen, Test)
- `src/scenes/GameScene.ts` (nur die Eingabe-Sperre im Dev-Fokus), `src/scenes/HudScene.ts` (höchstens eine Zeile)
- `docs/sprints/aktiv/DBG2-debug-overlay-aktionen/` (nur Status und Ergebnis), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Serverseitige Aktionen (DBG1), Wechsel des Schwierigkeitsgrads (B-107), Stufenwechsel und Neustart (B-080), Änderungen in `src/input/`, Abnahme am Gerät (DBG2.3).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Zustandsfeld und Nachricht aus `docs/protocol.md` bestätigen; `DebugWorld` und `DebugInput` in `debugOverlay.ts` um den Zeitfaktor erweitern.
2. `debugLines` um eine Zeile für den Zeitfaktor ergänzen (z. B. `Zeit 4×`; nur im Dev-Mode des Raums, Faktor 1 als `Zeit 1×`); Test in `debugOverlay.test.ts` mit wörtlich erwarteter Zeile, auch für den Raum ohne Dev-Mode (keine Zeile).
3. `RoomClient.sendDev(message)`: sendet die Nachricht nur im Status `room` (wie `addSlot`); Test in `clientConnection.test.ts` mit dem Mock-Socket (Nachricht geht raus, außerhalb des Raums nicht).
4. `debugOverlayPanel.ts`: DOM-Schaltflächen aus der Aktionsliste, sichtbar nur bei `actionsVisible(...)`; Klick/Tippen ruft `devMessage` und `client.sendDev`; die gewählte Zeile des Controller-Fokus ist hervorgehoben. Aufräumen beim Szenen-Ende (Listener entfernen, Elemente löschen), damit die Schaltflächen nicht im Menü hängen bleiben.
5. Controller: Fokus-Schalter, D-Pad hoch/runter, A auslösen wie im Kontext; `GameScene` ignoriert in dieser Zeit Bewegung und Bestätigung der Controller-Spieler. Taste Ö und Klick auf den linken Stick schalten das Overlay weiter um.
6. `forbidden`: Hinweis im Overlay anzeigen (kein erneutes Senden).
7. Lokal mit `task dev` und `task start` im Browser-Pane (Anleitung in `CLAUDE.md` › „Im Browser-Pane testen“) mit `K3C_DEV` an prüfen: Overlay mit Ö öffnen, „Gold 50“ klicken, „Zeit 8×“ klicken; die Zeile zeigt 8×. Die Prüfung am Handy und Controller macht 🧑 in DBG2.3.
8. `task check` ausführen; `.oxlintrc.json` nicht ändern. Ergebnis eintragen.

## Fertig, wenn

- [x] AC-03: Ein Test in `debugOverlay.test.ts` zeigt die Zeitfaktor-Zeile aus dem Zustand (Faktor 8 → `Zeit 8×`), und im Browser-Pane zeigt das Overlay nach „Zeit 8×“ den Faktor.
- [x] AC-04: `task check` grün, darin `src/scenes/noSim.test.ts`.
- [x] Ohne Dev-Mode des Raums erscheinen weder Liste noch Zeitfaktor-Zeile (Test aus Schritt 2 und Beobachtung im Browser-Pane mit `K3C_DEV=0`).
- [x] B, View + Menu und X sind für die Dev-Bedienung nicht belegt (Durchsicht der Eingabe-Änderungen im Diff).
- [x] `.oxlintrc.json` unverändert (die Ausnahmen steigen nicht).

## Prüfen

```bash
task check
```

Manuelle Prüfungen am Handy und mit Controller macht 🧑 in DBG2.3; hier nur der Browser-Pane.

## Ergebnis

2026-10-03, Agent (Claude Opus 5.5), Branch `dbg2/2-overlay-bedienung`.

- Protokoll bestätigt: Nachricht `dev`, Zustandsfeld `devTimescale` (nur Dev-Mode, auch 1), Fehlercode `forbidden`.
- **AC-03 geprüft:** `debugOverlay.test.ts` „Zeitfaktor nur im Dev-Mode des Raums“: Faktor 8 → Zeile `Zeit 8×`, Faktor 1 → `Zeit 1×`, ohne Feld keine Zeile. Browser-Pane (Server aus diesem Branch auf Port 8091, Dev-Mode an, Frames von Hand getaktet): Overlay mit Ö geöffnet, Liste erscheint oben rechts; Klick „Gold 50“ → 50 Münzen in der Welt; Klick „Zeit 8×“ → `devTimescale` 8 und Zeile `Zeit 8×` im Overlay.
- Controller (Pad im Browser-Pane gemockt): RB schaltet den Dev-Fokus (Rahmen gelb, `devFocus` in der Registry), D-Pad runter ×2 wählt „Gold 100“, A löst aus → +100 Münzen; A im Fokus lässt keinen zweiten Spieler beitreten, Bewegung und Münzen der Controller-Spieler sind im Fokus stumm (`muteFocused`). D-Pad links/rechts bzw. die Schaltfläche „für Spieler n“ wählt den lokalen Spieler (2 Spieler am Gerät).
- **Ohne Dev-Mode** (`K3C_DEV=0`): Overlay offen, keine Aktionsliste, keine Zeitzeile (Browser-Pane und Test).
- `forbidden`: Overlay zeigt „Dev abgelehnt: <Hinweis>“, keine Wiederholung (Test in `debugOverlay.test.ts`).
- **AC-04 geprüft:** `task check` grün (656 Tests, darin `noSim.test.ts`, `projectRules.test.ts`); `.oxlintrc.json` unverändert; `HudScene.ts` unverändert.
- Belegung: RB (Fokus), D-Pad, A nur im Fokus; B, View + Menu und X nicht belegt (Test „B, X, View und Menu gehören nicht zur Fokus-Bedienung“). Hinweis für S3: RB wird dort Skill-Slot 2 (`rules/monarch.md` § 4); der Dev-Fokus greift nur bei offenem Overlay im Dev-Raum.
- Dateien: neu `debugOverlayPanel.ts` (reine Fokus-Logik, DOM-Schaltflächen z-index 11 über dem Touch-Overlay, Aufräumen bei Szenen-Ende) mit Test; `RoomClient.sendDev` mit Test; `GameScene` sperrt im Fokus die Controller-Spieler.
