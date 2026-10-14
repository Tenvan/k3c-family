# BD1.5 · Review BD1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** bd1/5-review
- **Abhängig von:** BD1.4
- **Tickets:** B-392, B-391, B-389, B-399, B-398, B-414, B-393, B-370, B-374
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

BD1 ist abgenommen: alle Kriterien AC-01 bis AC-05 mit Nachweis, schwere Befunde behoben, der eine PR des Sprints ist offen und Auto-Merge eingeschaltet.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session: leicht, nur schwere Befunde (falsches Verhalten, Sicherheit, Determinismus/2 Spieler, Regeln aus `CLAUDE.md`, Kriterium ohne Nachweis). Nur der Diff `origin/develop...origin/sprint/bd1` wird gelesen. Domäne CLI. Läuft nie im selben Lauf wie eine Umsetzung.

Besonderheiten BD1:
- AC-01 und AC-02 (Kamera, Beitritt, Ö) sowie die Beobachtungen aus AC-03/AC-04 sind Abnahmen am PC; sie bleiben bei 🧑 und werden als `angenommen, Validierung offen` geführt, falls nicht bestätigt.
- B-374 hat keine Code-Änderung; der Diff darf `src/scenes/siteView.ts` nicht berühren.
- Hat BD1.4 mangels Mond-Bild blockiert, ist der Sprint nicht abnahmefähig: Review nicht beginnen.
- W6 berührt dieselben Dateien (`HudScene.ts`): Merge-Stand prüfen.

## Erlaubte Dateien

- Befunde beheben nur in den Dateien der Sessions BD1.1–BD1.4 (`src/scenes/`, `src/core/texts.*`, `docs/assets/zuordnung-welt.md`); außerhalb → Ticket
- Planung: Sprint-README BD1 (Abnahme, Status), `docs/sprints/README.md`, `docs/projekte/` (Sprint-Tabelle), Tickets B-392, B-391, B-389, B-399, B-398, B-414, B-393, B-370, B-374 (Status, Archiv)

## Nicht-Ziele

Stil, Benennung, Doku-Feinschliff und mögliche Vereinfachungen (kein Befund); neue Funktionen; manuelle Abnahme am Gerät anstelle von 🧑; einen Versions-Tag setzen (nur Vorschlag).

## Schritte

1. `check_run` (`task check`, `task check:go`) → grün.
2. `git fetch`; `git diff origin/develop...origin/sprint/bd1` lesen, nur den Diff.
3. Befunde nach der Liste in Kontext prüfen; schwere beheben (Domäne CLI), sonst Ticket.
4. Kriterien AC-01 bis AC-05 gegen die Session-Ergebnisse abgleichen; offene PC-Abnahmen als `angenommen, Validierung offen (Session)` führen.
5. Abnahme in die Sprint-README (≤ 5 Zeilen: Datum, Kriterien, behobene Befunde, neue Tickets), `Version: v… vorgeschlagen (Grund)`.
6. Erledigte Tickets (B-374: erledigt, entschieden) nach `backlog/archiv/` und in den Index-Abschnitt „Archiv“; Sprint-Ordner nach `sprints/erledigt/`, `Status: erledigt` (nur wenn keine Hardware-Session offen); Fahrplan anpassen.
7. Commit, `git merge origin/develop`, push; **den einen PR** des Sprints öffnen (Vorlage, eine Zeile je Session), danach `gh pr merge --auto --squash` (ist der PR schon `CLEAN`: `gh pr merge --squash`).

## Fertig, wenn

- [ ] AC-01 bis AC-05: je Kriterium Nachweis im Ergebnis (`geprüft`, `umgesetzt`, `angenommen, Validierung offen` oder `verschoben` mit Ticket).
- [ ] `task check` und `task check:go` grün; keine schweren Befunde offen.
- [ ] Abnahme in der Sprint-README steht (≤ 5 Zeilen), Versionsvorschlag eingetragen.
- [ ] Der eine PR des Sprints ist offen, Auto-Merge eingeschaltet; Ordner und Fahrplan stimmen.

## Prüfen

```bash
task check
task check:go
git fetch && git diff origin/develop...origin/sprint/bd1 --stat
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
