# WB2.3 · Review WB2 und Tickets für Styleguide, Sound, Quellen

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wb2/3-review
- **Abhängig von:** WB2.2
- **Tickets:** –
- **Kriterien:** AC-06

## Ziel

WB2 ist mit Opus reviewt, wichtige Befunde behoben, Tickets für Styleguide-Prompts sowie Filter und Prompts für Sound und Quellen liegen im Backlog (Projekt PAP, Domäne DEV), der Sprint ist abgenommen.

## Kontext

Diff `origin/feature/pap..sprint/wb2`. Vorhandene Tickets vorher suchen (`plan_list`, Backlog-Index); bekannt: B-703 (Review auf WB1-Backend), CU1.4 (Sound-Seite liest künftig `data/sounds.json`).

## Erlaubte Dateien

- Dateien aus WB2.1 und WB2.2 (Fixes)
- `docs/backlog/` (nur über `plan_create`), `docs/sprints/…/WB2-*`

## Nicht-Ziele

Die Tickets umsetzen.

## Schritte

1. Reviewer-Agent (Opus) auf den Diff.
2. Wichtige Befunde mit Test beheben.
3. Tickets anlegen.
4. `origin/feature/pap` mergen, `task check:diff`, Abnahme, Sprint erledigt, Push nach `sprint/wb2`.

## Fertig, wenn

- [ ] AC-06 Review ohne offene wichtige Befunde, `task check:diff` grün
- [ ] Tickets angelegt und in der Abnahme genannt

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Review (Opus) auf `cb45f3f3..HEAD -- tools/`: 2 wichtige, 8 kleine Befunde.
- Behoben mit Test (rot → grün): 1 Filter klebt in deckender Hülle, Rahmen bleibt; 2 Vorabladen als Kette mit Abbruch, das Abspielen wartet auf den gezeigten Frame (Browser: höchstens 1 Render offen statt aller Frames); 4 defekter Frame lässt das letzte Bild stehen, Fehler liegt darüber und wird nicht jede Runde neu geladen; 6 Animations-Auswahl der Review mit fester Breite.
- Zurückgestellt (klein): 3 Bühne beim ersten Laden und Asset-Wechsel ohne vorgehaltenes Maß; 5 Live-Region der Lade-Anzeige; 7 Frame-Player in sehr schmalen Fenstern; 8 Kopfzeile bricht um → zweite Scrollebene (alle Seiten mit `calc(100vh - 104px)`); 9 Detail behält Scroll-Position beim Wechsel; 10 Breiten-Tests prüfen nur Inline-Stile.
- Tickets: B-727 (Styleguide Prompt-Buttons), B-728 (Sound Filter und Prompts), B-729 (Quellen Filter und Prompts); vorher kein offenes Ticket, nur erledigte zum alten Studio (B-599, B-673, B-677).
- Prüfung: Asset- und webkit-Tests 133/133 (zweimal), Typecheck Workbench-Frontend grün, oxlint ohne Befund, Browser-Messung. Letztes `task check:diff` auf Wunsch von Ralf abgebrochen; der vorherige Lauf war bis auf den inzwischen behobenen Befund-2-Test grün (3223 + 449/450), `task k3c-workbench:vitest` 450/450.
- `origin/feature/pap` (30ba3062) gemergt.
