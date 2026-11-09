# RX1.2 · WebGL-Kontextverlust abfangen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** rx1/2-kontextverlust
- **Abhängig von:** RX1.1
- **Tickets:** B-763
- **Kriterien:** AC-05

## Ziel

Ein WebGL-Kontextverlust führt zu einer spielbaren Anzeige statt zur weißen Seite (B-763).

## Kontext

Xbox-Edge meldet nach 3–4 s dreimal „WebGL Context lost. Renderer disabled“; Phaser schaltet ab, Seite weiß. Start in `src/main.ts` (`Phaser.AUTO`, `?renderer=` aus B-761). Die Log-Haken stammen aus RX1.1. Shell-Regeln: Seitenwechsel nur über `openPage()`; Neuladen des eigenen Frames ist mit der Shell abzustimmen. Spec: B-763. Offene Frage der README (Neuaufbau vs. Canvas-Neuladen) hier entscheiden.

## Erlaubte Dateien

- `src/main.ts`, `src/core/contextGuard.ts` (neu), `src/core/renderInfo.ts`, ggf. `src/core/shell.ts`
- Hinweis-Anzeige in `src/scenes/hud/`
- zugehörige `*.test.ts`

## Nicht-Ziele

Keine Texturverkleinerung (RX1.5), keine Optimierung des Canvas-Renderers.

## Schritte

1. Entscheidung Neuaufbau (Kontext wiederherstellen, Texturen neu hochladen) oder Neuladen mit `?renderer=canvas` begründen und im Ticket B-763 notieren.
2. Höchstens ein automatischer Versuch je Sitzung (sessionStorage), danach Canvas und sichtbarer Hinweis.
3. Verbindung und Spielstand bleiben (Client ohne Zustand); Ereignisse ins Log (RX1.1).
4. Tests mit simuliertem Verlust; Reviewer-Agent vor dem Commit (Shell-/Pfad-Code).

## Fertig, wenn

- [ ] AC-05: Test: erster Verlust löst genau einen Neuaufbau/Canvas-Neustart aus, zweiter keinen weiteren.
- [ ] Keine Schleife bei sofortiger Wiederholung; B nicht belegt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
