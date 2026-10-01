# B-090 · Ein Radar im HUD zeigt Burg, Portale, Ausgang, Mitspieler und Gegner

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** U1
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ein Level ist bis zu ~1100 Units breit, die Kamera jedes Spielers zeigt nur einen Ausschnitt. Wo die Burg, die Portale, der Tiefen-Eingang, Mitspieler und anrückende Gegner sind, sieht man erst, wenn man hinläuft. Das HUD (`src/scenes/HudScene.ts`) zeigt nur Zahlen. Das Spiel kennt keine Sprünge, die Welt ist eindimensional; eine Leiste reicht.

## Ziel

Jeder Spieler sieht in seiner Bildschirmhälfte eine schmale Radar-Leiste über das ganze Level mit Burg/Hub, Portalen, Tiefen-Eingang bzw. Treppe, allen Monarchen, Gegnern und dem eigenen Kamera-Ausschnitt. Nutzen: Orientierung und rechtzeitige Warnung vor der Welle.

## Beteiligte und Zielgruppen

Alle Spieler am Sofa (auch mit 3–4 lokalen Spielern); 🧑 nimmt am TV ab.

## Anforderungen

- Das Radar zeichnet nur Daten aus dem vorhandenen `World`-Zustand des Clients; der Server und das Protokoll ändern sich nicht.
- Die Marker-Berechnung ist eine reine, getestete Funktion (Welt, Zelle → Markerliste), getrennt vom Zeichnen.
- Es passt in jede Zelle des Layouts für 1–4 lokale Spieler und für die Partner-Zelle, ohne Spielfeld oder Home-Button (oben ca. 70 px) zu verdecken.
- Spieler sind unterscheidbar (Farbe je Monarch), Gegner heben sich ab, die Leiste bleibt bei 2 Spielern gleichzeitig lesbar.

## Nicht-Ziele

Randmarker (Pfeile am Bildschirmrand), Nebel des Krieges, Zoom oder Interaktion mit dem Radar, Änderungen an Server oder Protokoll. Randmarker kommen bei Bedarf als eigenes Ticket.

## Regeln und Einschränkungen

`src/scenes` rechnet nichts, es zeichnet; neue Logik in einer eigenen Datei mit Test. Datei ≤ 400 Zeilen, Funktion ≤ 60. Controller-Taste B und View + Menu bleiben unbelegt. Jede Mechanik muss mit 2 Spielern gleichzeitig funktionieren.

## Beispiele

Zwei Spieler, einer am Hub, einer 300 Units links: beide Hälften zeigen beide Monarchen, die Burg in der Mitte und das eigene Sichtfenster. Nacht mit Welle am linken Portal: rote Gegner-Marker wandern von links zur Burg.

## Ausnahme- und Fehlerfälle

Level ohne Portale (Hub-Stufe) → keine Portal-Marker. Unter Tage mit Aggressionspool → Radar zeigt Gegner wie oben, kein Fehler. Spieler am Boden → Marker bleibt, wird gedämpft dargestellt.

## Akzeptanzkriterien

- **AC-01** Eine reine Funktion liefert aus einem `World` die Marker (Burg, Portale, Ausgang/Treppe, Spieler, Gegner, Sichtfenster); Vitest deckt Level mit und ohne Portale, 1 und 2 Spieler und einen Spieler am Boden ab.
- **AC-02** Das Radar erscheint in jeder Zelle der Layouts für 1, 2, 3 und 4 lokale Spieler sowie in der Partner-Zelle; ein Test belegt, dass keine Radar-Fläche den oberen Streifen für den Home-Button (70 px) berührt.
- **AC-03** `task check` ist grün, `src/scenes` bleibt frei von Spiel-Logik (`noSim`-Test grün).
- **AC-04** 🧑 hat das Radar am TV mit zwei Spielern und einer Nacht-Welle abgenommen.

## Offene Fragen

Soll das Radar ausblendbar sein (Taste, nicht B)? Sollen Gegner einzeln oder verdichtet (Häufchen) erscheinen? Entscheidet 🧑.

## Notizen

Im alten Godot-GDD nur als „Minimap, optional, Post-MVP“ (`FamilyCrowns/docs/gdd/ui_ux.md`).
