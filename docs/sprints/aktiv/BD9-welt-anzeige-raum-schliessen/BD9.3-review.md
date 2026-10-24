# BD9.3 · Review BD9

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** bd9/3-review
- **Abhängig von:** BD9.1, BD9.2
- **Tickets:** B-604, B-581
- **Kriterien:** alle

## Ziel

Der Sprint ist geprüft, Abnahme-Abschnitt ausgefüllt, PR auf `develop`.

## Kontext

Sessions BD9.1 und BD9.2; Review-Ablauf `docs/arbeitsweise.md` › Review-Session.

## Erlaubte Dateien

- Tests in `engine/**`, `src/**`
- kleine Fixes (< 30 Zeilen) in den Dateien der Sessions
- `docs/sprints/**/BD9-*/**`, `docs/backlog/B-604*`, `B-581*`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Kriterien AC-01 bis AC-03 prüfen.
2. `task check`, `task check:go`, `task check:race` nacheinander.
3. Lobby und Pausenmenü im Browser-Pane mit zwei Geräten (zwei Tabs) durchgehen.
4. Abnahme-Abschnitt, PR.

## Fertig, wenn

- [ ] Prüfläufe grün, Abnahme ausgefüllt, PR offen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 bis AC-03 geprüft gegen BD9.1 und BD9.2 › Ergebnis; `task check` und `task check:go` grün (`-race` lokal nicht möglich, CGO aus; läuft in der CI).
- Im Browser-Pane mit Spielserver und Vite, zwei Tabs: Lobby zeigt den offenen Raum „familie · Welt standard · 1/4 Plätze“ (AC-02); `POST api/room/close?slot=familie` → 200, unbekannter Slot → 404; Tab im Raum kehrt mit „Raum ist geschlossen“ in die Lobby zurück, der Stand steht danach als Spielstand in der Liste (AC-03, Server). Der Dialog selbst ließ sich im Pane nicht per Tastatur bedienen (Fokus), ist per Vitest belegt; die Prüfung am Gerät folgt in BD9.4.
- Keine schweren Befunde in den Reviews von BD9.1 und BD9.2. Neues Ticket B-613 (Origin-Prüfung schreibender Endpunkte).
