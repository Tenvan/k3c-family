# W11 · REG, SRV, CLI · Bauplätze im Protokoll, Abstand der Treppen, Ausbilden und Tauschen, Figuren

- **Status:** aktiv
- **Projekt:** WRT
- **Domäne:** REG, SRV, CLI
- **Reife:** bereit
- **Tickets:** B-409, B-208, B-410, B-408, B-412
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt)

## Ausgangslage

Aus der Sichtung von W6 (2026-10-11): Bauplätze stehen im Hub zu eng, besonders die Treppen (B-409, `data/hub.json › sites`, je 4 Units Abstand); Plätze „aller Stufen“ erscheinen schon im Hub einer niedrigen Stufe (B-408, Ursache Server oder Client offen); das Protokoll trägt gestreute Bauplätze, Platz- und Hub-Stufe nicht zum Client (B-208); `actions` enthält Ausbilden und Tauschen nicht (B-410, `docs/protocol.md`); Adern, Plantage und Lager haben keine Figuren (B-412).

## Ziel

Der Client zeigt nur Bauplätze, die zur Hub- und Stufen-Lage passen, in lesbarem Abstand, dazu Ausbilden, Tauschen und die Wirtschafts-Figuren. Am Ende sichtbar: Hub am TV ohne überlappende Preisschilder, Aktionen-Overlay mit Ausbilden und Tauschen, Adern, Plantage und Lager als Figuren; von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spielende am TV; Agent in REG, SRV und CLI nacheinander; 🧑 nimmt am Gerät ab.

## Anforderungen

B-409, B-208, B-410, B-408, B-412 › Anforderungen (Specs noch Entwurf).

## Nicht-Ziele

Neue Gebäude oder Wirtschaftsregeln; Gesperrt-Gründe und Rohstoff-Typen (W8: B-207, B-209, B-188); Ausrüstung und Spielstand (W7, SV1).

## Regeln und Einschränkungen

Reihenfolge REG → SRV → CLI. Mindestmaß der Zahlziele ≥ 4 Units (Glossar › Zahlziel) bleibt. Protokoll-Änderungen (W11.2, W11.3) mit Beispielen unter `testdata/protocol/`, Version nach Absprache. Der Client rechnet nichts. Der Spielstand bleibt kompatibel oder wird migriert (Golden, F2).

## Beispiele

Hub-Stufe 1 → Plätze mit `hubLevel` > 1 und `fromDepth` > Stufe werden nicht als baubar gezeigt. Treppe hoch und runter stehen mindestens 8 Units von jedem anderen Zahlziel entfernt (Burg, Händler, Plätze, Angebote, Linien).

## Ausnahme- und Fehlerfälle

Alter Server ohne neue Felder → Client zeigt Plätze wie bisher mit Warnung 🤒. Unbekannte Aktion in `actions` → ignorieren, Log.

## Akzeptanzkriterien

- **AC-01** Die Bauplätze im Hub haben bei den Treppen mindestens 8 Units Abstand, `task balance` und Golden bleiben grün (B-409).
- **AC-02** Das Protokoll trägt Bauplätze mit Platz- und Hub-Stufe; nicht erreichbare Plätze erscheinen nicht (B-208, B-408).
- **AC-03** Der Server liefert Ausbilden und Tauschen in `actions` (B-410).
- **AC-04** Der Client zeigt Ausbilden und Tauschen im Aktionen-Overlay sowie Adern, Plantage und Lager als Figuren; von 🧑 am TV abgenommen (B-408, B-410, B-412).

## Offene Fragen

Geklärt mit 🧑 (2026-10-14): W11 vor W8; Treppen ≥ 8 Units zu **allen** Zahlzielen derselben Tiefe. „Linien +8“ verworfen (Kristallhöhle: Portale ab 135, Camps ohne freien Chunk); stattdessen teilen sich Treppe hoch (ab Tiefe 1) und Händler-Kaufen (nur Tiefe 0) ein Raster, Linien bleiben. B-408: Der Server filtert; „erreichbar“ heißt Platz-Stufe ≤ Hub-Stufe + 1 (bezahlte und gebaute Plätze immer), damit W8 die nächste Stufe als gesperrt zeigen kann. Protokoll: keine neue Version (additiv, Vorschlag W11.2). Offen: SV1-Verweis auf B-208 anpassen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W11.1 | `W11.1-treppen-abstand.md` | Umsetzung | autonom | fertig |
| W11.2 | `W11.2-bauplaetze-im-zustand.md` | Umsetzung | autonom | in Arbeit |
| W11.3 | `W11.3-aktionen-ausbilden-tauschen.md` | Umsetzung | autonom | offen |
| W11.4 | `W11.4-bauplaetze-figuren-client.md` | Umsetzung | autonom | offen |
| W11.5 | `W11.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
