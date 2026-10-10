# GL1.2 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** gl1/2-review
- **Abhängig von:** GL1.1
- **Tickets:** B-369
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07

## Ziel

Der Sprint GL1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und der PR des Sprints ist geöffnet; AC-06 steht als `angenommen, Validierung offen (GL1.3)`.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/landing/`, `index.html`, `src/core/texts.*.ts` und `src/core/platformTextRule.test.ts`. Besonders prüfen: Landingpage bleibt offen (iframe), Vollbild nur über `toggleFullscreen`/`toggleLocal`, die gemerkte Kachel funktioniert nach dem Sprachwechsel, `showNoServer` auf der Spielseite zeigt weiter Text, B unbelegt.

## Erlaubte Dateien

- `src/landing/`, `index.html`, `src/core/texts.de.ts`, `src/core/texts.en.ts` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Umbau von Produktionscode; Browser- oder Geräteprüfungen (GL1.3).

## Schritte

1. `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/gl1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis für AC-02 aus GL1.1 › Ergebnis prüfen; AC-06 als `angenommen, Validierung offen (GL1.3)` führen; AC-01, AC-03, AC-04, AC-05, AC-07 als `entfällt (B-339 verworfen)`.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. GL1.3 ist offen: Sprint bleibt in `docs/sprints/aktiv/`. Fahrplan anpassen, committen, `git merge origin/develop`, pushen und den einen PR des Sprints öffnen (Branch `sprint/gl1`, Ziel `develop`).

## Fertig, wenn

- [ ] AC-02 hat einen Nachweis in GL1.1 › Ergebnis; AC-06 steht als `angenommen, Validierung offen (GL1.3)`.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt.
- [ ] `task check` grün; PR des Sprints ist geöffnet.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
