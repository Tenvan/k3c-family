# B-321 · Schlag ohne Treffer und Skill ohne Ziel erzeugen ein Ereignis

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SK1
- **Projekt:** SKL
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`stepAttack` (`engine/sim/monarch.go`) sendet `strike` nur, wenn ein Gegner in Reichweite ist; ohne Gegner gibt es
weder Ereignis noch Abklingzeit. `castSkill` (`engine/sim/skills.go`) bricht ohne Wirkung still ab, wenn der Effekt kein
Ziel findet. Der Client kann deshalb nicht zeigen, dass die Taste ankam (B-318, Beschluss 🧑 2026-10-06: Rückmeldung
als Server-Ereignis, der Client zeichnet nur).

## Ziel

Jeder Druck auf Schlag oder einen bereiten Skill-Slot erzeugt ein Ereignis, auch ohne Ziel. Voraussetzung für S9.2 und S9.3.

## Beteiligte und Zielgruppen

Alle Spieler (über B-318); Agent baut in `engine/sim/`.

## Anforderungen

- Schlag ohne Gegner in Reichweite: Ereignis `strike` mit `from`, `x` und `hit: false` (mit Treffer `hit: true`) und
  dieselbe Abklingzeit wie mit Treffer, damit eine gehaltene Taste nicht je Tick ein Ereignis erzeugt.
- Bereiter, belegter und gelernter Skill-Slot, dessen Effekt kein Ziel findet: Ereignis `castFailed` mit `from`, `slot`,
  `x`; keine Abklingzeit.
- Ereignis-Liste in `engine/sim/events.go` (Kommentar) nachgezogen.

## Nicht-Ziele

Protokoll und Client (S9.2, S9.3); Abklingzeit für fehlgeschlagene Skills.

## Regeln und Einschränkungen

SIM; deterministisch; Werte (Abklingzeit) aus `data/monarch.json`, keine Zahl im Code. Gegner-`strike` (`enemies.go`) bleibt unverändert.

## Beispiele

Schlag ins Leere → `strike {from, x, hit:false}`, nächster Schlag erst nach der Abklingzeit.
Feuerball ohne Gegner in Reichweite → `castFailed {from, slot:1, x}`.

## Ausnahme- und Fehlerfälle

Slot leer, Skill nicht gelernt oder Abklingzeit läuft → kein Ereignis (Taste wirkt nicht, wie heute).

## Akzeptanzkriterien

- **AC-01** Ein Sim-Test zeigt `strike` mit `hit: false` ohne Gegner und keine zweite Meldung während der Abklingzeit.
- **AC-02** Ein Sim-Test zeigt `castFailed` mit `from`, `slot`, `x` für einen bereiten Skill ohne Ziel und kein Ereignis für einen leeren oder abklingenden Slot.
- **AC-03** `task check:go` ist grün, Golden-Daten ändern sich nur um die neuen Ereignisse (Begründung im Ergebnis).

## Offene Fragen

keine

## Notizen

Vorgeschlagen bei der Planung von S9 (2026-10-06), Voraussetzung für B-318.
