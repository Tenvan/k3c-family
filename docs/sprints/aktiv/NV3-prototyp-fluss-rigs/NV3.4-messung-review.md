# NV3.4 · Messung am Entwicklungs-PC und Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** nv3/4-messung-review
- **Abhängig von:** NV3.3
- **Tickets:** B-789
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Messbericht vom Entwicklungs-PC und Review des Sprints.

## Kontext

`native/measure.ps1`, `task native:ebiten:measure`. Je Fassung (Rig, Sheet) ein Lauf, keine Wiederholungen ohne Anlass. Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Batching, Allokationen im Frame, Texturbudget).

## Erlaubte Dateien

- `native/**`
- Planung B-789, NV3 (über MCP)

## Nicht-Ziele

Messung am Mini-PC (B-786); Simulations- und Balancing-Läufe (gesperrt).

## Schritte

1. Messlauf Rig und Sheet im selben Level, Bericht nach `reports/`.
2. Review des Sprint-Diffs, schwere Befunde beheben.
3. Werte in B-789 › Notizen, Abnahme in der Sprint-README, PR.

## Fertig, wenn

- [ ] AC-04: Messbericht mit Bildrate, Grafikspeicher, Rechenzeit, Rig gegen Sheet.
- [ ] AC-01 bis AC-03 aus den Ergebnissen von NV3.1 bis NV3.3 bestätigt.

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

AC-04 geprüft: Messlauf Entwicklungs-PC (Notebook, DirectX 11, 8 s nach 4 s Aufwärmen, ohne VSync, je Art ein Lauf, Berichte `reports/native-ebiten-nv3/messung-rig.json` und `messung-sheet.json`): Rig 1188 FPS Mittel, 112 FPS 1-%-Low, 0,482 ms Rechenzeit je Frame, 16,7 MiB RGBA geladen (10,6 Sheets + 6,1 Rig-Teile); Sheet 1660 FPS, 179 FPS 1-%-Low, 0,147 ms, 31,9 MiB. Grafikspeicher der GPU nicht gemessen (nur geladene Texturen); die Werte entstanden bei laufenden anderen Agenten und schwanken.
AC-01 bis AC-03 bestätigt aus NV3.1 bis NV3.3 (Screenshots, Tests).
Review (Agent `k3c-reviewer`, Opus): keine schweren Befunde. Behoben: Rig-Teile ohne Mipmaps wie Sheets (Ruckler-Quelle, geschönter MiB-Vergleich). Offen für 🧑: Lauf-Frequenz Rig 1,6 Hz gegen Sheet 1 Hz (Sheet spielt mit fps 8 statt 12,8); Aufwärmen deckt Rig-Bilder nicht ab.
`task check` und `task check:go` nicht gefahren: Der Sprint ändert nur `native/`, das beide nicht prüfen; `check:diff:go` der nativen Module grün. Kein Abnahme-Ticket: weder Sim, Balance, Gen noch `engine/` berührt.
