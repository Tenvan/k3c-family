# NG8.2 · Ereignisse zu Klängen, Menü-Klänge und Musik

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** ton
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng8/2-ereignisse-musik
- **Abhängig von:** NG8.1
- **Tickets:** B-801
- **Kriterien:** AC-02

## Ziel

Ereignisse des Frames, Menü-Aktionen und die Lage (Tag, Nacht, Kampf) lösen dieselben Klänge und dieselbe Musik aus wie im Browser.

## Kontext

Spec: B-801. Vorbild: `src/audio/events.ts`, `ui.ts`, `music.ts` (Zuordnungen, Crossfade), Ereignisse aus `engine/sim/events*.go` über den Frame. Vertrag und Importregel wie NG8.1.

## Erlaubte Dateien

- `native/k3c-game/audio/**`
- Planung NG8, B-801 (über MCP)

## Nicht-Ziele

Neue Musik; Raum-Klang.

## Schritte

1. Test zuerst: jede Zuordnung aus `events.ts` und `ui.ts` als Tabelle, Musikwechsel nach `music.ts`.
2. Zuordnung Ereignis → Slot.
3. Musik mit Überblendung.

## Fertig, wenn

- [ ] AC-02: Tests grün über alle Einträge.
- [ ] `task check:diff` grün.

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

AC-02 umgesetzt: Zuordnung Ereignis/Cue → Klang und Musik je Zustand als reine Logik (Dämpfung nach Abstand, Nacht ×0,6, Musik-Absenkung bei Warnungen, Hufschlag je Reiter, Überblenden über 60 Takte). Geprüft mit Tests, die `events.ts`, `ui.ts` und `music.ts` des Browser-Clients direkt lesen und in beide Richtungen vergleichen, `TestNextMusicState` (Fälle aus `music.test.ts`), `TestPlanNoAllocsPerFrame`. Aufnahme-Modus bleibt stumm und bildgleich (zwei Läufe byte-gleich).
Abweichungen: gleicher Klang je Takt nur einmal; Boss- und Sieg-Musik setzt der Automat wie im Browser nie. Nur mit Ohren prüfbar (🧑, NG9): Balance, Überblenden, Absenkung, Hufschlag, Menü-Klicks.
