# BD1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Kamera, Beitritts-Hinweis, Overlay-Taste und HUD-Nachträge
- **Projekt:** BED

## Ziel

Zwei Spieler am PC und an der Xbox sehen im Split-Screen ihre eigene Ansicht, die richtigen Hinweise zur richtigen Zeit und ein ruhiges HUD. Am Ende sichtbar: Kamera jedes lokalen Spielers folgt seiner Figur, Beitritts-Hinweis für Spieler 2, Steuerhinweis verschwindet, Welt-Meldungen im Band oben, Münz- und Mond-Bild vor den Hinweisen; von 🧑 am PC abgenommen.

## Ergebnis

2026-10-15 (BD1.5): AC-01 und AC-02 umgesetzt und am PC von 🧑 bestätigt; AC-03 und AC-04 umgesetzt, angenommen, Validierung offen (BD1.3, BD1.4: HUD am PC, Münz-Hinweis mit frischem Profil, Mond-Optik am Gerät); AC-05 `task check` und `task check:go` grün. Behobener Befund: Steuerhinweis blieb mit 2 Spielern wegen Gerätewechsel stehen (B-399). Neue Tickets: keine (B-370 und B-427 gelöst: Mond als Emoji-Textur). Der Sprint-PR #299 ist gemergt; BD1.4 und dieser Review liegen lokal auf `develop`, noch nicht gepusht. Version: v0.17.0 vorgeschlagen (Minor: sichtbare Bedienungs- und HUD-Änderungen i …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Im Split-Screen mit 2 lokalen Spielern folgt die Kamera jeder Ansicht ihrer Figur (B-392). |
| AC-02 | – | Der Beitritts-Hinweis erscheint für jeden freien Platz; Ö schließt bei offenem Cheat-Dialo … |
| AC-03 | – | Der Steuerhinweis blendet 20 s nach dem Beitritt aus und ist über „Hinweise“ wieder einsch … |
| AC-04 | – | Das Münzbild erscheint im Münz-Hinweis (Ursache von der Session mit frischem Profil geprüf … |
| AC-05 | geprüft | `task check` ist grün; alle Texte kommen aus `texts.de.ts`. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-392 | erledigt | AC-01 – | Im Split-Screen scrollt nur die Kamera von Spieler 1 mit; Spieler 2 ist nicht an … |
| B-391 | erledigt | AC-02 – | Der Beitritts-Hinweis für Spieler 2 wird nicht angezeigt |
| B-389 | erledigt | AC-02 – | Ö schließt bei offenem Cheat-Dialog zuerst den Dialog, das Overlay bleibt offen |
| B-399 | erledigt | AC-03 – | Der Steuerhinweis im HUD verschwindet nach einiger Zeit |
| B-398 | erledigt | AC-03 – | Das Gold eines Spielers steht im HUD, nicht als Beutel über dem Reiter |
| B-414 | erledigt | AC-03 – | Welt-Meldungen erscheinen für alle im globalen Band oben |
| B-393 | erledigt | AC-04 – | Das Münzbild vor „Hinlaufen: Münze aufheben“ erscheint im Browser nicht |
| B-370 | erledigt | AC-04 – | Der Hinweis „Die Nacht naht“ zeigt ein Mond-Bild vor dem Text |
| B-374 | erledigt | AC-04 – | Das Preisschild nennt die Taste zum Bezahlen |
