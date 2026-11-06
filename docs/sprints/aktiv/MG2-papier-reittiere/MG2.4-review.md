# MG2.4 · Review MG2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** reittiere
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg2/4-review
- **Abhängig von:** MG2.3
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG2 ist geprüft und abgenommen: alle Reittiere gültig, gerendert, Golden aktuell, Befunde behoben oder als D-Zeile/Ticket.

## Kontext

Sprint-README und Ergebnisse MG2.1–MG2.3; Prüfung nur über `assets_list`, `assets_preview`, `assets_review` mit `checkout`; Sichtprüfung durch 🧑 in PS1.

## Erlaubte Dateien

- `public/grafik/reittiere/**` (nur Befund-Fixes)
- `docs/sprints/*/MG2-papier-reittiere/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Assets.

## Schritte

1. `assets_list {gruppe: reittiere}`, `assets_review {basis: origin/develop}` bzw. gegen den Start-Commit.
2. Je Asset alle drei Gangarten Frame 0 und 4 per `assets_preview` ansehen; Stichprobe Monarch beim Reiten (`monarch galopp`) mit pferd.
3. `task gen:paper`, `task test -- paper`, `task check:diff`.
4. Abnahme in der Sprint-README, D-Zeilen bestätigen.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis im Ergebnis.
- [ ] Abnahme eingetragen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Review (Opus, getrennt, nur über MCP): AC-01 erfüllt (11/11 gültig, keines aus `mounts` fehlt), AC-02 erfüllt (33 PNGs hash-gleich mit `golden.json`, Index vollständig), AC-03 erfüllt bis auf die Sichtprüfung durch 🧑 in PS1; `assets_review` gegen 44f2d30c: 10× `neu`, kein `ungueltig`, `pferd` unverändert; Monarch reitet sauber (`galopp` 2/3).
- Behoben: Schweif im Galopp links abgeschnitten → Rahmen der zehn neuen Reittiere links um 10 Einheiten erweitert (D-MG2-02; `pferd` gehört MG1, offen). D-MG2-01 und -06 an den tatsächlichen Stand angepasst.
- Hingenommen (Minor): Wölfe/Höllenhund/Elefant pferdeartig (D-MG2-08, Wunsch an RD1); rappe/albtraum/hoellenhund dunkelbraun statt schwarz (D-MG2-01, Lane `stil`).
- `task test -- paper` grün; `task check:diff`: Client 3173/3173, Lint grün, rot nur Frontend-Tests der Workbench (`AnimationSync`, Prompt-Buttons; D-MG2-07, nicht MG2).
