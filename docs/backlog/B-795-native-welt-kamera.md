# B-795 · Das native Spiel zeigt Welt, Kamera, Schichten, Fluss und Licht je Split-Screen-Zelle

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG2
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Der Browser-Client zeichnet Kamera, Hintergrund, Licht und Stufen in `src/scenes/render/` (`cameraFollow.ts`, `cameraCull.ts`, `stageView.ts`, `nightLight.ts`, `lightView.ts`, `paperBackdrop*.ts`, `viewRules.ts`) und teilt den Bildschirm in `src/scenes/layout.ts`. Der Prototyp `native/k3c-ebiten` hat die Aufteilung aus NV3: Welt oben, Fluss im unteren Drittel, `fern` fest, `mitte` und `nah` scrollen (`river.go`, `level.go`).

## Ziel

Paket `native/k3c-game/world` zeichnet je Zelle Hintergrund, Boden, Fluss und Licht und führt die Kamera wie im heutigen Spiel.

## Beteiligte und Zielgruppen

Familie (spielt zu zweit), Claude (Umsetzung), 🧑 Ralf (Sichtprüfung in NG9).

## Anforderungen

- Kamera je Zelle folgt ihrem Monarchen wie `cameraFollow.ts` (Totzone, Grenzen der Stufe); Culling zeichnet nur Sichtbares.
- Schichten: `fern` fest, `mitte` und `nah` scrollen mit den Faktoren aus den Daten bzw. NV3; Boden der Stufe.
- Fluss im unteren Drittel jeder Zelle mit sechs Wellen-Reihen wie NV3 (B-789).
- Tag und Nacht, Licht und Lava wie `nightLight.ts`, `lightView.ts`; Stufen- und Inselwechsel zeigen den Übergang wie `stageView.ts`.
- Rechnung (Kamera, Culling, Licht-Stärke) als reine Funktionen mit Tests.

## Nicht-Ziele

Shader für den Fluss (B-790, NG11); neue Hintergründe zeichnen; Figuren und Gebäude (NG3, NG4).

## Regeln und Einschränkungen

Vertrag `view` aus NG1 wird nicht geändert; fehlt etwas, Ticket. Entscheidung 005.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/world/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Monarch reitet nach rechts: Ufer, Boden, `mitte` und `nah` ziehen vorbei, `fern` steht, der Fluss fließt weiter; bei Nacht wird die Zelle dunkler, Fackeln leuchten.

## Ausnahme- und Fehlerfälle

Monarch am Rand der Stufe → Kamera bleibt an der Grenze. Zwei Spieler auf verschiedenen Stufen → jede Zelle zeigt ihre Stufe.

## Akzeptanzkriterien

- **AC-01** Kamera und Culling: Tests mit den Fällen aus `cameraFollow.ts` und `cameraCull.ts` (Totzone, Grenze, zwei Zellen) grün.
- **AC-02** `fern` steht, `mitte` und `nah` scrollen, der Fluss füllt das untere Drittel jeder Zelle (Test der Lage, Screenshot im Ergebnis).
- **AC-03** Licht bei Tag und Nacht und der Übergang beim Stufenwechsel folgen dem Zustand der Simulation (Test der Licht-Stärke je Tageszeit).

## Offene Fragen

keine

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG2).
