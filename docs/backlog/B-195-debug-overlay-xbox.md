# B-195 · Das Debug-Overlay lässt sich auf der Xbox öffnen

- **Domäne:** PLAT
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** PL1
- **Projekt:** BED
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Debug-Overlay (U4, DBG2) öffnet sich per Ö oder Klick auf den linken Stick (LS, Taste 10, `src/scenes/debugOverlayView.ts`). Beim Test am 2026-10-03 auf der Xbox ließ es sich laut 🧑 nicht aufrufen, obwohl die Gamepad-Testseite LS am Controller #0 erkannt hat (X1.2).

## Ziel

Das Debug-Overlay ist mit dem Controller auf der Xbox erreichbar.

## Beteiligte und Zielgruppen

🧑 und die Familie am TV (Xbox, Edge), Entwickler im Client.

## Anforderungen

- Öffnen und Schließen mit dem Controller auf der Xbox, ohne B, View + Menu oder eine Spieltaste zu belegen.

## Nicht-Ziele

Leistungsziel des Servers (B-042, LT1).

## Regeln und Einschränkungen

`CLAUDE.md` › Seiten & Navigation und Regeln (B nicht belegen, View + Menu reserviert, 2 Spieler gleichzeitig). Komplexitäts-Budget.

## Beispiele

nicht relevant, die Beobachtung steht in der Ausgangslage.

## Ausnahme- und Fehlerfälle

nicht relevant, Ursache noch offen.

## Akzeptanzkriterien

- **AC-01** Ursache steht im Ticket (Code-Pfad und Beobachtung).
- **AC-02** 🧑 öffnet und schließt das Overlay auf der Xbox mit dem Controller.

## Offene Fragen

Auf welcher Seite wurde es versucht (`game.html`, `testing.html`)? Mit `?dev=0` in der Adresse? Klärt 🧑. Nicht blockierend (🧑, Chat, 2026-10-06): PL1.1 stellt das Problem zuerst auf `game.html` und `testing.html`, mit und ohne `?dev=0`, nach.

## Notizen

Gemeldet von 🧑 nach X1.2. Die Domäne hängt von der Ursache ab (Eingabe PLAT oder Overlay CLI).

**Befund PL1.1 (2026-10-09, Browser-Pane, Shell mit iframe, gemockter Controller):**
- Code-Pfad heute: `HudScene` erzeugt `DebugOverlay` nur, wenn `debugEnabled(location.search)` (`src/scenes/debugOverlay.ts`: alles außer `?dev=0`). Gesten in `src/scenes/debugOverlayView.ts` › `takeGestures` über `holdStep` (`debugGestures.ts`): RB 3 s = Diagnose, LB + RB 3 s = Cheat-Dialog, LB + RB kurz schließt; Ö/Ä an der Tastatur. Die Pads kommen aus `hud.input.gamepad` (Phaser `refreshPads` je Frame, also auch für Pads, die vor dem Start der HUD-Szene verbunden waren). Der LS-Weg (Taste 10) aus dem Ticket ist seit B-231 (Commit `5c03798d`, 2026-10-04) ersetzt.
- Vier Fälle: `game.html` → Overlay erzeugt, HUD sieht das Pad, RB 3 s zeigt die Diagnose, LB + RB 3 s öffnet den Dialog, LB + RB kurz schließt, Ö und Ä wirken. `game.html?dev=0` → kein Overlay (gewollt). Testszenario (`testing.html` → `game.html?autostart=1&fresh=1&save=…&mock=1`) → wie `game.html`. `testing.html?dev=0` → `scenarioUrl` reicht `dev` nicht weiter, also wie ohne `?dev=0`; nur `?dev=0` am Spiel selbst schaltet ab.
- Ursache im Browser nicht nachstellbar; weder `src/input/` noch der heutige Overlay-Code zeigen einen Fehler. Vermutet (ungeprüft): Am 2026-10-03 lief auf der Xbox noch der LS-Weg; oder Edge auf der Xbox nutzt LB/RB bzw. die Tasten-Emulation (keyCode 195–218, Bericht `gamepad-2026-10-03T16-19-33-906Z.json`; LS-Klick 209 fehlt dort) selbst. Prüfung an der Xbox in PL1.5 (Schritt 3).
