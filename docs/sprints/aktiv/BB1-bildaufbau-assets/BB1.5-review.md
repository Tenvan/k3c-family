# BB1.5 · Review BB1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hauptrechner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/bb1
- **Abhängig von:** BB1.1, BB1.2, BB1.3, BB1.4
- **Tickets:** –
- **Kriterien:** alle

## Ziel

Der Sprint BB1 ist geprüft, die Abnahme steht in der Sprint-README, die Folge-Tickets und das Abnahme-Ticket sind angelegt, der Sprint-PR ist geöffnet und mit Auto-Merge versehen.

## Kontext

- Ablauf nach `docs/arbeitsweise.md` › Review-Session (Sprint-Abnahme); ein günstiges Modell reicht, gesucht werden nur schwere Befunde.
- Abnahme-Ticket (B-742, genau eins): „Abnahme-Läufe BB1“, Domäne PAP, Typ `Schuld`, Prio `mittel`, Umgebung `live`, Sprint BB1, Projekt PAP, Sperre `golden`; Kriterien: AC-03 Gen (`check_run task:test` mit `gen:paper` und Grafik-Golden für `truppen`, `gebaeude-paper`, `objekte`, `hintergruende`; eine gewollte Golden-Änderung nach `docs/arbeitsweise.md` › Golden aktualisieren) und AC-04 Race (`task:check:race`); Sim und Balance `nicht relevant` mit Grund. Das Ticket läuft auf dem zweiten Rechner (Lane `zweit`).
- Folge-Tickets (Projekt RSU, Domäne CLI; Sprint: Vorschlag ein neuer Sprint „Look-Anbindung BB1“ nach Rang, sonst der nächste geplante RSU-Sprint; zuerst `git fetch` und höchste Ticketnummer aller Branches prüfen): (1) Waldwand und neue Baumvarianten (BB1.3) im Rust-Spiel anbinden (Wahl der Varianten, Waldwand-Schichten aus RU55.6, `rust/crates/k3c-data/src/paper.rs` liest `waldwand`); (2) Leitmotive je Welt (BB1.4) anbinden (`leitmotive`, Platz aus `data/look.json` › `worlds.<level>.motif`); (3) Befunde aus den Ergebnissen von BB1.1 und BB1.2 (Schiffslänge, Bauplatz-Überlappung, Gerüste, feste Pixelwerte der Marken über Figuren).
- Abnahme am TV (AC-06) bleibt bei Ralf; die Review führt sie als Ralf-Queue mit den Ticket-Kriterien (B-1038/AC-02, B-1039/AC-02, B-1040/AC-03).

## Erlaubte Dateien

- `public/grafik/**`
- `data/paper.json`
- `tests/bildaufbauMasstab.test.ts`

## Nicht-Ziele

Neue Assets zeichnen, Stil und Umbenennungen, `gen:paper` und Golden (Abnahme-Ticket), Sim und Balancing.

## Schritte

1. `check_run` mit Ziel `task:check`, danach `task:check:go` (nacheinander): grün.
2. `git fetch` und `git diff origin/main...origin/sprint/bb1` lesen (nur den Diff); `k3c-grafik` parallel zu `check:diff`.
3. Nur schwere Befunde zählen (falsche Maße gegen den Standard, Katalogeinträge ohne Spec, Kriterium ohne Nachweis); in den Erlaubten Dateien beheben, sonst Ticket.
4. Je Kriterium AC-01 bis AC-05 die Ergebnisse von BB1.1 bis BB1.4 prüfen (`geprüft` mit Test, `verschoben` mit Ticket); AC-06 steht in der Ralf-Queue.
5. Abnahme-Ticket und Folge-Tickets anlegen (`plan_create`); Tickets ohne offenes Kriterium auf `erledigt`.
6. Abnahme in der README (höchstens fünf Zeilen), Fahrplan, `git merge origin/main`, pushen, den einen PR öffnen, Auto-Merge `gh pr merge --auto --squash`.

## Fertig, wenn

- [ ] AC-07: `task check` und `task check:go` grün; Abnahme-Ticket angelegt.
- [ ] AC-07: Abnahme in der README, Folge-Tickets angelegt, PR geöffnet mit Auto-Merge.
- [ ] Alle Kriterien AC-01 bis AC-05 mit Nachweis oder `verschoben` mit Ticket.

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
