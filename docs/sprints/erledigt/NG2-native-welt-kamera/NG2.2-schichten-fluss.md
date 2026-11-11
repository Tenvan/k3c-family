# NG2.2 · Schichten, Boden und Fluss

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** welt
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng2/2-schichten-fluss
- **Abhängig von:** NG2.1
- **Tickets:** B-795
- **Kriterien:** AC-02

## Ziel

Jede Zelle zeigt die Schichten `fern` (fest), `mitte` und `nah` (scrollend), den Boden der Stufe und den Fluss im unteren Drittel.

## Kontext

Spec: B-795. Vorbild aus NV3: `native/k3c-ebiten/level.go` (Schichten, Scroll-Faktoren fern 0,15 / mitte 0,35 / nah 0,5, `fern` seit NV3 fest), `native/k3c-ebiten/river.go` (sechs Wellen-Reihen, eine Farbe, nach links, vorn höher, breiter, schneller). Browser: `src/scenes/paperBackdrop.ts`, `paperBackdropData.ts`. Grafik über `art` (NG1). Vertrag und Importregel wie NG2.1.

## Erlaubte Dateien

- `native/k3c-game/world/**`
- Planung NG2, B-795 (über MCP)

## Nicht-Ziele

Shader (NG11); neue Hintergründe.

## Schritte

1. Test zuerst: Versatz je Schicht bei zwei Kamera-Positionen (`fern` gleich, `mitte`/`nah` verschoben), Fluss-Rechteck = unteres Drittel der Zelle.
2. Schichten und Boden zeichnen.
3. Fluss aus NV3 übernehmen (Code kopieren und anpassen, Prototyp nicht ändern).
4. Screenshots an zwei Kamera-Positionen ins Ergebnis.

## Fertig, wenn

- [ ] AC-02: Tests grün; zwei Screenshots im Ergebnis zeigen stehendes `fern` und fließenden Fluss.
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

AC-02 umgesetzt: Himmel, drei Schichten aus den Papier-Hintergründen (`fern` steht, `mitte` 0,35, `nah` 0,5), Uferstreifen, Fluss im unteren Drittel mit sechs Reihen nach links; Assets je Level über `view.AssetSource`. Geprüft mit `layers_test.go`, `river_test.go` und am Bild `reports/native-game-shots/ng2/shot-01.png` (Agent und Claude: beide Zellen mit Schichten, Ufer, Fluss).
Abweichung: Scroll-Faktoren und Biom-Namen stehen als Tabelle im Code (nicht in `data/`). Dass `fern` steht, belegt der Test; im Bild schwer zu sehen.
