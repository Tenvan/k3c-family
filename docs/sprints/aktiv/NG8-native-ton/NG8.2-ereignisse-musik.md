# NG8.2 · Ereignisse zu Klängen, Menü-Klänge und Musik

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
