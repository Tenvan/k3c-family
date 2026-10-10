# B-382 · Ein gewählter Spielstand startet nie leer neu

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-10
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`LobbyFlow.step` in `src/scenes/lobbyLogic.ts` wiederholt ein `create` nach `save_not_found` einmal mit `fresh: true`, wenn `last.save === params.save` (LB1.1). Wählt man in der Lobby einen Spielstand, dessen Name gleich `params.save` ist (Standard `familie`), und wurde er zwischen Liste und Auswahl gelöscht, startet still ein leeres Spiel statt eines Hinweises (Review LB1.2). Außerdem sortiert `lobbyEntries` nach `Date.parse(savedAt)`; ein ungültiges `savedAt` ergibt `NaN` und eine unsichere Reihenfolge.

## Ziel

Der Neuversuch mit `fresh: true` gilt nur für „Spielen“ und den Autostart; Spielstand-Einträge mit kaputtem Datum stören die Reihenfolge nicht.

## Beteiligte und Zielgruppen

Familie am TV und Handy; Agent in `src/scenes/`.

## Anforderungen

- Neuversuch an der Herkunft des Befehls festmachen („Spielen“/Autostart), nicht am Namen.
- Ungültiges `savedAt` sortiert ans Ende.

## Nicht-Ziele

Änderungen am Server oder an `GET /api/saves`.

## Regeln und Einschränkungen

CLI; B nicht belegen; `src/scenes` rechnet keine Spiel-Logik.

## Beispiele

Spielstand-Eintrag `familie` gewählt, Datei inzwischen gelöscht → Hinweis `save_not_found`, kein neues leeres Spiel.

## Ausnahme- und Fehlerfälle

Eintrag mit `savedAt: "x"` → steht unter den gültigen, die übrigen bleiben nach Datum sortiert.

## Akzeptanzkriterien

- **AC-01** Test in `lobbyLogic.test.ts`: Spielstand-Eintrag mit Namen `params.save` plus `save_not_found` liefert keinen weiteren Befehl.
- **AC-02** Test: ein Eintrag mit ungültigem `savedAt` steht hinter den gültigen.

## Offene Fragen

keine

## Notizen

Befund „mittel“ und „leicht“ aus dem Review LB1.2 (2026-10-10).
