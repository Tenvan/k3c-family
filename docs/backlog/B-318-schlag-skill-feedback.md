# B-318 · Schlag und Skills zeigen auch ohne Ziel sichtbar, dass die Taste ankam

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme S3.4 am 2026-10-06 (🧑, PC, Tastatur, am Tag ohne Gegner): E (Schlag) und ein belegter Skill-Slot (Q/R/T/Z) „tun nichts“. Ursache laut Code: Der Schlag wirkt nur mit lebendem Gegner in Reichweite und meldet sonst nichts (`engine/sim/monarch.go` › `stepAttack`); ein Skill wirkt nur, wenn sein Effekt ein Ziel findet (`engine/sim/skills.go` › `castSkill`). Der Client zeichnet für das Ereignis `strike` keine Bewegung des Monarchen (`src/scenes/effects.ts` kennt nur `hit`, `kill` …).

## Ziel

Jeder Druck auf Schlag oder einen bereiten Skill-Slot ist sichtbar, auch ohne Ziel. Nutzen: Spieler und Abnahmen erkennen sofort, ob die Taste ankam oder ob nur das Ziel fehlt.

## Beteiligte und Zielgruppen

Alle Spieler; 🧑 bei Abnahmen.

## Anforderungen

- Schlag zeigt am Monarchen eine kurze Bewegung, mit und ohne Treffer.
- Ein Skill ohne Ziel zeigt das kurz am Slot oder Monarchen (z. B. „kein Ziel“), statt still zu bleiben.
- 2 Spieler: Feedback je Monarch getrennt.

## Nicht-Ziele

Werte von Schlag und Skills (B-099); Treffer-Effekte (GR5).

## Regeln und Einschränkungen

`src/scenes` rechnet nichts (`noSim.test.ts`): Braucht der Client dafür ein Ereignis ohne Treffer, ist das eine SIM-Änderung mit eigener Session (Protokoll-Regel in `docs/arbeitsweise.md`).

## Beispiele

Tag, kein Gegner, E → Monarch holt kurz aus. Skill-Slot „bereit“, kein Ziel → kurzer Hinweis „kein Ziel“, Slot bleibt bereit.

## Ausnahme- und Fehlerfälle

Abklingzeit läuft → kein neues Feedback außer der Anzeige der Abklingzeit.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Schlag ohne Gegner erzeugt ein darstellbares Ereignis bzw. eine Darstellung, Schlag mit Gegner zusätzlich den Treffer.
- **AC-02** Ein Test belegt: Ein bereiter Skill ohne Ziel erzeugt den Hinweis „kein Ziel“ und keine Abklingzeit.
- **AC-03** 🧑 sieht am PC mit Tastatur bei E und Q/R/T/Z sofort eine Reaktion (Beobachtung).

## Offene Fragen

- Ereignis vom Server (`strike` auch ohne Treffer, `castFailed`) oder rein aus der eigenen Eingabe im Client: Entscheidung in der Planung, Vorschlag Server-Ereignis (Client rechnet nichts).

## Notizen

Anlass: S3.4, Ergebnis 2026-10-06. Nachts mit Gegnern noch nicht geprüft.
