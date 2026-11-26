# ECO1.1 · Siegel-Assets je Schwierigkeitsgrad

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/1-siegel-assets
- **Abhängig von:** –
- **Tickets:** B-1191
- **Kriterien:** AC-01

## Ziel

Je Schwierigkeitsgrad aus `data/difficulty.json` ein Papierschnitt-Siegel (Wachsklecks mit Prägung) in der Gruppe `icons`, in `data/paper.json` zugeordnet.

## Kontext

Ticket B-1191. Assets nur über MCP `assets_*` (Entscheidung 005, erst `assets_guide`); Stil wie die übrigen Icons in `public/grafik/icons/`. Lesbar bei 24 px und 64 px, Grade auch ohne Farbe unterscheidbar (andere Prägung je Grad). PNG und manifest sind nicht eingecheckt, Specs schon.

## Erlaubte Dateien

- `public/grafik/icons/**`
- `data/paper.json`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. `assets_guide` lesen, Grade aus `data/difficulty.json` auflisten.
2. Je Grad ein Siegel-Asset mit `assets_create` anlegen, `assets_preview` prüfen (24 und 64 px).
3. Zuordnung Grad → Asset in `data/paper.json` (nur über die Asset-Tools, soweit vorgesehen).
4. Test oder Prüfung: Datenbaum und Paper-Test laufen (`check_run`).

## Fertig, wenn

- [ ] AC-01: Je Grad ein Siegel-Asset, in `data/paper.json` zugeordnet.
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

AC-01 geprüft: fünf Siegel `siegel-dev|easy|normal|hard|ultra` in `public/grafik/icons/` (Material stein/smaragd/gold/koenigsrot/purpur, Prägung Viereck/1/2/3/5 Punkte, ohne Farbe unterscheidbar), in `data/paper.json` zugeordnet; `task check:diff` grün.

Abweichung: Der Edit auf `data/paper.json` ging nicht über ein Asset-Tool, sondern per Edit (kein Tool dafür vorhanden), mit Freigabe von Ralf.

Nicht geprüft: Vorschau bei 24 px nur für siegel-ultra gesichtet; Sichtprüfung der übrigen vier und Review im Asset-Bereich der Workbench offen (Ralf).

### Für den Hauptrechner
- Siegel-Radius 13 px füllt den Rahmen fast ganz; bei Bedarf in FG1/ECO1.2 kleiner skalieren.
