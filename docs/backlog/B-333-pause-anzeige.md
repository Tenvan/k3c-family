# B-333 · Der Client zeigt einen angehaltenen Raum deutlich an und hält die Figuren-Animationen an

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** AZ1
- **Projekt:** BED
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-10, 🧑 im Chat (mit AZ1)

## Ausgangslage

Bei der Abnahme DBG3.4 (2026-10-07, PC) hat 🧑 einen Raum über `/dm` mit „⏸ Pause“ angehalten. Die Welt steht, aber im Spielbild fehlt eine deutliche Pause-Anzeige, und die Figuren laufen in ihrer Animation weiter. Der Zustand meldet die Pause schon (`devPaused`, `src/scenes/debugOverlay.ts` zeigt nur im Debug-Overlay „Raum angehalten“).

## Ziel

Wer auf das Spielbild schaut, erkennt sofort, dass der Raum angehalten ist; die Welt wirkt auch optisch eingefroren.

## Beteiligte und Zielgruppen

Spieler am TV oder PC; 🧑 als Spielleiter mit `/dm`. 🧑 nimmt ab.

## Anforderungen

- Ist der Raum angehalten, zeigt jeder Bildschirmbereich (Split-Screen, 2+ Spieler) eine gut lesbare Pause-Anzeige.
- Während der Pause stehen die Animationen der Figuren (Monarchen, Bürger, Truppen, Gegner, Tiere) still; nach „Weiter“ laufen sie wieder.
- Die Anzeige verschwindet, sobald der Raum weiterläuft.

## Nicht-Ziele

Pause im Couch-Raum durch den Server (B-214); Optionen- und Pause-Szene des Clients (B-146, erledigt); Änderungen an Server oder Protokoll.

## Regeln und Einschränkungen

Domäne CLI, nur zeichnen (keine Spiel-Logik im Client, ADR 001). Texte über `t()` in de und en. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Raum läuft, `/dm` → „⏸ Pause“ → im Spielbild erscheint „Pausiert“, alle Figuren stehen in ihrer aktuellen Pose. `/dm` → „▶ Weiter“ → Anzeige weg, Animationen laufen.

## Ausnahme- und Fehlerfälle

Verbindung bricht während der Pause ab → die vorhandene Verbindungsanzeige hat Vorrang, die Pause-Anzeige verdeckt sie nicht. Zustand ohne Pause-Feld → keine Anzeige.

## Akzeptanzkriterien

- **AC-01** Bei angehaltenem Raum zeigt jeder Spieler-Bildschirmbereich eine Pause-Anzeige; läuft der Raum, ist sie weg (Test der Anzeige-Funktion + Beobachtung am PC).
- **AC-02** Während der Pause stehen die Figuren-Animationen still und laufen nach „Weiter“ wieder (Beobachtung am PC).

## Offene Fragen

- Aussehen der Anzeige (Text mittig, abgedunkeltes Bild, Symbol): 🧑, nicht blockierend.

## Notizen

Anmerkung von 🧑 bei DBG3.4 (2026-10-07): „Im Pausemodus sollte eine entsprechende Anzeige erscheinen und auch die Figuren Animationen stehen bleiben.“
