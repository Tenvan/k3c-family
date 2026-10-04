# B-118 · Der Monarch schlägt zu, Skill-Punkte kommen aus einem Fund-Pool und jeder Spieler verteilt sie für sich

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** S1
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S1

## Ausgangslage

Der Monarch hat keinen Angriff, kein Level und keine Skills; Skill-Punkte sind ein gemeinsamer Zähler (`World.SkillPoints`), `monarch.json` kennt `perLevel`, `maxLevel` und Presets ohne Code (`docs/rules/archiv/ist-monarch-buerger.md`).

## Ziel

Der Monarch hat einen Schlag (Taste X), keine Level; Skill-Punkte stammen aus einem Fund-Pool je Insel, jeder Spieler verteilt sie in seinem eigenen Baum, Respec ist nur am Tag an der Burg möglich. Nutzen: Grundlage für die Skills (`docs/rules/monarch.md` §§ 1–3).

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen); Werte pflegt REG, Feintuning B-099.

## Anforderungen

- `PlayerCommand` bekommt Schlag (Taste X) und Skill-Slots 1–4; Schlag: Schaden 10, Reichweite 1,5 Units, Abklingzeit 0,7 s (`data/monarch.json` › `attack`).
- `perLevel` und `maxLevel` entfallen (`data/monarch.json`).
- Fund-Pool je Insel: Quellen versteckt 3–5 je Stufe, Miniboss 1, Endboss 3, Meilensteine Hub-Stufe 2–5 je 1, jede 3. Truhe 1; jeder Spieler verteilt die Pool-Punkte für sich; späte Beitretende bekommen alle bisherigen Punkte.
- Tier-Gating 5/10/15 Punkte je Linie; Respec kostenlos an der Burg jedes Hubs, nur am Tag (nicht in Dämmerung und Nacht).
- Presets (Tank, Zauberer, Heiler, Dieb) als Startverteilung beim Beitritt (wählbar).
- Spielstand speichert Pool und Verteilung je Spieler (`SAVE_VERSION`).

## Nicht-Ziele

Skills selbst (B-119), Wiederbeleben (B-120), Protokoll (B-123), Client (B-124, B-125).

## Regeln und Einschränkungen

`docs/rules/monarch.md`; Werte nur in `data/`; Protokolländerung als eigene Session (B-123). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Drei Punkte im Pool, zwei Spieler: jeder verteilt drei Punkte in seinem Baum; Spieler 3 tritt später bei und hat sofort drei zu verteilen.

## Ausnahme- und Fehlerfälle

Respec in der Nacht → abgelehnt. Punkt über das Gating hinaus → abgelehnt.

## Akzeptanzkriterien

- **AC-01** Test: Schlag trifft Gegner in Reichweite mit Abklingzeit; ohne Gegner kein Effekt.
- **AC-02** Test: Pool zählt für jeden Spieler; Verteilung je Spieler getrennt; Spätbeitretende erhalten alle Punkte.
- **AC-03** Test: Tier-Gating und Respec (Tag ja, Nacht nein) wie beschrieben.
- **AC-04** Spielstand speichert und lädt Pool und Verteilung; ein alter Stand wird geladen.
- **AC-05** `task check:go` grün.

## Offene Fragen

Quelle „jede 3. Truhe“ je Insel zählt gemeinsam oder je Spieler (Annahme: gemeinsam).

## Notizen

Aus R3.2. Abhängig von B-100 (Insel/Stufen) und B-103 (Bosse für Punkte). Ersetzt den Pool-Teil von B-007.
