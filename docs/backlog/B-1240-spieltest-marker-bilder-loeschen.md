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

Jeder Marker bekommt ein Bild (frames/marker-<nr>.jpg). Viele sind doppelt oder unnötig; der Chat liest sie mit playtest_frame oder über den Bildpfad und verbraucht dabei Tokens.

## Ziel

Alle Marker-Bilder sind vorab ausgewählt; Ralf nimmt unerwünschte oder doppelte heraus. Nur ausgewählte Bilder kommen in den Prompt (playtest_get) und werden später vorab beschrieben (B-1241). Nichts wird gelöscht.

## Beteiligte und Zielgruppen

Ralf (🧑), Claude als Leser der Session.

## Anforderungen

- Haken am Marker-Bild der Zeitleiste, vorab gesetzt; Haken weg blendet das Bild ab (abgedunkelt).
- Die Markierung steht in session.json › skipFrames (Nummern der abgewählten Marker); Datei und Marker bleiben.
- playtest_get nennt bei abgewählten Bildern keinen Pfad, sondern „Bild nicht ausgewählt“.
- Gesperrt während der Aufnahme.

## Nicht-Ziele

Dateien löschen (ersetzt: Ralf wollte markieren statt löschen); Mehrfachauswahl auf einmal.

## Regeln und Einschränkungen

Atomic Design; Pfad nur aus Session-Kennung und Nummer, nie vom Client.

## Beispiele

Marker #3 ist ein Doppel von #2: Haken bei #3 entfernen.

## Ausnahme- und Fehlerfälle

Nummer 0 oder Kennung mit Pfadteilen: abgelehnt. Doppeltes Abwählen ändert nichts.

## Akzeptanzkriterien

- **AC-01** Go-Test SetFrameSkip: Markierung, Datei bleibt, ungültige Eingaben abgelehnt.
- **AC-02** Vitest: Haken abwählen und wieder anwählen, Marker bleibt.
- **AC-03** Abnahme durch Ralf.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
