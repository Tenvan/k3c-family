# B-599 · Szenen auf Sound- und Grafik-Seite sind sortier- und filterbar

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ29
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Szenen-Ansichten (Sound: Ereignis-SFX, Musik, Umgebung; Grafik: ScenesView) zeigen lange, nur in fester Reihenfolge sortierte Listen. Ralf (2026-10-24): Szenen besser sortier- und filterbar machen, für Ton und Grafik.

## Ziel

Man findet jeden Eintrag schnell durch Filter, Suche und Sortierung, auf beiden Seiten gleich bedient.

## Beteiligte und Zielgruppen

Ralf; Agent DEV.

## Anforderungen

- Gemeinsamer Baustein in `@webkit` (Molecule): Suchfeld, Filter-Chips, Sortierung, Zähler „n von m“, Zurücksetzen; Auswahl wird gemerkt.
- Filter: Art/Gruppe, Status (zugeordnet, Lücke, Platzhalter), Linie/Klasse, Biom/Tiefe, Bus (Sound), Lizenz (CC0, CC-BY, NC), „noch nicht gehört/gesehen“, „vom Chat vorgeschlagen“.
- Sortierung: Name, Art, Status, Reihenfolge im Spiel, zuletzt geändert.
- Gruppieren mit einklappbaren Abschnitten.
- Tastatur: `/` fokussiert die Suche.

## Nicht-Ziele

Neue Szenen (B-597, B-598); Datenbearbeitung.

## Regeln und Einschränkungen

Reine Filter-/Sortierfunktionen mit Vitest, Atomic Design; kein Netz; läuft für 1000 Einträge flüssig.

## Beispiele

Sound › Szenen › Filter „Lücke“ + „Heiler“ → nur Heiler-Skills ohne Ton.

## Ausnahme- und Fehlerfälle

Kein Treffer → Hinweis mit „Filter zurücksetzen“.

## Akzeptanzkriterien

- **AC-01** Der Baustein filtert, sucht und sortiert (Vitest der reinen Funktionen).
- **AC-02** Sound- und Grafik-Szenen nutzen ihn und merken die Auswahl (Test, Beobachtung).
- **AC-03** Gruppieren und Tastatur `/` funktionieren.

## Offene Fragen

keine

## Notizen

Aus Ralfs Wunsch 2026-10-24. Spec: Entwurf. Sprint WZ29.
