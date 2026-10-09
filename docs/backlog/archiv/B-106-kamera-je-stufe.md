# B-106 · Jeder Spieler sieht seine Stufe, auch wenn die Spieler in verschiedenen Stufen sind

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** S4
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S4

## Ausgangslage

Der Client zeichnet eine Welt; Split-Screen-Zellen zeigen dieselbe Welt (`src/scenes/GameScene.ts`, `worldRenderer.ts`, `layout.ts`).

## Ziel

Jede Zelle zeigt die Stufe ihres Spielers, auch wenn zwei Spieler am selben Gerät in verschiedenen Stufen sind; Radar und HUD folgen der Stufe der Zelle. Nutzen: Koop mit freier Stufenwahl ist spielbar.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 testet am Gerät.

## Anforderungen

- Zelle je Spieler mit eigener Stufe, Parallax und Palette je Stufe.
- Radar und HUD je Zelle zeigen die Stufe und den Hub dieser Zelle.
- Kein Spiel-Logik-Code in `src/scenes` (Client zeichnet nur).

## Nicht-Ziele

Simulation (B-100), Protokoll (B-104).

## Regeln und Einschränkungen

`CLAUDE.md` (Client rechnet nichts), Layout 1–4. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Zwei Spieler am selben Bildschirm: oben Wald, unten Höhle.

## Ausnahme- und Fehlerfälle

Stufe noch nicht geladen → Platzhalter statt Absturz.

## Akzeptanzkriterien

- **AC-01** Zeichnen je Zelle aus der Stufe des Spielers; Test der reinen Zuordnungs-Logik.
- **AC-02** Radar und HUD zeigen je Zelle die richtige Stufe.
- **AC-03** 🧑 hat 2 Spieler in verschiedenen Stufen am Gerät abgenommen.

## Offene Fragen

keine

## Notizen

Aus R1.3. Abhängig von B-100 und B-104.

Erledigt mit Sprint S4 (2026-10-03): AC-01 und AC-02 umgesetzt (S4.1, S4.2), AC-03 angenommen, Validierung offen (S4.3, am Gerät); der Betrieb mit zwei Stufen am selben Gerät wartet auf B-176 (Protokoll mit mehreren Stufen, S2).
