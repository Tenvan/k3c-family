# US2.3 · Review US2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** us2/3-review
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-03

## Ziel

Review des Sprints mit Opus, wichtige Befunde behoben, Abnahme in der README.

## Kontext

Diff ab `origin/feature/pap` beim Start.

## Erlaubte Dateien

- Dateien aus US2.1 und US2.2, Sprint-README

## Nicht-Ziele

Neue Features.

## Schritte

1. Reviewer (Opus).
2. Befunde beheben.
3. `origin/feature/pap` mergen, prüfen, Commit, Push.

## Fertig, wenn

- [ ] AC-03 erfüllt, Abnahme eingetragen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Review Opus über `bb688941..HEAD`: 0 kritisch, 1 wichtig, 6 klein.
- Behoben: Befund 1 Phase je Objekt aus der Weltposition (`animPhase`, Test RED→GREEN; im Browser 97 Bäume/Büsche über alle 8 Frames gestreut), 2 Deko-Wurzeln und 3 Schild mit Rahmen-Ursprung, Zoom und Animation, 5 Tests exakt (Zoom, Lauf mit Rückfall plus Variante, alte Gebäude-Dateien nur für die Lücken), 7 Emoji 🤒.
- Zurückgestellt: 4 und 6 als D-US2-04.
- `origin/feature/pap` gemergt (schon aktuell); `task check:diff` grün, Vitest `tests` + `src/scenes` 1850/1850.
