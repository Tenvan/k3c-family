# U1 · CLI · Radar im HUD

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-090
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Ausblendbar per Taste? Gegner einzeln oder verdichtet? (B-090 › Offene Fragen, entscheidet 🧑 vor der Freigabe.)

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- U1.1 Marker-Logik als reine Funktion mit Tests (AC-01).
- U1.2 Radar zeichnen in allen Layout-Zellen, Test für den freien Home-Button-Streifen (AC-02, AC-03).
- U1.3 Review (alle); AC-04 ist die Abnahme durch 🧑 am TV.

## Abnahme

–
