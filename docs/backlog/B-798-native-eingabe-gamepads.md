# B-798 · Im nativen Spiel steuern 1–2 Gamepads und die Tastatur je einen Spieler und die Menüs

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG5
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Im Browser liefern `src/input/playerInput.ts`, `slotBindings.ts` und `keyboardLayouts.ts` die Aktionen je Spieler, die Belegung steht in `docs/rules/bedienung.md` und `src/scenes/keyTable.ts`. Der Prototyp `native/k3c-ebiten/players.go` liest zwei Pads über Ebitengine.

## Ziel

Paket `native/k3c-game/input` macht aus Gamepads und Tastatur Befehle je Spieler und Menü-Aktionen (Typen aus `view`).

## Beteiligte und Zielgruppen

Familie (zwei Gamepads am TV), Claude (Umsetzung).

## Anforderungen

- Belegung wie `docs/rules/bedienung.md` (Laufen, Galopp, Zahlen, Skills, Menü); B ist in Menüs zurück, im Spiel unbelegt; View + Menu gemeinsam zurück ins Menü.
- 1–2 Pads, je Pad ein Spieler; Pad kommt oder geht während des Spiels (Hot-Plug); Tastatur als Rückfall für Spieler 1 bzw. 2.
- Xbox-Belegung für jedes Pad (keine Controller-Familien).
- Zuordnung als reine Funktion mit Tests (Zustand der Tasten → Befehl).

## Nicht-Ziele

Touch, Browser-Eingabe, Tastenbelegung ändern; Menü-Logik (NG7).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md` (B zurück, View + Menu reserviert). Vertrag `view` aus NG1 unverändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/input/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Zweites Pad wird im Spiel eingesteckt → Spieler 2 kann beitreten (Hinweis aus NG6). Pad 1 fällt aus → Spieler 1 steht, Spiel läuft weiter.

## Ausnahme- und Fehlerfälle

Pad ohne Standard-Belegung → Meldung im Log, Pad ignoriert. Kein Pad → Tastatur.

## Akzeptanzkriterien

- **AC-01** Tests: Jede Zeile der Belegung aus `docs/rules/bedienung.md` ergibt den erwarteten Befehl bzw. die Menü-Aktion.
- **AC-02** Tests: Pad kommt, Pad geht, zwei Pads, kein Pad (Tastatur) ordnen Spieler richtig zu.
- **AC-03** B ist in Menüs zurück und im Spiel unbelegt; View + Menu gemeinsam liefert „zurück ins Menü“ (Test).

## Offene Fragen

keine

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG5).
