# B-122 · Elite-Upgrades, Rüstung, Truppen-Limit je Hub und Heilung der Truppen sind umgesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W4
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Krieger und Elite-Truppen gibt es nur als Daten; es gibt kein Truppen-Limit, keine Heilung außer Respawn und kein Verlust-Ereignis (`docs/rules/ist-monarch-buerger.md` § 3).

## Ziel

Schmiede liefert Elite-Upgrades, Rüstkammer Rüstung für alle Truppen, das Truppen-Limit (Basis 10, Kaserne +10, nur Kämpfer) gilt je Hub, der Heilplatz heilt Truppen und Spieler, Verluste werden als Ereignis gemeldet. Nutzen: Truppenaufbau mit Grenzen und Tiefe (`docs/rules/buerger.md` §§ 2–3).

## Beteiligte und Zielgruppen

Spieler; Messung mit B-099.

## Anforderungen

- Krieger mit Schwert (Werkstatt) und Nahkampf-Verhalten (B-014); Elite-Bogenschütze (100 Stein + 50 Gold) und Elite-Krieger (100 Kupfer + 50 Gold) in der Schmiede.
- Rüstungs-Upgrade in der Rüstkammer: 100 Eisen + 100 Gold je Stufe, +20 % HP für alle Truppen.
- Truppen-Limit je Hub: Basis 10, Kaserne +10, nur Kämpfer zählen.
- Heilung am Heilplatz (Truppen und Spieler in Reichweite) und durch den Heiler-Skill (B-119); keine Regeneration.
- Ereignis `troopLost` für Verluste (Messgröße für B-099).

## Nicht-Ziele

Berufe und Händler (B-121), Gegner-Werte (Regelwerk III), Darstellung (B-126).

## Regeln und Einschränkungen

`docs/rules/buerger.md`; Werte nur in `data/`; Verhalten wie heute (Bauern fliehen, Kämpfer halten Posten). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Ein Hub mit 10 Kämpfern und Kaserne: die 11. Waffe kann bis zum Limit 20 weiter ausgegeben werden; ab 20 wird der Bauer nicht mehr zum Kämpfer.

## Ausnahme- und Fehlerfälle

Limit erreicht → Hinweis (B-126), keine Verschwendung von Material (Bogen bleibt im Regal).

## Akzeptanzkriterien

- **AC-01** Test: Elite-Upgrade und Rüstung wirken mit Kosten und Werten aus den Daten.
- **AC-02** Test: Limit je Hub (Basis, Kaserne), nur Kämpfer zählen.
- **AC-03** Test: Heilplatz heilt in Reichweite, ohne Heilplatz keine Heilung.
- **AC-04** Test: `troopLost` bei Verlust; Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

keine

## Notizen

Aus R3.3. Verwandt mit B-014 (Krieger, Elite) und B-116 (Gebäude-Wirkungen).
