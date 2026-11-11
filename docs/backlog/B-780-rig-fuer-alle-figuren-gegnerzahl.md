# B-780 · Prototypen zeigen, wie viele Gegner, Truppen und Bosse gleichzeitig als Rig-Modelle laufen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Entscheidung 005 und 008. Das Rig-Format aus B-778 (Teile-Baum und Plätze) gilt für alle Figuren. Balancing-Werte (Gegnerzahl je Welle) ändert dieses Ticket nicht; eine nötige Obergrenze wird als eigenes Ticket vorgelegt.

## Beispiele

200 Käfer als Rig mit je einer Waffe auf dem Mini-PC: Bildrate, Grafikspeicher und Rechenzeit je Bild gegen 200 Käfer aus Sheets.

## Ausnahme- und Fehlerfälle

Die größte Welle des Spiels läuft als Rig nicht flüssig → Vorschlag: Mischbetrieb (Sheets für Massen-Gegner) oder Obergrenze, Entscheidung 🧑.

## Akzeptanzkriterien

- **AC-01** Messbericht mit der Grenze je Figurentyp (Rig gegen Sheet) vom Entwicklungs-PC und vom Mini-PC.
- **AC-02** Empfehlung mit Reihenfolge der Umstellung und Stand von `gen:paper` je Gruppe, von 🧑 entschieden.

## Offene Fragen

- Ziel-Bildrate und größte Gegnerzahl, die als Maßstab gelten (aus den Balancing-Daten ableiten).
- Welche Gegner brauchen Waffen und Rüstungen überhaupt als eigene Elemente?

## Notizen

Angelegt 2026-11-11 aus dem Chat mit 🧑. Baut auf B-778/AC-05 auf (2, 50, 200 Rig-Figuren).
