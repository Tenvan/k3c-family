# WE6.1 · Spiel: Bau und Endboss in jedem Biom, Ersatz-Asset der Basisart (B-993)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we6
- **Abhängig von:** –
- **Tickets:** B-993
- **Kriterien:** AC-01

## Ziel

Das Spiel zeichnet den Bau in jedem Biom und einen Endboss ohne eigenes Asset mit dem vergrößerten Asset seiner Basisart (B-993).

## Kontext

Natives Spiel `native/k3c-game` (Go + Ebitengine, eigenes Modul): Assets laden `art/manifest.go`, `art/scales.go`, Figuren-Katalog `actors/catalog.go`, Objekte `props/`, Texte `hud/hints.go`, `hud/texts.go` (dort stehen Bau und Endboss). Zuordnung Art → Asset in `data/paper.json` (`gegner`, `skalierung`), Boss-Assets unter `public/grafik/bosse/`. Seit WE5.2 (B-981) meldet die Engine den Bau in der tiefsten Stufe jeder Insel, und `data/bosses.json` › `endBosses` hat Einträge mit `base` (Basisart) ohne eigenes Asset. Das Spiel zeichnet nur, keine Logik. Tests laufen ohne Fenster (`go test` im Modul).

## Erlaubte Dateien

- `native/k3c-game/**`
- `data/paper.json`

## Nicht-Ziele

Engine ändern, neue Assets, Skins (WE6.2).

## Schritte

1. Stellen finden, die Bau oder Endboss an die Kristallhöhle oder eine feste ID binden.
2. Tests zuerst: Asset-Auflösung Endboss (eigenes Asset, sonst Basisart); Bau für Tiefen 0–4.
3. Auflösung und Skalierung umsetzen (`skalierung` in `data/paper.json`, vorläufig 2,5), Namen aus den Daten.
4. Log bei fehlendem Asset (Emoji nach Regel); Review `k3c-hud` oder `k3c-grafik` nur bei Risiko.

## Fertig, wenn

- [ ] AC-01: B-993/AC-01, AC-02 als Tests grün.
- [ ] B-993/AC-03 (Sichtprüfung 🧑) steht in der Ralf-Queue, nicht Teil dieser Session.
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
