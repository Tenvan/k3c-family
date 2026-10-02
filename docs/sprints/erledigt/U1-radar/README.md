# U1 · CLI · Radar im HUD

- **Status:** erledigt
- **Domäne:** CLI
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-090
- **Start-Commit:** be978e0
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02 🧑 Chat („Ja, freigeben und umsetzen“; Revision 1)

## Ausgangslage

Der Client zeichnet pro Spieler-Zelle nur einen Kamera-Ausschnitt des Levels; Burg, Portale, Ausgang, Mitspieler und Gegner außerhalb des Bildes sind unsichtbar (B-090).

## Ziel

Jeder Spieler sieht in seiner Zelle eine Radar-Leiste über das ganze Level. Am Ende sichtbar: im Spiel mit zwei Spielern und einer Nacht-Welle wandern die Gegner-Marker vom Portal zur Burg, die Monarchen sind unterscheidbar.

## Beteiligte und Zielgruppen

Spieler am Sofa; 🧑 nimmt am TV ab.

## Anforderungen

B-090 › Anforderungen.

## Nicht-Ziele

Randmarker, Nebel des Krieges, Interaktion mit dem Radar (B-090 › Nicht-Ziele); Änderungen an Server und Protokoll.

## Regeln und Einschränkungen

Domäne CLI (`src/scenes`, `src/model`). `src/scenes` rechnet nichts; Marker-Logik in eigener Datei mit Test. Datei ≤ 400 Zeilen, Funktion ≤ 60. B-Taste und View + Menu unbelegt. Die Abnahme am TV macht nur 🧑.

## Beispiele

Zwei Spieler, Nacht, Welle am linken Portal → beide Zellen zeigen rote Marker links, die Burg in der Mitte und die beiden Monarchen.

## Ausnahme- und Fehlerfälle

Level ohne Portale → keine Portal-Marker; Spieler am Boden → gedämpfter Marker (B-090 › Ausnahme- und Fehlerfälle).

## Akzeptanzkriterien

- **AC-01** Marker-Funktion getestet (B-090/AC-01).
- **AC-02** Radar in allen Layout-Zellen, Home-Button-Streifen frei (B-090/AC-02).
- **AC-03** `task check` grün, `src/scenes` ohne Spiel-Logik (B-090/AC-03).
- **AC-04** 🧑 hat das Radar am TV abgenommen (B-090/AC-04).

## Offene Fragen

keine. Entschieden 2026-10-02 durch 🧑 (Chat): Das Radar ist immer sichtbar (kein Umschalter), Gegner erscheinen als einzelne Marker.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| U1.1 | `U1.1-marker-logik.md` | Umsetzung | autonom | fertig |
| U1.2 | `U1.2-radar-zeichnen.md` | Umsetzung | autonom | fertig |
| U1.3 | `U1.3-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-02, Review U1.3 (Agent): AC-01 belegt (U1.1 › Ergebnis), AC-02 und AC-03 belegt (U1.2 › Ergebnis, nachgeprüft in U1.3 › Ergebnis).
- AC-04 offen: wartet auf Abnahme durch 🧑 am TV.
- Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: keine.
