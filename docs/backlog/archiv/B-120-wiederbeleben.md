# B-120 · Gefallene Monarchen bleiben liegen, Mitspieler beleben sie wieder, sonst Respawn nach 15 s

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** W4
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, mit Sprint W4

## Ausgangslage

Ein gefallener Monarch erscheint nach 5 s an der Burg mit voller HP; Wiederbeleben gibt es nicht (`engine/sim/economy.go`).

## Ziel

Ein gefallener Monarch bleibt als Grabstein liegen; ein Mitspieler belebt ihn durch A halten (3 s) mit 50 % HP, sonst Respawn an der Burg nach 15 s mit voller HP. Nutzen: Koop-Gefühl, der Tod hat Gewicht ohne Strafe (`docs/rules/monarch.md` § 5).

## Beteiligte und Zielgruppen

Spieler; Mitspieler in derselben Stufe.

## Anforderungen

- Zustand „gefallen“ mit Ort; `respawnSeconds` 15 (`data/monarch.json`).
- Wiederbeleben: Mitspieler hält A 3 s in Reichweite des Gefallenen (kein Zahlziel in der Nähe); Wiederbelebung am Ort mit 50 % HP.
- Reichweite 2 Units als eigener Wert in `data/monarch.json`; mehrere Helfer gleichzeitig beschleunigen nicht, keiner lässt dabei eine Münze fallen; ein getrennter Monarch (`Player.Free`) ist nicht wiederbelebbar (Beschluss Q33, 2026-10-04).
- Bezahlte, nicht fertige Münzen werden wie heute erstattet; gilt in jeder Stufe der Insel.
- Ereignisse `playerDown`, `revived`.

## Nicht-Ziele

Darstellung (B-124, B-125), Protokoll (B-123).

## Regeln und Einschränkungen

`docs/rules/monarch.md` § 5; Taste A nur für Interagieren. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Spieler 1 fällt im Wald; Spieler 2 hält A 3 s daneben → Spieler 1 steht mit 50 % HP wieder.

## Ausnahme- und Fehlerfälle

Beide Spieler fallen → Respawn nach 15 s. Der Helfer wird getroffen → Wiederbeleben unterbrochen.

## Akzeptanzkriterien

- **AC-01** Test: Wiederbeleben nach 3 s mit 50 % HP; Abbruch bei Treffer oder Loslassen.
- **AC-02** Test: ohne Hilfe Respawn nach 15 s an der Burg der Stufe.
- **AC-03** Test: Münzen werden beim Fallen erstattet; funktioniert in einer anderen Stufe als der Burg.
- **AC-04** `task check:go` grün.

## Offene Fragen

keine

## Notizen

Aus R3.2. Abhängig von B-100.
