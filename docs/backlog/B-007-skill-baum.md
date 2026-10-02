# B-007 · Skill-Baum mit Tank und Zauberer ist spielbar

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Truhen und versteckte Skill-Punkte gibt es schon, einen Skill-Baum noch nicht.

## Ziel

Skill-Baum mit Tank und Zauberer ist spielbar. Nutzen: Der Monarch soll aktiv mitkämpfen, das unterscheidet das Spiel von K2C.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen gleichzeitig, lokal und online); Umsetzung durch Entwickler oder Agent in Go.

## Anforderungen

- Skill-Punkte verteilen mit Tier-Gating.
- Zwei aktive Skills je Linie: Tank (Taunt, Shield Bash), Zauberer (Fireball, Ice Wall).
- Jeder Spieler hat seinen eigenen Skill-Baum, auch mit 2+ Spielern gleichzeitig.

## Nicht-Ziele

Weitere Linien (alle 4 Linien: später).

## Regeln und Einschränkungen

Neue Mechaniken nur in Go (Entscheidung 001, Feature-Stopp in `src/world/`); deterministisch, nur der Seed-RNG, kein `Math.random()`; Werte in `data/`; Komplexitäts-Budget. Erst nach der Go-Portierung; Feature-Kette REG (Regelwerk II) → SIM → CLI.

## Beispiele

Spieler 1 wählt Taunt, Spieler 2 Fireball → beide Skills wirken gleichzeitig, jeder auf seiner Taste.

## Ausnahme- und Fehlerfälle

Zu wenig Skill-Punkte oder Tier nicht freigeschaltet → Skill nicht wählbar, Punkte bleiben erhalten.

## Akzeptanzkriterien

- **AC-01** Die Skills sind in der Go-Simulation umgesetzt, mit Tests.
- **AC-02** Im Client sind sie bedienbar.
- **AC-03** Sie stehen im Spielstand (B-022).

## Offene Fragen

Skill-Tasten (B-026) und Klassen-Presets (B-017) sind noch nicht entschieden (🧑).

## Notizen

Erst nach der Go-Portierung (nach SP11): REG Regelwerk II → SIM → CLI.

R1 (2026-10-02): Das Skill-Menü liegt **nicht auf View** (View + Menu ist reserviert), die Taste legt Regelwerk II fest (Vorschlag LB + RB); Taste X bleibt für Skills frei.
Regelwerk dazu: B-110 (Skillung, Klassen, Level von Monarchen und Bürgern); die SIM-Umsetzung folgt nach dem Beschluss.
