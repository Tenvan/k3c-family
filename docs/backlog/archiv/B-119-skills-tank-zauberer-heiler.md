# B-119 · Die Skills von Tank, Zauberer und Heiler wirken in der Simulation

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** S1
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S1

## Ausgangslage

Es gibt keine Skills (`docs/rules/archiv/ist-monarch-buerger.md` § 2); das GDD beschreibt 4 Linien mit Tiers und Zahlen.

## Ziel

Die aktiven Skills und Passiven der Linien Tank, Zauberer und Heiler (Tier 1–4) wirken; der Dieb bleibt im Regelwerk und folgt später. Nutzen: Der Monarch kämpft aktiv mit und ergänzt die Truppen.

## Beteiligte und Zielgruppen

Spieler; Zahlen aus `game-design.md` › Skill-Tabelle.

## Anforderungen

- Skill-Daten in `data/monarch.json` › `skills` (Tier, Abklingzeit, Wirkung): Taunt, Shield Bash, Iron Wall, Last Stand; Fireball, Ice Wall, Lightning Storm, Meteor; Heal, Group Heal, Divine Shield, Resurrection (Truppen).
- Wirkungen auf Gegner, Truppen und Spieler; Passive (Armor Aura, Thick Skin, Regeneration, Guardian, Fortified, Arcane Power, Spell Echo, Frost Armor, Elemental Mastery, Healing Aura, Blessings, Holy Ground) in den Daten mit Startwerten.
- Skill-Einsatz als Befehl in `PlayerCommand` (Slot 1–4), Abklingzeiten je Spieler.

## Nicht-Ziele

Dieb-Linie, Skill-Menü und Tasten (B-124), Protokoll (B-123), Grafiken/Effekte (B-010).

## Regeln und Einschränkungen

`docs/rules/monarch.md`; Zahlen als Startwerte, Feintuning B-099; deterministisch; Gating aus B-118. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Zauberer mit 5 Punkten: Fireball (30 Schaden, Fläche 3 Units, Abklingzeit 8 s) trifft Gegner in der Fläche.

## Ausnahme- und Fehlerfälle

Skill in Abklingzeit → ohne Wirkung. Ziel außer Reichweite → verfällt ohne Abklingzeit (Annahme).

## Akzeptanzkriterien

- **AC-01** Test je Skill: Wirkung, Reichweite/Fläche und Abklingzeit laut Daten.
- **AC-02** Test: Tier-Gating verhindert Skills ohne genug Punkte in der Linie.
- **AC-03** Test: Passive wirken (z. B. Armor Aura, Healing Aura) in 2-Spieler-Szenen.
- **AC-04** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Passive: Startwerte und Reichweiten sind noch nicht festgelegt (Vorschlag mit B-099).

## Notizen

Aus R3.2. Löst den Skill-Teil von B-007 ab. Abhängig von B-118.
