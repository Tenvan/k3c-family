# W15 · SIM · Fackeln als Spielobjekt mit Licht in der Nacht

- **Status:** aktiv
- **Projekt:** WRT
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-622
- **Start-Commit:** 29493645
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

## Ausgangslage

Beim Licht der Nacht (W13) fehlt ein eigenes Spielobjekt für Fackeln (B-622).

## Ziel

Fackeln stehen in der Welt (z. B. an Mauern und Türmen) und leuchten nachts. Am Ende sichtbar: Nacht mit Fackeln an der Mauer im Spiel.

## Beteiligte und Zielgruppen

Spielende; Agent SIM (dann CLI und DEV für Darstellung); 🧑 entscheidet über Ort und Optik.

## Anforderungen

`B-622 › Anforderungen`.

## Nicht-Ziele

Neue Beleuchtungs-Engine; Balancing.

## Regeln und Einschränkungen

Client zeichnet nur, was der Server meldet; deterministisch; Protokoll nur additiv nach Entscheidung von 🧑.

## Beispiele

Nacht → Fackel an der Mauer leuchtet, Tag → aus.

## Ausnahme- und Fehlerfälle

Fackel an zerstörter Mauer → entfällt.

## Akzeptanzkriterien

- **AC-01** Fackeln sind Spielobjekte und leuchten nachts (`B-622/AC-01`).

## Offene Fragen

keine. Entschieden (🧑, 2026-10-31): Fackeln sind Teil von Mauer und Turm, kein eigener Kauf und keine neue Bauoption; sie leuchten nachts.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W15.1 | `W15.1-fackeln-an-mauer-und-turm.md` | Umsetzung | autonom | fertig |
| W15.2 | `W15.2-review.md` | Review | autonom | fertig |
| W15.3 | `W15.3-abnahme-am-geraet.md` | Workshop | Mensch | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W15.1 (SIM) Fackel als Objekt (AC-01).
- W15.2 Review, alle Kriterien, schließt ab.

## Abnahme

2026-11-02, Review W15.2: AC-01 geprüft (go:test Fackel, nightLight, noSim, projectRules grün; Sim und Client teilen testdata/torches.json).
Befund niedrig: TorchXs in der Sim nur von Tests genutzt, kein Fix. Keine neuen Tickets.
Offen: Abnahme am Gerät (W15.3, Mensch), B-622 bleibt bis dahin offen; check, check:go, check:race stehen vor dem Commit aus.
