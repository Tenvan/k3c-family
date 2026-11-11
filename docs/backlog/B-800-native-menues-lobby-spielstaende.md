# B-800 · Das native Spiel hat Hauptmenü, Lobby für 1–2 Spieler, Spielstand-Menü, Optionen und Pause

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG7
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Im Browser führen `src/scenes/LobbyScene.ts`, `LoadScene.ts`, `OptionsScene.ts`, `lobbyLogic.ts`, `lobbyOptions.ts`, `lobbyPad.ts`, `saveMenu.ts`, `loadLogic.ts`, `optionsLogic.ts`, `pauseHub.ts`, `runStart.ts`, `menuPages.ts` durch Menü, Beitritt, Spielstände, Optionen und Pause. Raum-Code, Online-Beitritt und Werkzeug-Seiten gibt es nativ nicht.

## Ziel

Paket `native/k3c-game/menu` führt vom Start bis ins Spiel und zurück: Hauptmenü, Lobby für 1–2 Spieler, Spielstände, Optionen, Pause, Spielende.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Sichtprüfung NG9).

## Anforderungen

- Hauptmenü: Weiter, Neues Spiel, Laden, Optionen, Beenden; mit dem Pad bedienbar, B zurück.
- Lobby: Spieler tritt mit A seines Pads bei (1–2), Welt und Seed wählen wie `lobbyOptions.ts`, Start.
- Spielstände: Liste, Laden, Löschen mit Rückfrage, „Weiter“ lädt den neuesten; über `view.Session` (kein Import von `local`).
- Optionen (Lautstärke, Vollbild, was `optionsLogic.ts` lokal sinnvoll hat) und Pause-Menü; Spielende führt ins Hauptmenü.
- Menü-Logik als reine Funktionen mit Tests.

## Nicht-Ziele

Online-Räume, Raum-Code, Beobachter, Werkzeug-Seiten, Dev-Menü; Speichern selbst (B-793).

## Regeln und Einschränkungen

B zurück in jedem Menü, View + Menu reserviert (`CLAUDE.md`). Vertrag `view` unverändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/menu/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Start → Hauptmenü → Neues Spiel → beide drücken A → Welt wählen → Spiel mit zwei Zellen. Im Spiel Menu → Pause-Menü → Hauptmenü → Weiter lädt den Stand.

## Ausnahme- und Fehlerfälle

Kein Spielstand → „Weiter“ ausgegraut. Spielstand nicht ladbar → Eintrag markiert, Laden gesperrt. Löschen nur nach Bestätigung.

## Akzeptanzkriterien

- **AC-01** Hauptmenü und Menü-Gerüst: Tests für Auswahl, B zurück und Übergänge zwischen den Bildschirmen.
- **AC-02** Lobby: Tests für Beitritt von 1 und 2 Spielern per Pad, Wahl von Welt und Seed, Start.
- **AC-03** Spielstände: Tests gegen eine Schein-`view.Session` für Liste, Weiter, Laden, Löschen mit Rückfrage und nicht ladbare Stände.
- **AC-04** Optionen, Pause-Menü und Spielende: Tests für Werte und Übergänge.

## Offene Fragen

- Welche Optionen gibt es nativ? Vorschlag: Lautstärke Musik und Effekte, Vollbild, Sprache wie heute; Online- und Dev-Optionen entfallen. Entscheidet 🧑.

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG7).
