# B-007 · Skill-Baum mit Tank und Zauberer ist spielbar

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** SKL
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Chat (Ralf), Revision 1 (mit SK1)

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

Entschieden 2026-10-06 (🧑, Chat): Skill-Tasten (B-026) und Presets (B-017) sind erledigt (archiviert); Grundlage `docs/rules/monarch.md`, S1 und S3. SK1 plant nur, was nach S1 fehlt.

## Notizen

**Erledigt (2026-10-24):** Der Skill-Baum mit Tank, Zauberer, Heiler und Dieb ist spielbar (SK1, RM1, SK2, SK3); Skills wirken sichtbar mit eigenem Effekt (B-578), Ralf hat die Effekte am PC abgenommen (SK2.5). Balancing der Skills läuft über WZ28/BAL.
