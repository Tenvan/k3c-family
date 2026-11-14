# NG9.2 · Messlauf am Entwicklungs-PC und Paket für den Mini-PC

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** ng9/2-messung-paket
- **Abhängig von:** NG9.1
- **Tickets:** B-802
- **Kriterien:** AC-03

## Ziel

Ein Messbericht vom Entwicklungs-PC liegt vor und ein Paket für den Mini-PC ist gebaut.

## Kontext

Spec: B-802/AC-03, Mini-PC-Teil von B-786 und B-780. Vorbild: `native/measure.ps1`, `native/messen.cmd`, Tasks `native:rig:paket` und `native:ebiten:measure` in `native/Taskfile.yml`. Messlauf je einmal (Bildrate, Grafikspeicher, Rechenzeit je Bild, Takt der Simulation), 1 und 2 Spieler, fester Seed; keine Wiederholungen ohne Anlass. Paket nach `.work/native-game/` mit EXE, Grafik, Messskript und Anleitung für 🧑 (Sichtprüfung und Messung am Mini-PC, Bericht zurück).

## Erlaubte Dateien

- `native/k3c-game/game/**`
- `native/Taskfile.yml`
- `native/measure.ps1`
- `reports/native-game-*.json`
- Planung NG9, B-802, B-786 (über MCP)

## Nicht-Ziele

Lasttest (`k3c-load`, `sim_test`); Optimierungen (Befund → Ticket).

## Schritte

1. Messmodus `--measure` im Spiel (falls nicht vorhanden) und Task `native:game:measure`.
2. Je Spielerzahl ein Lauf, Bericht nach `reports/`.
3. Task `native:game:paket`, Paket bauen, Inhalt im Ergebnis.

## Fertig, wenn

- [ ] AC-03: Bericht unter `reports/`, Werte im Ergebnis; Paket unter `.work/native-game/`.
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

AC-03 geprüft: Messung des ganzen Spiels am Entwicklungs-PC (`task native:game:measure`, Core Ultra 7 165H, Intel Arc integriert, DirectX 11, 1920×1080, VSync aus, 2 Spieler, je ein Lauf): Tag 1794 FPS Mittel, 1-%-Low 350, Zeichnen 0,29 ms je Bild, RAM 335 MiB; Nacht mit 9 Gegnern 1181 FPS, 1-%-Low 190, 0,54 ms, RAM 433 MiB; Texturen 18,2 MiB Sheets und 9,5 MiB Rigs. Paket `task native:game:paket` nach `.work/native-game/` (49 MB, nur die geladene Grafik), Start aus dem Paket-Ordner geprüft; Checkliste `.work/native-game/CHECKLISTE.md` für NG9.4 und NG9.5.
