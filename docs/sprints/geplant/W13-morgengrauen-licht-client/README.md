# W13 · CLI · Morgengrauen im Client und Welt-Betrachter, dunklere Nacht mit Fackeln und Lagerfeuer als Licht

- **Status:** geplant
- **Projekt:** WRT
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-472, B-470
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach W12 kennt der Server die Phase `dawn`. Der Client zeichnet das Morgengrauen noch als letzte 10 % der Nacht (`daylight()` in `src/scenes/render/viewRules.ts`), der Welt-Betrachter im Studio ebenso (B-470/AC-04, AC-05). Nacht und Morgengrauen sind laut 🧑 zu hell, Fackeln und Lagerfeuer geben kein Licht (B-472).

## Ziel

Spiel und Welt-Betrachter zeigen das Morgengrauen als eigene Phase, die Nacht ist deutlich dunkler und Lichtquellen hellen ihre Umgebung auf. Am Ende sichtbar: dunkle Nacht mit Lichtkegeln am TV, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spielende am TV, zwei Spieler; 🧑 im Studio; Agent in CLI und DEV; 🧑 legt Helligkeit fest und nimmt am TV ab.

## Anforderungen

B-472 › Anforderungen; B-470 › Anforderungen für Client und Welt-Betrachter (B-470/AC-04, B-470/AC-05). Specs noch Entwurf.

## Nicht-Ziele

Licht in der Simulation; neue Lichtquellen als Spielobjekte (eigenes Ticket, falls es Fackeln noch nicht gibt); Werte der Simulation (W12).

## Regeln und Einschränkungen

Reihenfolge CLI → DEV. **Abhängigkeit:** nach W12. Darstellung rein im Client, Werte aus `data/`, eine Funktion `daylight()` für Spiel und Welt-Betrachter (keine zweite Kopie). 2 Spieler im Split-Screen, Xbox ohne spürbaren Einbruch.

## Beispiele

Nacht am Lagerfeuer → Umgebung im Radius heller, außerhalb dunkel. Welt-Betrachter mit Tageszeit „Morgengrauen“ → Phase `dawn`, gleiche Helligkeit wie im Spiel.

## Ausnahme- und Fehlerfälle

Überlappende Lichtquellen → nie heller als der Tag. Fehlen Licht-Werte in `data/` → kein Licht, Warnung 🤒.

## Akzeptanzkriterien

- **AC-01** Der Client zeichnet `dawn` als eigene Phase, Nacht und Morgengrauen sind dunkler, Fackeln und Lagerfeuer hellen auf (B-470/AC-04, B-472/AC-01, B-472/AC-02).
- **AC-02** Der Welt-Betrachter setzt Morgengrauen als Phase `dawn` und nutzt dieselbe Darstellung ohne zweite Kopie (B-470/AC-05, B-472/AC-03).
- **AC-03** 🧑 hat Helligkeit und Licht am TV mit zwei Spielern abgenommen (B-472/AC-02).

## Offene Fragen

🧑: Mindesthelligkeit, Radius und Stärke des Lichts (B-472 › Offene Fragen); Helligkeitskurve im Morgengrauen (B-470 › Offene Fragen 2).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W13.1 (CLI) Morgengrauen zeichnen, dunklere Nacht, Lichtkegel (AC-01).
- W13.2 (DEV) Welt-Betrachter mit Phase `dawn` und derselben Darstellung (AC-02).
- W13.3 (DEV) Review (Code-Sprint): alle Kriterien prüfen.
- W13.4 (CLI, Mensch) Abnahme am TV (AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
