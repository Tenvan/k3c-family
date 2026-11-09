# RX1.5 · Texturprofil Xbox: Sheets höchstens 2048 px, Laden je Stufe

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** rx1/5-texturprofil
- **Abhängig von:** RX1.4
- **Tickets:** B-764
- **Kriterien:** AC-06, AC-07

## Ziel

Alle Papierschnitt-Sheets sind höchstens 2048 px je Kante, der Client lädt beim Start nur die Texturen der ersten Stufe (B-764).

## Kontext

Sheets bis 3072×1920 px in `public/grafik` (Renderer `tools/paper`, Vertrag Entscheidung 005, Zuordnung `data/paper.json`). PNG und Manifeste sind nicht eingecheckt (B-737): Änderung am Generator und an den Specs, `task gen:paper` rendert. Grenzen aus RX1.4 verwenden. Nur ein Profil, kein Low-Profil. Spec: B-764.

## Erlaubte Dateien

- `tools/paper/**`, Specs und `index.paper.json`/`golden.json` unter `public/grafik/`
- `data/paper.json`, Lade-Code in `src/` (Ladereihenfolge je Stufe), Budget-Regel und Test
- `docs/decisions/005*` nur für den Budget-Verweis

## Nicht-Ziele

Kein zweites Profil, keine neue Grafik, kein Kontextschutz (RX1.2); keine Sim-/Balance-/Gen-Läufe (B-742).

## Schritte

1. Budget (max. Kante, Texturspeicher je Stufe) als Regel an einer Stelle festlegen.
2. Generator auf höchstens 2048 px je Kante umstellen, Golden pflegen.
3. Lade-Code: beim Start nur Texturen der ersten Stufe, weitere je Stufenwechsel; prüfen, ob die Indizes das tragen, sonst Folge-Ticket für den Umbau.
4. Tests für beide Kriterien. Reviewer-Agent ab ca. 150 geänderten Code-Zeilen.

## Fertig, wenn

- [ ] AC-06: Test: kein generiertes Sheet größer als 2048 px je Kante.
- [ ] AC-07: Test: Start lädt nur die Texturen der ersten Stufe.
- [ ] Budget-Regel steht an einer Stelle und wird im Test geprüft.

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

–
