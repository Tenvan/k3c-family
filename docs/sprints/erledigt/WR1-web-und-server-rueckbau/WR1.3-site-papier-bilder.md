# WR1.3 · Präsentationsseite site/ mit Papierschnitt-Bildern

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Lane:** –
- **Modell:** –
- **Umgebung:** ?
- **Branch:** wr1/3-site-papier-bilder
- **Abhängig von:** –
- **Tickets:** B-849
- **Kriterien:** AC-03, AC-04

## Ziel

`site/` zeigt Papierschnitt-Bilder und Aufnahmen des nativen Spiels statt Pixel-Grafik; die Texte nennen das native Spiel am PC (B-849).

## Kontext

`site/index.html` ist statisch (Pages lädt nur `site/`, `tests/pagesSite.test.ts`). Pixel-Grafik lag unter `site/img/grafik`, `site/img/sprites`, `site/img/spiel`. Papierschnitt-PNGs entstehen mit `task gen:paper` in `public/grafik/<gruppe>/<asset>/` (nicht eingecheckt), Aufnahmen mit `k3c-game.exe --shots`.

## Erlaubte Dateien

- `site/**`

## Nicht-Ziele

Neues Seitenlayout; `public/`, natives Spiel.

## Schritte

1. `task gen:paper` rendern, Hintergründe (Wald, Höhle, Mine, Eisenstollen, Kristall), Monarch (Trab), Bogenschütze, Goblin, Münze, Krone und Burg in fünf Stufen nach `site/img/papier/` kopieren.
2. Zwei Aufnahmen des nativen Spiels (Split-Screen, Seed k3c-dev) nach `site/img/spiel/`.
3. Pixel-Grafik entfernen, `site/index.html` auf die neuen Bilder und Texte (natives Spiel am PC, kein Server) umstellen.
4. Sichtprüfung im Browser-Pane (Desktop und Mobil), `task check:diff`.

## Fertig, wenn

- [ ] AC-03: `site/` ohne Pixel-Grafik; Sichtprüfung durch Ralf steht aus.
- [ ] AC-04: `task check:all` grün (Sprint-Abschluss).
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

- AC-03 umgesetzt, Sichtprüfung durch Ralf offen: Pixel-Grafik (`site/img/grafik`, `site/img/sprites`, alte Screenshots) entfernt; neu `site/img/papier/` (Wald-Parallax, Tiefen-Hintergründe, Monarch im Trab, Bogenschütze, Goblin, Münze, Krone, Burg in fünf Stufen, alles aus `task gen:paper`) und `site/img/spiel/` (zwei Aufnahmen des nativen Spiels, `k3c-game.exe --shots`, Seed k3c-dev, Takt 30 und 300). Texte nennen das native Spiel am PC, kein Server, kein Xbox-Edge. Claude hat Desktop (1280×800) und Mobil (375 px, ohne Querscrollen, keine kaputten Bilder) im Browser-Pane geprüft.
- AC-04 geprüft: `task check` grün (1635 Tests), `task check:go` grün (0 Lint-Befunde), `task check:dev` grün (zweiter Lauf; erster Lauf: `TestSimTestMitCheckout` mit Zeitüberschreitung von `git worktree list` unter Last, allein grün), `task build` (natives Spiel nach `dist/k3c-game/`) grün. `check:race` nicht gelaufen (Abnahme-Ticket B-907). Die Shell-Läufe von check, check:go und build liefen, weil k3c-workbench zwischendurch nicht erreichbar war.
- Review (k3c-reviewer, Sonnet): keine schweren Befunde; Kommentar in `native/Taskfile.yml` nachgezogen, `gen:paper`-Zeilenend-Artefakte verworfen, Rest in B-897.
- Beobachtet, ungeprüft: `k3c-game.exe --shots --players 1` bricht mit „--input: Spieler 1 bei 1 Spielern“ ab.
