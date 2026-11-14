# KB5.3 · Client zeichnet DoTs, Buffs und Kontrolleffekte (B-675)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** kb5/3-status-darstellung
- **Abhängig von:** KB5.2
- **Tickets:** B-675
- **Kriterien:** AC-02

## Ziel

Der Client zeichnet Betäubung, Verlangsamung, Spott, Sturm-Fläche, Schild, Auren und Regeneration aus dem Snapshot.

## Kontext

Spec: `docs/backlog/B-675-status-effekte-sichtbar.md`. Snapshot-Felder aus KB5.2. Zeichnen: `src/scenes/render/worldRenderer.ts`, Effekte `src/scenes/effects/` (Muster `skillLooks.ts` für Werte, damit das Studio sie importiert), Leistung `src/scenes/perfProbe.ts`.

## Erlaubte Dateien

- `src/scenes/**`

## Nicht-Ziele

- Snapshot-Änderungen (KB5.2), Studio (KB5.4), Ton.

## Schritte

1. Werte je Status-Effekt in `src/scenes/effects/statusLooks.ts`.
2. Darstellung mit Pool, Split-Screen tauglich.
3. Vitest je Effektart (zeichnen, entfernen).
4. `task check:diff`.

## Fertig, wenn

- [x] B-675 AC-03: Vitest jede Effektart gezeichnet und entfernt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- B-675 AC-03 geprüft: `src/scenes/effects/statusLooks.test.ts` – je Effektart (stun, slow, taunt, storm, shield, lastStand, aura) gezeichnet und ohne Effekt wieder leer; Sturm mit Radius am Boden, Aura in der Farbe ihres Skills, Holy Ground an der Burg, Schild-Balken nach Schild-HP, Boss-Symbole höher, tote Gegner und gefallene Monarchen ohne Anzeige; `StatusView` leert beide Flächen je Bild.
- Umsetzung: Werte und Zeichnung in `statusLooks.ts` (`STATUS_LOOKS`, `statusMarks`, `drawMark`, Zeichenfläche `StatusPainter`, damit das Studio sie nutzt), Phaser-Anteil `statusView.ts` mit zwei Graphics je Stufe (Boden unter, Symbole über den Figuren), je Bild geleert, keine neuen Phaser-Objekte je Takt; Split-Screen über die Ebene der Stufe. Eingebaut in `StageView` und `GameScene.draw` (`storms` direkt aus dem Zustand, da `applyState` ein verschwundenes Feld sonst stehen ließe).
- Offene Frage B-675 (Symbole gezeichnet oder Grafik-Slots) mit Default entschieden: gezeichnet. Regeneration und Frost Armor erscheinen als Aura-Ring (Radius 0 → eine Unit).
- Prüfung: `tsc` grün, oxlint `src` ohne Fehler, Vitest `src` und `tests` grün (123 Dateien). `task check:diff` rot nur durch fremde Studio-Chat-Änderungen (worker-2). Keine Prüfung im Browser oder mit `perfProbe` (manuell, AC-04 bei 🧑 in KB5.6).