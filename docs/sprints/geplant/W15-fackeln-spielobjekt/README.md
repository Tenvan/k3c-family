# W15 · SIM · Fackeln als Spielobjekt mit Licht in der Nacht

- **Status:** geplant
- **Projekt:** WRT
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-622
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

B-622 hat Spec Entwurf: Ort, Kosten, Grafik. Entscheidet 🧑 vor der Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W15.1 (SIM) Fackel als Objekt (AC-01).
- W15.2 Review, alle Kriterien, schließt ab.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
