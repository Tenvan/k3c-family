# B-317 · Der Cheat-Dialog zeigt den Fokus und lässt sich mit Pfeiltasten, Leertaste und Controller bedienen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** U5
- **Projekt:** BED
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beobachtung 🧑 am 2026-10-06 (Chat): Im Cheat-Dialog des Debug-Overlays (B-231, `src/scenes/debugOverlayPanel.ts`, `src/scenes/debugOverlayView.ts`) lassen sich die Buttons mit der Maus anklicken, das Fokus-Rechteck gibt aber kein Feedback. Pfeiltasten und Leertaste tun nichts. Mit dem Controller auf der Xbox geht die Bedienung ebenfalls nicht.

Befund im Code: Die Auswahl bewegt nur `padEdges()` (Controller-Knöpfe A, D-Pad); eine Tastatur-Bedienung außer Ä (schließen) gibt es nicht. Die Markierung ist die Klasse `.sel` (gelber Rand); ein Mausklick (`pointerdown` mit `preventDefault`) löst die Aktion aus, setzt aber weder `.sel` noch den Browser-Fokus.

## Ziel

Der Dialog ist mit Maus, Tastatur und Controller gleich bedienbar, und man sieht immer, welcher Button gewählt ist und dass er ausgelöst wurde. Nutzen: Cheats im Test schnell und sicher nutzen, auch am PC ohne Controller (B-314).

## Beteiligte und Zielgruppen

🧑 beim Testen am PC und an der Xbox; Agents beim Testen im Browser-Pane.

## Anforderungen

- Pfeiltasten wählen (hoch/runter Aktion, links/rechts Spieler), Leertaste oder Enter löst aus.
- Die Markierung folgt Maus, Tastatur und Controller; ein Klick setzt die Markierung auf den geklickten Button.
- Auslösen gibt sichtbares Feedback (z. B. kurzes Aufleuchten).
- Controller (D-Pad, A) bedient den Dialog auch in Edge auf der Xbox.
- Solange der Dialog offen ist, bewegen diese Tasten keinen Spieler (wie `muteFocused` für Controller).

## Nicht-Ziele

Neue Cheats; Gesten (B-231/AC-02).

## Regeln und Einschränkungen

B nicht belegen, View + Menu reserviert; `src/scenes` rechnet nichts (`noSim.test.ts`); Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Dialog mit Ä öffnen, Pfeil runter → zweiter Button gelb umrandet, Leertaste → Aktion, Button leuchtet kurz.

## Ausnahme- und Fehlerfälle

Server ohne Dev-Mode → Hinweis bleibt, Markierung und Feedback funktionieren trotzdem.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Pfeiltasten und Leertaste ändern die Auswahl bzw. lösen die gewählte Aktion aus, wie das D-Pad und A in `focusStep`.
- **AC-02** Ein Test belegt: Ein Klick setzt die Markierung auf den geklickten Button.
- **AC-03** 🧑 sieht am PC mit Maus und Tastatur Markierung und Feedback (Beobachtung).
- **AC-04** 🧑 bedient den Dialog mit Controller auf der Xbox (Beobachtung; zurückgestellt nach B-314, solange keine Controller-Tests).

## Offene Fragen

- Entschieden 2026-10-06 (🧑, Chat): Warum der Controller auf der Xbox nicht wirkt, klärt die Umsetzung über einen Bericht von `gamepad-test.html`; die Vermutung (Edge nutzt das D-Pad für Spatial Navigation zwischen HTML-Buttons oder die Gamepad-Abfrage der HUD-Szene ist leer) bleibt bis dahin ungeprüft.

## Notizen

Verwandt: B-231 (Cheat-Dialog), B-179 (Debug-Overlay-Aktionen), B-195 (Debug-Overlay auf der Xbox), B-314.
