# B-285 · Der Server nennt je Spieler die lernbaren Skills

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RM1
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`actions` je Spieler (`docs/protocol.md` › Skills und Aktionen) kennt nur ein pauschales `{ action: 'learn' }`. Das Skill-Menü (S3.2, `src/scenes/skillMenuLogic.ts`) markiert deshalb jeden ungelernten Skill als „lernen“, auch Tier 2–4 ohne genug gelernte Skills der Linie (Tier-Gating, `docs/rules/monarch.md` § 3). Der Server lehnt den Versuch mit `bad_request` ab; der Spieler sieht nur, dass nichts passiert.

## Ziel

Das Skill-Menü zeigt nur Skills als lernbar, die der Server auch annimmt; der Client rechnet das Tier-Gating weiterhin nicht selbst.

## Beteiligte und Zielgruppen

Spieler am TV und Handy; 🧑 gibt die Spec frei.

## Anforderungen

- Der Zustand je Spieler nennt die gerade lernbaren Skill-IDs (z. B. `learn` mit Feld `skills` oder eigenes Feld), berechnet von `engine/sim`.
- Eigene Protokoll-Session (Server und Client, `docs/protocol.md`, `testdata/protocol/`); danach liest `menuEntries` das Feld statt des pauschalen `learn`.
- Budget je Tick beachten (nur senden, wenn sich etwas ändert).

## Nicht-Ziele

Neue Regeln für das Lernen (bleibt `docs/rules/monarch.md` § 3), Gestaltung des Menüs (S3.2).

## Regeln und Einschränkungen

Protokolländerung als eigene Session (`docs/arbeitsweise.md` › Domänen, Grenzfall Protokoll); Client rechnet nichts; deterministisch, 2+ Spieler.

## Beispiele

Spieler hat 2 Punkte und nichts gelernt → Menü zeigt „lernen“ bei Taunt, Shield Bash, Fireball, Ice Wall, Heal, Group Heal; Tier II–IV zeigen „–“.

## Ausnahme- und Fehlerfälle

Keine Punkte → keine lernbaren Skills; Feld fehlt (älterer Server) → wie heute pauschal über `learn`.

## Akzeptanzkriterien

- **AC-01** Go-Test: Ein Spieler mit 1 Punkt und keinem gelernten Tank-Skill bekommt `taunt` als lernbar, `ironWall` (Tier 3) nicht.
- **AC-02** Client-Test: `menuEntries` markiert nur die vom Server genannten Skills als lernbar.

## Offene Fragen

Form des Felds (in `actions` oder eigenes Feld): beim Planen, 🧑 gibt frei.

## Notizen

Gefunden in S3.2 (Nachweis im Browser-Pane, 2026-10-05).
