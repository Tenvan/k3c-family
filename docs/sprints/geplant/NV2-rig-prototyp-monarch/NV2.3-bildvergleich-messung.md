# NV2.3 · Bildvergleich gegen das Sheet und Messung am Entwicklungs-PC

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** nv2/3-bildvergleich-messung
- **Abhängig von:** NV2.2
- **Tickets:** B-778
- **Kriterien:** AC-04, AC-05

## Ziel

Der Prototyp vergleicht Rig- und Sheet-Monarch je Frame als Bild und misst FPS und Speicher; die Werte vom Entwicklungs-PC stehen in B-778 › Notizen.

## Kontext

- Messmodus und Berichtsfelder wie `native/k3c-ebiten/measure.go` und `native/measure.ps1` (B-777).
- Vergleich: Rig-Monarch in den Phasen der 8 Sheet-Frames in ein Offscreen-Bild zeichnen, gegen den Sheet-Frame legen; Kennzahl mittlere und größte Abweichung je Kanal, Anteil abweichender Pixel; Differenzbild als PNG nach `reports/`.
- Der Umhang ist im Sheet ein eingeschwungener Zyklus; für den Vergleich den Umhang ausblenden oder getrennt bewerten.

## Erlaubte Dateien

- `native/k3c-rig/**`
- `native/Taskfile.yml`, `native/measure.ps1`
- Planung B-778, NV2 (über MCP)

## Nicht-Ziele

Optik nachbessern über die Defaults hinaus (Befunde als Notiz für 🧑); Messung am Mini-PC.

## Schritte

1. Vergleichsmodus `--vergleich` mit Kennzahlen und Differenzbild für `idle`, `lauf` und `trab` (Reiter mit Pferd).
2. Messmodus: 2, 50 und 200 Figuren (Reiter mit Pferd), je als Rig und als Sheet: FPS, Rechenzeit je Bild, GPU-Speicher, RAM.
3. `task native:rig:measure` schreibt `reports/native-rig-<rechner>-<zeit>.json`.
4. Werte und Bewertung von Schatten und Schärfe in B-778 › Notizen, Hinweis für B-780 (Grenze der Figurenzahl); Paket für den Mini-PC.

## Fertig, wenn

- [ ] AC-04: Kennzahlen und Differenzbilder für `idle`, `lauf` und `trab` im Bericht, Schatten und Schärfe bewertet.
- [ ] AC-05: Messwerte Entwicklungs-PC mit 2, 50 und 200 Figuren in B-778 › Notizen.

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
