# GR3 · CLI · Grafik im Renderer

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-010
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gebäude und Bauplätze sind Rechtecke in `src/scenes/worldRenderer.ts`; kein Spiel-Code lädt `public/grafik/`. Zuordnung (GR1) und Lückenschluss (GR2) liegen vor.

## Ziel

Gebäude, Ressourcen und Hintergründe werden mit den zugeordneten Grafiken gezeichnet. Am Ende sichtbar: Hub mit Sprites statt Formen, Parallax je Biom, Hub- und Materialstufen unterscheidbar am TV.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; Umsetzung durch Agent; 🧑 nimmt am TV ab.

## Anforderungen

B-010 › Anforderungen; Zuordnung aus `docs/assets/zuordnung.md`. Sprint-eigen: Hub-Stufen 1–5 und Mauer-/Turm-Materialstufen sichtbar (sobald B-112 sie liefert, vorher Stufe 1). Das Ticket B-010 hat nur zwei grobe Kriterien; vor der Freigabe prüfen, ob seine Spec nachgeschärft wird.

## Nicht-Ziele

Reittiere (Mechanik fehlt, B-152), Atlas und Lade-Szene (GR4), Effekte (GR5), neue Grafiken (GR2).

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts (`noSim.test.ts`); Seiten-Regeln aus `CLAUDE.md`; B nicht belegen; 2 Spieler im Split-Screen; Datei ≤ 400 Zeilen (`worldRenderer.ts` bei Bedarf teilen).

## Beispiele

Mauer im Hub → Sprite aus der Zuordnung; Werkstatt ohne Grafik → Platzhalter-Form bleibt.

## Ausnahme- und Fehlerfälle

Grafik fehlt oder lädt nicht → Platzhalter-Form, keine leere Stelle.

## Akzeptanzkriterien

- **AC-01** Alle Gebäude, Ressourcen und Parallax-Ebenen, die in der Zuordnungstabelle „zugeordnet“ sind, werden als Sprites gezeichnet (B-010/AC-01).
- **AC-02** Die Credits aller eingebauten Grafiken stehen in `public/` (B-010/AC-02).
- **AC-03** Fehlt eine Grafik, bleibt die Platzhalter-Form sichtbar (Test oder Beobachtung).
- **AC-04** Hub-Stufen und Mauer-/Turm-Materialstufen sind am TV unterscheidbar (Beobachtung).
- **AC-05** Wald, Höhle und Mine haben je eigene Parallax-Ebenen oder einen dokumentierten Platzhalter (Beobachtung).
- **AC-06** `task check` ist grün, mit 2 Spielern im Split-Screen sind keine Darstellungsfehler zu sehen.

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GR3.1 Gebäude, Bauplätze, Hub- und Materialstufen als Sprites, Platzhalter-Rückfall (AC-01, AC-03, AC-04).
- GR3.2 Ressourcen, Adern, Plantage und Parallax je Biom, Credits prüfen (AC-02, AC-05).
- GR3.3 Review (AC-06).

## Abnahme

–
