# B-166 · Audio-Autoplay und Formate auf Edge der Xbox sind geprüft

- **Domäne:** PLAT
- **Typ:** Frage
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** X1
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Gamepad-Testseite (`gamepad-test.html`, Script `src/tools/gamepadTest.ts`) kennt kein Audio (geprüft: kein Treffer für „audio“ in beiden Dateien). Sie schickt Berichte nach `reports/*.json`. Ob Edge auf der Xbox Audio ohne Geste startet, welche Formate (ogg, m4a/AAC, mp3, wav) dekodiert werden und wie hoch die Latenz ist, ist unbekannt; B-011 (Sound und Musik) und der Audio-Kern SO1 hängen davon ab.

## Ziel

Die Gamepad-Testseite prüft Autoplay, Formate und Latenz von Audio auf der Xbox und legt das Ergebnis im Bericht ab. Nutzen: SO1 wählt Format und Entsperr-Verhalten nach Messung statt nach Vermutung.

## Beteiligte und Zielgruppen

🧑 führt den Test an der Xbox aus (wie X1); der Agent baut die Prüfung ein und wertet den Bericht aus; Entwickler von SO1 lesen das Ergebnis.

## Anforderungen

- Neuer Abschnitt auf `gamepad-test.html` (Logik in `src/tools/gamepadTest.ts`): Zustand des `AudioContext` vor und nach der ersten Geste (Taste auf dem Controller), Ergebnis eines Abspielversuchs ohne Geste.
- Dekodier-Test je Format ogg (Vorbis), m4a (AAC), mp3 und wav mit je einer kurzen Testdatei unter `public/` (selbst erzeugt oder CC0, Credit-Eintrag, wenn nicht selbst erzeugt); Ergebnis je Format ja/nein.
- Latenz: `baseLatency` und `outputLatency` des `AudioContext`, soweit vorhanden.
- Das Ergebnis steht im Bericht in `reports/*.json` (neues Feld `audio`) und sichtbar auf der Seite.
- Die Seite hält die Regeln aus `CLAUDE.md` weiter ein (Home-Button, B nicht belegen).

## Nicht-Ziele

Audio-Kern und Mixer (SO1), Soundeffekte und Musik (B-167, B-168), Hörprobenseite (B-169).

## Regeln und Einschränkungen

`installPageChrome()` bleibt; B nicht belegen, View + Menu reserviert; den Test an der Xbox macht nur 🧑; Testdateien klein halten (Komplexitäts-Budget).

## Beispiele

Auf der Xbox die Seite öffnen, Taste A drücken → Anzeige: „AudioContext: suspended → running, ogg: nein, m4a: ja, mp3: ja, wav: ja, Latenz 0,03 s“; der Bericht in `reports/` enthält dieselben Werte.

## Ausnahme- und Fehlerfälle

`AudioContext` nicht verfügbar → Anzeige „kein Audio“, Bericht mit `audio: null`, kein Fehler. Ein Format lässt sich nicht dekodieren → „nein“ für dieses Format, Test läuft weiter.

## Akzeptanzkriterien

- **AC-01** Die Gamepad-Testseite zeigt den Zustand des `AudioContext` vor und nach der ersten Geste sowie das Ergebnis eines Abspielversuchs ohne Geste.
- **AC-02** Die Seite zeigt je Format ogg, m4a, mp3 und wav ja oder nein für die Dekodierung.
- **AC-03** Latenzwerte (`baseLatency`, `outputLatency`) werden angezeigt, soweit der Browser sie liefert.
- **AC-04** Test: Die Auswertungsfunktion schreibt ein Feld `audio` in den Bericht; ein fehlender `AudioContext` ergibt `audio: null` ohne Fehler.
- **AC-05** Ein Bericht der Xbox mit Audio-Ergebnis liegt in `reports/` (Test durch 🧑); Format- und Autoplay-Ergebnis stehen in `docs/game-design.md`.

## Offene Fragen

keine

## Notizen

Lücke 3 aus `docs/plan-weiterentwicklung.md` § 4. Voraussetzung für SO1. Passt zu X1 (Xbox-Test), kann dort mit eingeplant werden.
