# BS10 · DEV · Grafiken für Hafen und Schiff

- **Status:** geplant
- **Projekt:** BAS
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-641
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach BS6 zeichnet der Client Hafen und Schiff mit Platzhaltern. Grafiken entstehen nach `docs/standards/gebaeude-grafik.md` (JSON in `public/grafik/gebaeude/`, `task gen:gebaeude`, Asset-Studio-Vorschau, Asset-Slots in `data/assets.json`).

## Ziel

Hafen und Schiff haben eigene Grafiken im Stil der übrigen Gebäude, je Bauabschnitt, und das Spiel zeigt sie.

**Am Ende sichtbar:** Asset-Studio › Szene Gebäude mit Hafen und vier Schiff-Stufen; im Spiel wächst das Schiff am Steg sichtbar.

## Beteiligte und Zielgruppen

Spieler am TV, Claude, 🧑 nimmt im Studio und am Gerät ab.

## Anforderungen

B-641 › Anforderungen.

## Nicht-Ziele

Animation, Bootsfahrt, Sound; Baumschul-Grafik (nutzt die Farm-Grafik weiter, eigenes Ticket bei Bedarf).

## Regeln und Einschränkungen

Letzter Sprint von BAS, abhängig von BS6 (Platzhalter und Client-Einbau) und BS5 (Zahl der Abschnitte). Hardware-Abnahme entkoppelt. ⚠️ Neue PNGs sind Dateien unter `public/` – der Spielserver startet selbst neu; das Studio braucht keinen Neustart, die Workbench nicht.

## Beispiele

B-641 › Beispiele.

## Ausnahme- und Fehlerfälle

B-641 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Daten-JSON mit Vorlage und Bausteinen für den Hafen; `task gen:gebaeude` rendert alle Größen (B-641/AC-01, Hafen).
- **AC-02** Daten-JSON für das Schiff je Abschnitt und fertig; Renderer-Tests grün (B-641/AC-01, Schiff).
- **AC-03** Asset-Slots in `data/assets.json`, der Client zeichnet die Grafiken statt der Platzhalter (B-641/AC-02).
- **AC-04** Abnahme im Studio und am Gerät durch 🧑 (B-641/AC-03).

## Offene Fragen

keine (Zahl der Abschnitte aus BS5).

## Sessions

Reife Entwurf, Stichpunkte:
- BS10.1 · DEV · Hafen: Vorlage, Bausteine, Rendern (AC-01)
- BS10.2 · DEV · Schiff je Abschnitt und fertig (AC-02)
- BS10.3 · CLI · Asset-Slots und Einbau im Client (AC-03)
- BS10.4 · Review (alle)
- BS10.5 · Abnahme im Studio und am Gerät, Mensch (AC-04)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
