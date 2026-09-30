# B-006 · Gamepad-Test auf der Xbox ist ausgewertet

- **Domäne:** PLAT
- **Typ:** Frage
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** X1
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Testseite `gamepad-test.html` ist fertig, der Test auf der Xbox steht aus (B-Taste, Vollbild, HTTPS, Sprite-Budget).

## Ziel

Gamepad-Test auf der Xbox ist ausgewertet. Nutzen: Steuerung und Sprite-Budget beruhen sonst auf Vermutungen.

## Beteiligte und Zielgruppen

🧑 testet an der Xbox; der Agent wertet den Bericht aus; Spieler profitieren von verlässlicher Steuerung.

## Anforderungen

- Test auf der Xbox mit zwei Controllern: B-Taste, Vollbild, HTTPS, Sprite-Budget.
- Das Ergebnis fließt in die Steuerungstabelle von `game-design.md`.

## Nicht-Ziele

Umbau der Eingabe (SP08).

## Regeln und Einschränkungen

Regel „Seiten & Navigation“ aus `CLAUDE.md` (`installPageChrome()`, `toggleFullscreen()`, `goHome()`); B nicht belegen, View + Menu reserviert. Den Test an der Xbox führt nur 🧑 durch (Session mit `Agent: Mensch`); Anleitung im README › Gamepad-Test auf der Xbox.

## Beispiele

Test mit 2 Controllern → Bericht in `reports/` mit Tastenbelegung und FPS je Sprite-Anzahl.

## Ausnahme- und Fehlerfälle

Bericht kommt nicht am Server an → Test wiederholen, Ursache (HTTPS, Netz) als Befund notieren.

## Akzeptanzkriterien

- **AC-01** Ein Bericht der Xbox liegt in `reports/`.
- **AC-02** Die Steuerungstabelle in `game-design.md` enthält kein „vermutlich“ mehr.

## Offene Fragen

keine

## Notizen

Anleitung im README › Gamepad-Test auf der Xbox.
