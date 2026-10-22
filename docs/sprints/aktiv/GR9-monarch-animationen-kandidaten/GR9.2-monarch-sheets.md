# GR9.2 · Monarch-Sheets für Wurf, Treffer, Tod und Respawn erzeugen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** gr9/2-monarch-sheets
- **Abhängig von:** –
- **Tickets:** B-505
- **Kriterien:** AC-02

## Ziel

Beide König-Sheets (`medieval-king`, `medieval-king-2`) haben die Animationen `give` (Wurf/Münze geben), `hit`, `death` und `respawn` als PNG-Streifen, deterministisch aus den vorhandenen Frames erzeugt.

## Kontext

- Workshop-Frage (Quelle der Animationen) autonom entschieden (Claude, 2026-10-22): keine neue Fremdquelle, Ableitung aus vorhandenen Frames (CC0, LuizMelo). 🧑 prüft die Optik in GR9.5 / Sprint GR10.
- Erzeuger: neues Unterkommando oder Paket in `tools/spritegen` (Go, Stdlib, deterministisch, kein `math/rand`), liest `public/sprites/<sheet>/idle.png` und `attack.png`.
- Ableitung: `give` = erste Frames von `attack` (Arm vor); `hit` = idle-Frame weiß aufgeblitzt (Mischung mit Weiß) und 1–3 px zurückgestoßen, 3 Frames; `death` = idle-Frame schrittweise um den Fußpunkt bis ca. 90° gekippt (Nearest-Neighbor), 5 Frames, letzter Frame hält; `respawn` = idle-Frame wächst von unten ein (Clip) mit Aufblitzen, 5 Frames.
- Frame-Größen bleiben (155×155 bzw. 160×111; Kippen darf nicht abschneiden: Fußpunkt-Rotation auf großem Canvas, dann auf Frame zuschneiden und prüfen).
- `data/sprites.json`: je Sheet `anims.give/hit/death/respawn` mit `frames` und `fps`; Test `tests/sprites.test.ts` verlangt Breite = frames × frameWidth.

## Erlaubte Dateien

- `tools/spritegen/**`, `public/sprites/medieval-king/**`, `public/sprites/medieval-king-2/**`, `data/sprites.json`, `public/sprites/CREDITS.md` (Abgeleitet-Vermerk)
- `tests/sprites.test.ts` nur bei nötiger Anpassung

## Nicht-Ziele

Abspielen im Client (GR9.3), Atlas-Code (GR9.3), neue Fremdgrafik.

## Schritte

1. Generator schreiben und Tests (deterministisch, Größen, nichts abgeschnitten).
2. Sheets erzeugen, ansehen, nachbessern.
3. `sprites.json` ergänzen; gezielte Tests.

## Fertig, wenn

- [ ] AC-02 (Teil Sheets): je König-Sheet vier neue PNG-Streifen, in `sprites.json` eingetragen, `tests/sprites.test.ts` und `go test ./tools/spritegen` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-02 (Teil Sheets) geprüft: `go run ./tools/spritegen -monarch` (`tools/spritegen/monarch.go`) schreibt je König-Sheet give/hit/death/respawn (King 4/3/5/5, King 2 3/3/5/5 Frames), in `sprites.json` eingetragen; `go test ./tools/spritegen`, `tests/sprites.test.ts` grün, Vorschau von Claude angesehen.
- Abweichung: Tod kippt nach vorn (Blickrichtung) und wird auf den Boden angehoben, nichts abgeschnitten (Test); give hängt idle-Frame 0 als Rückkehr an. Optik prüft 🧑 in GR9.5; keine neuen Tickets.
