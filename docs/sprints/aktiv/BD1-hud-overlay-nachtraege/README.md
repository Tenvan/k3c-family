# BD1 · CLI · Kamera, Beitritts-Hinweis, Overlay-Taste und HUD-Nachträge

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-392, B-391, B-389, B-399, B-398, B-414, B-393, B-370, B-374
- **Start-Commit:** 7724ead6
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-14, Chat 🧑 (Revision 3)

## Ausgangslage

Nach den Abnahmen U5.4, S8.4 und U6.4 sowie dem HUD-Umbau (B-413) sind neun Beobachtungen der Bedienung offen: Kamera von Spieler 2 (B-392), Beitritts-Hinweis (B-391), Ö bei offenem Cheat-Dialog (B-389), dauerhafter Steuerhinweis (B-399), Gold im HUD (B-398), Welt-Meldungen im globalen Band (B-414), fehlendes Münzbild (B-393), Mond-Bild (B-370) und die Frage, ob das Preisschild eine Taste nennt (B-374, entschieden: nein). Alle liegen in `src/scenes/`.

## Ziel

Zwei Spieler am PC und an der Xbox sehen im Split-Screen ihre eigene Ansicht, die richtigen Hinweise zur richtigen Zeit und ein ruhiges HUD. Am Ende sichtbar: Kamera jedes lokalen Spielers folgt seiner Figur, Beitritts-Hinweis für Spieler 2, Steuerhinweis verschwindet, Welt-Meldungen im Band oben, Münz- und Mond-Bild vor den Hinweisen; von 🧑 am PC abgenommen.

## Beteiligte und Zielgruppen

Spielende am PC und an der Xbox (höchstens 2 Spieler); ein Agent baut in `src/scenes/`; 🧑 nimmt am Gerät ab.

## Anforderungen

B-392, B-391, B-389, B-399, B-398, B-414, B-393, B-370, B-374 › Anforderungen. Freigegeben sind B-392, B-391, B-389 und B-414; B-399, B-398, B-393, B-370 und B-374 sind ausgefüllt (Entscheidungen 2026-10-14 eingetragen), ihre Freigabe steht aus. B-374 enthält keine Code-Arbeit mehr, nur die festgehaltene Entscheidung.

## Nicht-Ziele

Touch, Handy und 3–4 Spieler (entfallen seit 2026-10-11); neue HUD-Elemente über die genannten Tickets hinaus; Server-Änderungen (Grad-Aktionen: B-410, W11).

## Regeln und Einschränkungen

Domäne CLI. Der Client rechnet nichts und zeichnet nur Server-Zustand (`src/scenes/noSim.test.ts`). B nicht belegen, View + Menu reserviert. Texte zentral in `src/core/texts.de.ts`. Datei ≤ 400 Zeilen, Funktion ≤ 60. Mindest-Schriftgrößen je Split-Viertel (B-136).

## Beispiele

Zwei Spieler gehen in entgegengesetzte Richtungen → beide Ansichten scrollen. Ö bei offenem Cheat-Dialog → nur der Dialog schließt. Welle beginnt → „Nacht“ erscheint für alle im Band oben, nicht in der Mitte.

## Ausnahme- und Fehlerfälle

Textur für Münz- oder Mond-Bild fehlt → nur Text, Warnung im Client-Log 🤒. Spieler tritt aus → Kamera und Hinweise der übrigen Spieler bleiben unberührt.

## Akzeptanzkriterien

- **AC-01** Im Split-Screen mit 2 lokalen Spielern folgt die Kamera jeder Ansicht ihrer Figur (B-392).
- **AC-02** Der Beitritts-Hinweis erscheint für jeden freien Platz; Ö schließt bei offenem Cheat-Dialog nur den Dialog (B-391, B-389).
- **AC-03** Der Steuerhinweis blendet 20 s nach dem Beitritt aus und ist über „Hinweise“ wieder einschaltbar, das Gold steht im HUD statt als Beutel über dem Reiter und wird beim Bezahlen kurz hervorgehoben, Welt-Meldungen erscheinen im globalen Band oben (B-399, B-398, B-414).
- **AC-04** Das Münzbild erscheint im Münz-Hinweis (Ursache von der Session mit frischem Profil geprüft), die Nacht-Warnung zeigt ein Mond-Bild aus den vorhandenen Packs, das Preisschild bleibt ohne Taste (Entscheidung festgehalten) (B-393, B-370, B-374).
- **AC-05** `task check` ist grün; alle Texte kommen aus `texts.de.ts`.

## Offene Fragen

keine. Entschieden 2026-10-14 (🧑, Chat): B-399 20 s und über „Hinweise“ einschaltbar; B-398 Goldzahl kurz hervorheben; B-393 Session prüft die Ursache selbst; B-370 Mond-Icon aus vorhandenen Packs (Stern kein Ersatz); B-374 keine Taste im Preisschild; B-414 Projekt BED, Warteschlange, Umbruch sonst „…“.

Risiko: S8.2 fand in den vorhandenen Packs keinen Mond. Findet BD1.4 keinen, blockiert die Session mit einer Frage an 🧑 (neues Pack nur mit Lizenz-Eintrag und Zustimmung).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD1.1 | `BD1.1-kamera-spieler-2.md` | Umsetzung | autonom | fertig |
| BD1.2 | `BD1.2-beitritt-hinweis-oe.md` | Umsetzung | autonom | fertig |
| BD1.3 | `BD1.3-hud-hinweis-gold-meldungen.md` | Umsetzung | autonom | fertig |
| BD1.4 | `BD1.4-muenzbild-mondbild-hinweis.md` | Umsetzung | autonom | offen |
| BD1.5 | `BD1.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
