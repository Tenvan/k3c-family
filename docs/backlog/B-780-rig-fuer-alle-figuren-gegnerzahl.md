# B-780 · Prototypen zeigen, wie viele Gegner, Truppen und Bosse gleichzeitig als Rig-Modelle laufen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** NG3
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Zielbild (🧑 2026-11-11): Später sind möglichst alle Figuren Rig-Modelle aus Teilen statt PNG-Sheets, mit Waffen und Rüstungen als optionalen Elementen bei Monarch, Gegnern und Bossen. Ein Rig kostet je Figur und Bild mehr Rechnung und mehr Zeichenaufrufe als ein Sheet-Frame; das kann die Zahl gleichzeitiger Gegner begrenzen. Der Prototyp B-778 misst nur Monarch und Pferd.

## Ziel

Messwerte und Prototypen zeigen, wie viele Gegner, Truppen und Bosse gleichzeitig als Rig laufen, und daraus folgt die Reihenfolge, in der Figuren von Sheets auf Rig wechseln.

## Beteiligte und Zielgruppen

🧑 Ralf (Entscheidung über Obergrenzen und Reihenfolge), Claude (Prototypen, Messung).

## Anforderungen

- Lasttest mit gemischten Figuren als Rig (Gegner mit Waffe und Rüstung, Truppen, ein Boss) in den Mengen des Spiels: größte Welle, vier Spieler, Mini-PC.
- Grenze je Figurentyp, ab der die Bildrate unter das Ziel fällt; Vergleich mit Sheets.
- Wege zur Entlastung prüfen: Posen teilen (viele Gegner, wenige Phasen), vereinfachte Rigs in der Ferne, Rig nur für wichtige Figuren.
- `gen:paper` schrumpft Figur für Figur: Eine Figur verlässt das Gen-Skript erst, wenn ihr Rig im Spiel läuft.

## Nicht-Ziele

Umstellung der Figuren selbst (eigene Sprints je Gruppe); neue Figuren, Waffen oder Rüstungen zeichnen.

## Regeln und Einschränkungen

Entscheidung 005 und 008. Das Rig-Format aus B-778 (Teile-Baum und Plätze) gilt für alle Figuren. Balancing-Werte (Gegnerzahl je Welle) ändert dieses Ticket nicht; eine nötige Obergrenze wird als eigenes Ticket vorgelegt. Kein Lasttest über `k3c-load` oder `sim_test` (Sperre 2026-11-11): gemessen wird die Darstellung im Spiel-Modul mit einer festen Szene, je Fassung ein Lauf. Teil Entwicklungs-PC im Sprint NG3 (Session NG3.4), Teil Mini-PC mit B-786 in NG9.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/actors/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe, Entscheidung, Gerät
- **Größe:** M
- **Sperre:** –

## Beispiele

200 Käfer als Rig mit je einer Waffe auf dem Mini-PC: Bildrate, Grafikspeicher und Rechenzeit je Bild gegen 200 Käfer aus Sheets.

## Ausnahme- und Fehlerfälle

Die größte Welle des Spiels läuft als Rig nicht flüssig → Vorschlag: Mischbetrieb (Sheets für Massen-Gegner) oder Obergrenze, Entscheidung 🧑.

## Akzeptanzkriterien

- **AC-01** Messbericht mit der Grenze je Figurentyp (Rig gegen Sheet) vom Entwicklungs-PC und vom Mini-PC.
- **AC-02** Empfehlung mit Reihenfolge der Umstellung und Stand von `gen:paper` je Gruppe, von 🧑 entschieden.

## Offene Fragen

keine.

Entschieden von 🧑 am 2026-11-11 (Chat):

- **Maßstab:** 60 FPS bei 2 Spielern mit der Gegnerzahl der größten Welle laut `data/` (gelesen, nicht simuliert).
- **Waffen und Rüstungen für Gegner:** erst nach der Messung, als eigenes Ticket.

## Notizen

Angelegt 2026-11-11 aus dem Chat mit 🧑. Baut auf B-778/AC-05 auf (2, 50, 200 Rig-Figuren).

### Messung Entwicklungs-PC (NG3.4, 2026-11-11)

Echtes Fenster, 1920×1080, zwei Zellen, ohne VSync, je Fassung ein Lauf (60 Bilder Aufwärmen, 300 gemessen), Bericht `reports/native-actors-ng3.json`. Größte Welle laut `data/` (gelesen, nicht simuliert): 50 Gegner = (20 Standard + 5 Elite) × 1,25 bei 2 Spielern × 1,6 (ultra). Werte: Bildrate / Rechenzeit der Figuren je Bild.

| Figur | Fassung | 2 Gegner | 50 Gegner | 200 Gegner |
|---|---|---|---|---|
| Goblin | Rig | 1557 FPS / 0,23 ms | 926 FPS / 0,75 ms | 371 FPS / 2,30 ms |
| Goblin | Sheet | 2226 FPS / 0,13 ms | 2382 FPS / 0,14 ms | 1841 FPS / 0,31 ms |
| Wolf | Rig | 2109 FPS / 0,14 ms | 587 FPS / 1,45 ms | 172 FPS / 5,63 ms |
| Wolf | Sheet | 2713 FPS / 0,14 ms | 1593 FPS / 0,17 ms | 2359 FPS / 0,34 ms |

Die größte Welle liegt am Entwicklungs-PC weit über 60 FPS. Vorschlag zur Reihenfolge (Entscheidung 🧑 offen): Monarch und Truppen, dann Zweibeiner-Gegner, zuletzt Vierbeiner-Gegner (am teuersten); Mischbetrieb erst, wenn der Mini-PC (NG9.5) unter das Ziel fällt.
