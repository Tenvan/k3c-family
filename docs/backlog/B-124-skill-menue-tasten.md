# B-124 · Der Client hat Schlag, Skill-Slots, Skill-Menü und die Tasten für Controller, Tastatur und Touch

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** S3
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1, durch 🧑; mit Sprint S3

## Ausgangslage

`PlayerInput` kennt `confirm`, `interact`, `build`, `skillMenu`, `pause`, `fullscreen` (`src/input/playerInput.ts`); Skill-Menü und Slots gibt es nicht; das Touch-Overlay kennt keine Skill-Tasten.

## Ziel

Schlag (X, E), Skill-Slots 1–4 (LB, RB, LT, D-Pad hoch; Q, R, T, Z), Skill-Menü (D-Pad runter, K) und Touch-Tasten sind bedienbar; das Skill-Menü verteilt Punkte und macht Respec. Nutzen: Skills sind spielbar.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; 🧑 testet am Gerät.

## Anforderungen

- Neue Aktionen in `src/input/` (Domäne PLAT; Grenzfall dieses Tickets: Änderung nur mit Freigabe der Spec): Schlag, Skill 1–4, Skill-Menü; Belegung laut `docs/rules/monarch.md` § 4; B und View + Menu bleiben unbelegt.
- Skill-Menü in der HUD-Szene: Punkte verteilen, Respec am Tag, Anzeige der Slots und Abklingzeiten.
- Touch-Overlay mit Schlag- und Skill-Tasten; Grabstein-Darstellung (Wiederbeleben, B-120).
- Kein Spiel-Logik-Code in `src/scenes`.

## Nicht-Ziele

Simulation (B-118, B-119), Protokoll (B-123), Aktionen-Overlay (B-125).

## Regeln und Einschränkungen

`CLAUDE.md` (Tasten, Seiten, B-Taste), Eingabe über `PlayerInput`, nie konkrete Tasten im Spielcode. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Spieler drückt LB → Skill 1 feuert, die Abklingzeit läuft im HUD.

## Ausnahme- und Fehlerfälle

Skill ohne Punkte → Taste ohne Wirkung, Hinweis im Overlay (B-125).

## Akzeptanzkriterien

- **AC-01** Reine Funktion für Slot-Belegung je Gerät ist getestet.
- **AC-02** Skill-Menü und Slots sind bedienbar mit Controller, Tastatur und Touch.
- **AC-03** 🧑 hat Slots, Menü und Tasten am Gerät abgenommen.

## Offene Fragen

Domänen-Grenze: `src/input/` gehört zu PLAT; die Freigabe der Spec erlaubt die Änderung.

## Notizen

Aus R3.2. Abhängig von B-123.
