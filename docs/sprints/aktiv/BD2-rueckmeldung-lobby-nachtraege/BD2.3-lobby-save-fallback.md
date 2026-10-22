# BD2.3 · Lobby startet nach save_not_found nie still ein leeres Spiel

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** bd2/3-lobby-save-fallback
- **Abhängig von:** BD2.2
- **Tickets:** B-382
- **Kriterien:** AC-03

## Ziel

Ein in der Lobby gewählter Spielstand startet nach `save_not_found` nie ein leeres Spiel, sondern zeigt den Hinweis des Servers; Einträge mit ungültigem `savedAt` sortieren ans Ende (B-382).

## Kontext

- `src/scenes/lobbyLogic.ts` › `LobbyFlow.step`: `save_not_found` wiederholt ein `create` einmal mit `fresh: true`, wenn `last.save === this.params.save` (Z. 163). Wählt man einen Spielstand-Eintrag mit dem Namen `params.save` (Standard `familie`), der zwischen Liste und Auswahl gelöscht wurde, startet still ein leeres Spiel.
- Ursache: Der Neuversuch hängt am **Namen**, nicht an der Herkunft des Befehls. Fix: `LobbyFlow` merkt die Herkunft (`play`/Autostart vs. `save`-Auswahl) im gemerkten Befehl bzw. einem Feld (gesetzt in `step` beim Autostart und in `choose` für `play`, gelöscht für `save`); `fresh: true` nur bei Herkunft `play`/Autostart. Der Hinweis kommt unverändert aus `lobbyNotice` (`client.notice` mit der Server-`message`); neue Texte nur in der zentralen Textdatei, falls nötig.
- `lobbyEntries` sortiert mit `Date.parse(b.savedAt) - Date.parse(a.savedAt)`; ein ungültiges Datum ergibt `NaN` und eine unsichere Reihenfolge. Fix: Ungültige Zeitstempel zählen als `-Infinity` (stehen hinter den gültigen, stabile Reihenfolge).
- Tests: `src/scenes/lobbyLogic.test.ts` (vorhandene `LobbyFlow`-Fälle aus LB1.1/LB1.2 weiter grün halten).
- Regeln: CLI, `src/scenes` rechnet keine Spiel-Logik, Taste B nicht belegen.

## Erlaubte Dateien

- `src/scenes/lobbyLogic.ts`, `src/scenes/lobbyLogic.test.ts`
- zentrale Textdatei nur, wenn ein neuer Hinweistext nötig ist
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Änderungen am Server oder an `GET /api/saves`, Lobby-Umbau über B-382 hinaus, Touch.

## Schritte

1. Branch `bd2/3-lobby-save-fallback`, `Status: in Arbeit`. `lobbyLogic.ts` und die vorhandenen Tests lesen.
2. Tests zuerst (rot): (a) `choose({kind:'save', name: params.save})` + `save_not_found` → `step` liefert `null`; (b) „Spielen“ + `save_not_found` → weiterhin genau ein Neuversuch mit `fresh: true`; Autostart ebenso; (c) Eintrag mit `savedAt: "x"` steht hinter gültigen, übrige nach Datum sortiert.
3. Herkunft statt Namen auswerten, Sortierung härten.
4. `task check:diff` (Client). Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-03: Test in `lobbyLogic.test.ts`: Spielstand-Eintrag mit Namen `params.save` plus `save_not_found` liefert keinen weiteren Befehl; „Spielen“ und Autostart behalten ihren einen Neuversuch (B-382/AC-01).
- [ ] Test: ein Eintrag mit ungültigem `savedAt` steht hinter den gültigen (B-382/AC-02).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; währenddessen nur `task test -- lobbyLogic`. Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
