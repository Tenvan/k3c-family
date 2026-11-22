# B-1240 · Einzelne Marker-Bilder einer Session lassen sich löschen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Auftrag im Chat
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Jeder Marker bekommt ein Bild (frames/marker-<nr>.jpg). Viele sind doppelt oder unnötig; der Chat liest sie mit playtest_frame und verbraucht Tokens.

## Ziel

Ralf löscht einzelne Marker-Bilder auf der Seite Spieltest; Auswertung und Chat berücksichtigen nur vorhandene Bilder.

## Beteiligte und Zielgruppen

Ralf (🧑), Claude als Leser der Session.

## Anforderungen

- Kleiner Löschen-Knopf am Bild eines Markers mit Bestätigung.
- Go-Methode löscht nur frames/marker-<nr>.jpg der gewählten Session, nie andere Pfade.
- Zeitleiste und playtest_get zeigen den Marker weiter, ohne Bild.
- Erneutes Auswerten erzeugt gelöschte Bilder wieder.

## Nicht-Ziele

Mehrfachauswahl; Bilder nachträglich neu ziehen ohne Auswertung.

## Regeln und Einschränkungen

Atomic Design; Pfad nur aus Session-ID und Nummer, nie vom Client.

## Beispiele

Marker #3 ist ein Doppel von #2: Bild von #3 löschen.

## Ausnahme- und Fehlerfälle

Bild fehlt schon: kein Fehler. Aufnahme läuft: gesperrt.

## Akzeptanzkriterien

- **AC-01** Go-Test: löscht nur das genannte Bild, lehnt ungültige Nummer ab.
- **AC-02** Vitest: Knopf entfernt das Bild aus der Zeitleiste.
- **AC-03** Abnahme durch Ralf.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
