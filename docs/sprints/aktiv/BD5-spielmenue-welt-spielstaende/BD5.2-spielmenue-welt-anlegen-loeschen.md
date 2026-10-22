# BD5.2 · Spielmenü: Welt anzeigen, Spielstand anlegen und löschen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/bd5
- **Abhängig von:** BD5.1
- **Tickets:** B-506, B-534
- **Kriterien:** AC-02

## Ziel

Die Lobby zeigt die Start-Welt und die Welt je Spielstand, bietet „Neuer Spielstand“ und löscht einen Spielstand nach Rückfrage; das Pausenmenü nennt die Welt des Raums.

## Kontext

Lobby: `src/scenes/lobbyLogic.ts` (Einträge, `LobbyFlow`), `LobbyScene.ts` (zeichnet, Eingaben). Spielstände: `src/core/saveStore.ts` (`listSaves`). Pausenmenü: `OptionsScene.ts`, `optionsLogic.ts` (BD4). Welt des Raums: `joined.world` in `src/online/clientConnection.ts`. Server aus BD5.1: `DELETE /api/save?slot=` (200/404/409), `GET /api/saves` mit `world`. Entscheidung B-534.

## Erlaubte Dateien

- `src/scenes/lobbyLogic.ts`, `lobbyLogic.test.ts`, `LobbyScene.ts`
- `src/scenes/saveMenu.ts`, `saveMenu.test.ts` (neu, rein)
- `src/core/saveStore.ts`, `src/core/saveStore.test.ts`
- `src/scenes/OptionsScene.ts`, `optionsLogic.ts`, `optionsLogic.test.ts`, `GameScene.ts`
- `src/online/clientConnection.ts`, `clientConnection.test.ts`
- `src/core/texts.de.ts`, `texts.en.ts`

## Nicht-Ziele

Keine Welt-Auswahl, keine Namenseingabe, kein Löschen per Touch-Geste (nur Rückfrage-Zeilen sind tippbar).

## Schritte

1. Vitest: nächster freier Name `spiel-N`, Eintrag „Neuer Spielstand“ erzeugt `create` fresh, Rückfrage-Zustand (Abbrechen vorgewählt, nur Löschen + Bestätigen löscht), Hinweise je Server-Antwort, `deleteSave` (DELETE, Status → Ergebnis), Welt-Anzeige.
2. Umsetzen, Lobby im Browser-Pane prüfen.

## Fertig, wenn

- [ ] AC-02: Vitest der Menü-Logik grün; Lobby im Browser-Pane zeigt Welt, legt an, löscht nach Rückfrage.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-02 umgesetzt: Lobby zeigt „Welt: …“ und die Welt je Spielstand, Eintrag „Neuer Spielstand (spiel-N)“ (`create` fresh in der Start-Welt), X/Entf auf einem Spielstand öffnet die Rückfrage (`saveMenu.ts`, „Abbrechen“ vorgewählt), `deleteSave` in `saveStore.ts`, Hinweis je Server-Antwort; Pausenmenü zeigt „Welt: …“ (`roomWorld` aus `joined`).
- AC-02 geprüft: Vitest `saveMenu.test.ts`, `saveStore.test.ts`, `lobbyLogic.test.ts`, `optionsLogic.test.ts`, `clientConnection.test.ts`; Browser-Pane (Worktree-Server): Welt angezeigt, Abbrechen lässt den Stand, Löschen entfernt ihn, „Neuer Spielstand“ startet `spiel-1`, Pausenmenü „Welt: standard“; `task check:diff` grün; Reviewer: keine schweren Befunde.
- Offen, nicht Teil von AC-02: kein Touch-Weg zum Löschen (nur Rückfrage-Zeilen tippbar); ein gelöschter Name `spiel-N` wird wieder vergeben, seine Sicherungen liegen dann neben denen des neuen Stands (B-533).
