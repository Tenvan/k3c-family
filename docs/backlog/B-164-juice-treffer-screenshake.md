# B-164 · Treffer, Münzen und Bauen haben sichtbare Rückmeldung, Screenshake und Blitze sind abschaltbar

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** GR5
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint GR5

## Ausgangslage

Der Client zeichnet nur Zustand (`src/scenes/worldRenderer.ts`), Treffer, Kills und Münzen haben keine eigene Rückmeldung; die Simulation sendet noch keine Feedback-Events (F3 liefert sie, B-139; Protokoll B-140). Eine Optionen-Szene für Screenshake und Blitze kommt mit B-146.

## Ziel

Treffer-Blitz, Screenshake, Münz-Partikel sowie Todes- und Bau-Effekte machen das Spiel greifbar; Screenshake und Blitz sind abschaltbar, Controller-Vibration ist optional. Nutzen: Rückmeldung am TV, ohne dass der Client rechnet.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy, auch Kinder und Lichtempfindliche (Abschalten); Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Effekte reagieren nur auf Feedback-Events aus dem Server (B-139, B-140); keine Spiel-Logik im Client, kein `Math.random()` für Spiel-Logik (rein optische Streuung der Partikel ist erlaubt, sofern sie keinen Spielzustand beeinflusst).
- Effekte: Treffer-Blitz, Screenshake, Münz-Partikel (aufheben, geben), Todes-Effekt, Bau-Effekt.
- Screenshake und Blitz sind in der Optionen-Szene (B-146) abschaltbar; Einstellung gilt je Gerät.
- Controller-Vibration über die Gamepad API, wenn Edge auf der Xbox sie unterstützt (Prüfung auf der Gamepad-Testseite), sonst entfällt sie ohne Fehler.
- Je Split-Screen-Viertel wirken die Effekte nur in der Kamera des betroffenen Spielers.

## Nicht-Ziele

Feedback-Events selbst (B-139, B-140), Optionen-Szene (B-146), Sound (B-167), neue Spielregeln.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts (`src/scenes/noSim.test.ts`); 2 Spieler gleichzeitig; Taste B nicht belegen; Barrierefreiheit: kein schnelles Flackern (höchstens 3 Blitze pro Sekunde, Wert prüfbar im Test oder in der Konfiguration). Voraussetzung: F3, F4, B-146.

## Beispiele

Gegner wird getroffen → kurzer Blitz am Gegner; Spieler wirft Münze → Münz-Partikel; Screenshake in den Optionen aus → Kamera bleibt ruhig.

## Ausnahme- und Fehlerfälle

Gamepad ohne Vibration → keine Vibration, kein Fehler. Event unbekannt (alter Server) → ignoriert, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Test oder Beobachtung: Jedes der Feedback-Events Treffer, Kill, Münze aufheben, Münze geben, Bau fertig, Tod löst den zugehörigen Effekt aus (Liste aus B-139).
- **AC-02** Beobachtung am TV: Mit Screenshake und Blitz „aus“ in den Optionen treten beide Effekte nicht auf; mit „an“ sind sie sichtbar.
- **AC-03** Test: Die Effekt-Auslösung liest nur Events und Einstellungen und ändert keinen Spielzustand (`noSim.test.ts` bleibt grün).
- **AC-04** Mit 2 Spielern im Split-Screen wirkt ein Screenshake nur in der Kamera des betroffenen Spielers.
- **AC-05** Die Blitzfrequenz liegt bei höchstens 3 Blitzen pro Sekunde (Test auf der Konfiguration).
- **AC-06** Auf einem Controller ohne Vibration läuft das Spiel ohne Fehler weiter.

## Offene Fragen

keine

## Notizen

Lücke 12 (Barrierefreiheit) aus `docs/plan-weiterentwicklung.md` § 4. Quelle: Schiene G, GR5, hängt von F3.
