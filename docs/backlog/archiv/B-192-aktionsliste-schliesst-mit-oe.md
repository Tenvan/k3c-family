# B-192 · Die Dev-Aktionsliste schließt sich mit Ö

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** U5
- **Projekt:** BED
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Befund von 🧑 in der Abnahme DBG2.3 am PC (2026-10-03): Ö schließt das Debug-Overlay, die Aktionsliste bleibt aber stehen. Vermutete Ursache (ungeprüft): `DevActionPanel.render` setzt `root.hidden`, das CSS `.k3c-dev{display:flex}` in `src/scenes/debugOverlayPanel.ts` überschreibt aber `[hidden]{display:none}` des Browsers.

## Ziel

Overlay und Aktionsliste öffnen und schließen gemeinsam.

## Beteiligte und Zielgruppen

🧑 und Entwickler beim Testen; Domäne CLI.

## Anforderungen

- Ö bzw. Klick auf den linken Stick blendet Info-Zeilen und Aktionsliste gemeinsam aus und ein.
- Bei geschlossenem Overlay fangen keine Schaltflächen Maus- oder Touch-Eingaben ab.

## Nicht-Ziele

Position des Overlays (B-191).

## Regeln und Einschränkungen

`src/scenes` zeichnet nur; B nicht belegen, View + Menu reserviert.

## Beispiele

Overlay mit Ö öffnen → Liste sichtbar; Ö erneut → Overlay und Liste weg.

## Ausnahme- und Fehlerfälle

Dev-Fokus (RB) aktiv beim Schließen → Fokus endet, Spieler steuert wieder normal.

## Akzeptanzkriterien

- **AC-01** Test: Nach `render(false, …)` ist die Aktionsliste nicht sichtbar (berechnete `display` ist `none`).
- **AC-02** Browser-Pane: Ö zweimal → Aktionsliste verschwindet, Klick auf ihre frühere Fläche löst keine Dev-Aktion aus.

## Offene Fragen

keine

## Notizen

Aus DBG2.3 (Abnahme B-179). Betrifft auch das Verbergen ohne Dev-Mode, falls die Liste vorher einmal sichtbar war.
