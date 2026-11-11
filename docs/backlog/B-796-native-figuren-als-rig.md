# B-796 · Das native Spiel zeichnet Monarchen, Reittiere, Bürger, Truppen, Gegner und Bosse aus dem Spielzustand

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG3
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Der Browser-Client zeichnet Monarch, Reittier und Figuren aus Sheets (`src/scenes/render/monarchAnim.ts`, `mountGait.ts`, `mountView.ts`, `citizenView.ts`, `worldSprites.ts`). Seit NV3 gibt es Rig-Dateien für alle Figuren mit Teile-Baum und das Zeichenpaket `native/k3c-rig/paint`; `requisite`-Figuren bleiben Sheets.

## Ziel

Paket `native/k3c-game/actors` zeichnet alle Figuren aus dem Spielzustand, Figuren mit Teile-Baum als Rig, `requisite` als Sheet.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung), 🧑 Ralf (Sichtprüfung NG9, Entscheidung zu B-780).

## Anforderungen

- Monarch mit Reittier: Animation aus dem Zustand (Stehen, Gehen, Galopp, Bauen, Kampf) wie `monarchAnim.ts` und `mountGait.ts`, Blickrichtung, 2 Monarchen.
- Bürger und Truppen je Beruf, Gegner und Bosse mit ihren Zuständen; `requisite` als Sheet.
- Rechnung (Animation, Phase, Richtung aus dem Zustand) als reine Funktionen mit Tests.
- Messung Rig gegen Sheet mit vielen Gegnern (B-780) im Spiel-Modul.

## Nicht-Ziele

Neue Figuren, Waffen, Rüstungen zeichnen; Spiel-Logik; Effekte über den Figuren (NG6).

## Regeln und Einschränkungen

Rig-Format `native/k3c-rig/RIG.md`, Entscheidung 005 und 008. Vertrag `view` aus NG1 unverändert.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/actors/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Zwei Monarchen reiten in entgegengesetzte Richtungen → jeder blickt in seine Richtung, Galopp-Phase je Reittier eigen. Nacht mit 50 Käfern → alle laufen als Rig.

## Ausnahme- und Fehlerfälle

Figur ohne Rig-Datei → Meldung beim Laden des Levels (keine stille Ersatz-Grafik). Unbekannter Zustand der Simulation → Ruhe-Pose, Meldung im Log einmal je Figurentyp.

## Akzeptanzkriterien

- **AC-01** Monarch und Reittier: Tests ordnen jedem Zustand aus der Simulation Animation und Richtung zu wie `monarchAnim.ts` und `mountGait.ts`.
- **AC-02** Bürger, Truppen, Gegner und Bosse: Jeder Figurentyp aus `data/` hat eine Zuordnung zu Rig oder Sheet (Test über alle Typen).
- **AC-03** Messbericht unter `reports/`: 2, 50 und 200 Gegner als Rig und als Sheet am Entwicklungs-PC (Bildrate, Rechenzeit je Bild).

## Offene Fragen

keine

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG3). Zusammen mit B-791 (Bein-Bänder) und B-780 (Gegnerzahl) im Sprint NG3.
