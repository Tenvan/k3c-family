# NG1.6 · Review Fundament und Glossar

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng1/6-review
- **Abhängig von:** NG1.5
- **Tickets:** B-792, B-793, B-794
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07, AC-08, AC-09, AC-10, AC-11

## Ziel

Sprint NG1 ist abgenommen, der Vertrag `view` gilt als eingefroren, die neuen Begriffe stehen im Glossar und der PR des Sprints ist offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer`. Schwerpunkte: Vertrag vollständig für NG2–NG8 (Eingabe, Screen, Session, Ereignisse), Importregel, Nebenläufigkeit (Simulation und Zeichnen in einer Goroutine), Spielstände, keine Änderung unter `engine/`. Glossar ist Sammeldatei: im letzten Commit nach `git merge origin/develop` eintragen: „Natives Spiel“, „Frame“ (nativ, nicht Snapshot), „Zelle“, „Zeichner“. Abnahme-Ticket des Sprints nach B-742 anlegen.

## Erlaubte Dateien

- `native/k3c-game/**`
- `tools/checkdiff/**`
- `.github/workflows/ci.yml`
- `native/Taskfile.yml`
- Planung NG1, B-792, B-793, B-794 (über MCP)

## Nicht-Ziele

Neue Funktionen; Umbau des Vertrags über schwere Befunde hinaus; Sim-, Balance- und Lasttest-Läufe.

## Schritte

1. `task check` und `task check:go`, dann `go test ./...` in `native/k3c-game`.
2. Diff `origin/develop...origin/sprint/ng1` lesen, nur schwere Befunde beheben.
3. Glossar im letzten Commit.
4. Abnahme in der Sprint-README, Abnahme-Ticket, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01 bis AC-09: je Kriterium Nachweis aus den Session-Ergebnissen geprüft.
- [ ] AC-10: Glossar enthält die vier Begriffe.
- [ ] PR des Sprints offen, Auto-Merge an.

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
