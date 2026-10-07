# PL1.4 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** pl1/4-review
- **Abhängig von:** PL1.1, PL1.2, PL1.3
- **Tickets:** B-316, B-195, B-215
- **Kriterien:** alle

## Ziel

Der Sprint-Diff ist geprüft, jedes Kriterium hat einen Nachweis, ist als „offen in PL1.5“ vermerkt oder mit Grund und Ticket verschoben, und der eine PR des Sprints ist offen.

## Kontext

**2026-10-07:** B-292 („Neues Spiel“) ist nach LP1 gewechselt (Beschluss 🧑); alle B-292-Schritte und die PL1/AC-01-Punkte dieser Session entfallen.

- Ablauf und Befund-Behandlung: `docs/arbeitsweise.md` › Review-Session.
- Nachweise kommen aus den Ergebnissen von PL1.1 (AC-01, AC-03), PL1.2 (AC-02) und PL1.3 (AC-04). Die manuellen Teile (B-292/AC-02, B-316/AC-02, B-195/AC-02, B-215/AC-02) prüft 🧑 in PL1.5.
- Besonders prüfen: Die Änderung an `src/scenes/GameScene.ts` (PL1.2) bleibt auf die zweite Tastatur-Instanz begrenzt; Layout 2 belegt weder B noch eine reservierte Taste (Pos1, F, Esc, Ö, Ä); `src/core` importiert nichts aus `src/scenes`.
- Der Sprint bleibt aktiv, solange PL1.5 offen ist; die Domäne ist dadurch nicht gesperrt.

## Erlaubte Dateien

- Dateien, die PL1.1 bis PL1.3 geändert haben (nur zum Beheben von Befunden)
- `docs/sprints/geplant/PL1-start-tastatur-koop-texte/`, `docs/sprints/aktiv/PL1-start-tastatur-koop-texte/`, `docs/backlog/`

## Nicht-Ziele

Neue Funktionen; manuelle Abnahmen am PC oder an der Xbox (PL1.5); Werkzeug-Seiten übersetzen.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/pl1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-04 aus den Ergebnissen von PL1.1 bis PL1.3 prüfen; manuelle Teile als „offen in PL1.5“ vermerken.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag (`Version: v… vorgeschlagen (Grund)`).
5. Tickets bleiben bis PL1.5 offen (jedes hat ein manuelles Kriterium).
6. Committen, `git merge origin/develop`, pushen und den einen PR des Sprints öffnen (Branch `sprint/pl1`, Ziel `develop`). Den Sprint-Ordner erst nach PL1.5 nach `docs/sprints/erledigt/` verschieben.

## Fertig, wenn

- [ ] AC-01 bis AC-04 haben einen Nachweis im Ergebnis der jeweiligen Session, sind als „offen in PL1.5“ vermerkt oder mit Grund und Ticket verschoben.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt.
- [ ] `task check` grün; PR des Sprints offen.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
