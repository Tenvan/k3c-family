# B-398 · Das Gold eines Spielers steht im HUD, nicht als Beutel über dem Reiter

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beobachtung 🧑 in der Gesamtabnahme U6.4, Prüfliste Punkt 5b (2026-10-11, PC, Tastatur): Ein Goldbeutel über dem Reiter erscheint nicht (er wird nur beim Bezahlen und kurz nach einer Änderung gezeigt, `src/scenes/worldRenderer.ts` › `updatePlayer`: Text „n Gold“ über dem Reiter, Schrift `purse`), und er soll auch gar nicht über dem Spieler stehen, sondern im HUD. Das HUD zeigt Gold heute in der Spielerzeile (`HudScene.showCells`, `hud.playerFull`: Gold/Maximum, HP, Stufe).

## Ziel

Das Gold jedes Spielers steht ausschließlich im HUD (Spielerzeile) und nicht als Text über dem Reiter. Nutzen: ein Ort für die Anzeige, kein Text in der Welt über dem Monarchen, Prüfliste Punkt 5 ist eindeutig.

## Beteiligte und Zielgruppen

Spieler am PC und an der Xbox (höchstens 2); 🧑 nimmt ab.

## Anforderungen

- Der Goldbeutel-Text über dem Reiter (`purse` in `worldRenderer.ts`) entfällt.
- Das HUD zeigt das Gold jedes Spielers weiter an (Spielerzeile); beim Bezahlen und nach einer Änderung hebt es die Zahl sichtbar hervor (Vorschlag), falls das ohne Beutel nötig ist.
- HP-Balken über dem Reiter bleiben (B-173).
- 2 Spieler: Gold je Zelle in der eigenen Spielerzeile.

## Nicht-Ziele

Neue HUD-Inhalte außer der Hervorhebung; Münz-Animationen und Effekte (GR5); Änderung der Goldwerte.

## Regeln und Einschränkungen

CLI (`src/scenes/`), nur zeichnen (ADR 001); HUD-Elemente laufen über die Layout-Funktion (B-337); Mindest-Schriftgröße (B-136); Texte über `t()`; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Spieler bezahlt 3 Gold an einem Bauplatz → über dem Reiter erscheint kein Text; in der Spielerzeile des HUD sinkt die Goldzahl, kurz hervorgehoben.

## Ausnahme- und Fehlerfälle

Gefallener Monarch → Spielerzeile zeigt wie bisher den Respawn-Text, kein Goldwert in der Welt.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Die Ansicht eines Spielers erzeugt keinen Goldtext über dem Reiter; die HUD-Spielerzeile nennt das Gold (`hud.playerFull`/`hud.playerShort`).
- **AC-02** 🧑 sieht am PC beim Bezahlen über dem Reiter keinen Goldtext und im HUD die Goldzahl (Beobachtung).

## Offene Fragen

Soll die Goldzahl im HUD beim Bezahlen hervorgehoben werden (Farbe, kurzes Aufblinken), oder reicht die Zahl? Entscheidet 🧑.

## Notizen

Herkunft: U6.4, Prüfliste Punkt 5b (2026-10-11). Verwandt: B-173 (Monarch auf dem Reittier, Goldbeutel und HP über dem Reiter), B-337.
