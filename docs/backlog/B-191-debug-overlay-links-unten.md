# B-191 · Debug-Overlay und Aktionsliste verdecken das HUD nicht

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** U5
- **Projekt:** BED
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Debug-Overlay (Info-Zeilen, `src/scenes/debugOverlayView.ts`, oben links ab y = 96) und die Aktionsliste (`src/scenes/debugOverlayPanel.ts`, `position:fixed; right:12px; top:72px`) liegen halbtransparent über dem HUD. Befund von 🧑 in der Abnahme DBG2.3 am PC (2026-10-03).

## Ziel

Gold, Vorrat und HP im HUD bleiben bei offenem Overlay lesbar.

## Beteiligte und Zielgruppen

🧑 und Entwickler beim Testen; Domäne CLI.

## Anforderungen

- Info-Zeilen und Aktionsliste sind links unten verankert (Wunsch 🧑), nicht über dem HUD.
- Die Aktionsliste überdeckt die Lauf-Flächen des Touch-Overlays nicht (`src/input/touchInput.ts`).
- Gilt für 1 und 2 Spieler am Gerät (Split-Screen).

## Nicht-Ziele

Neue Aktionen, Bedienung (B-179), Schließen mit Ö (B-192).

## Regeln und Einschränkungen

`src/scenes` zeichnet nur; Pflicht-Info nach `docs/rules/bedienung.md` darf nicht verdeckt werden; oben ca. 70 px für den Home-Button frei.

## Beispiele

Overlay mit Ö öffnen → HUD oben ist vollständig sichtbar, Overlay und Schaltflächen stehen links unten.

## Ausnahme- und Fehlerfälle

Handy im Hochformat: Liste bricht um, bleibt unten und treffbar.

## Akzeptanzkriterien

- **AC-01** Bei offenem Overlay im Dev-Raum liegt kein Teil von Overlay oder Aktionsliste über dem HUD (Browser-Pane, 1 und 2 Spieler).
- **AC-02** 🧑 bestätigt am PC und am Handy, dass HUD und Lauf-Flächen frei bleiben.

## Offene Fragen

Entschieden 2026-10-06 (🧑, Chat): Bei Touch liegen Debug-Overlay und Aktionsliste rechts neben dem linken Lauf-Feld, unten mittig; das HUD oben bleibt frei. **Ersetzt** am 2026-10-09 (🧑, Chat, U5.1): Auch bei Touch links unten, weil unten mittig die Touch-Tasten liegen und darüber Beitritts-Hinweis und Reise-Text.

## Notizen

Aus DBG2.3 (Abnahme B-179).
