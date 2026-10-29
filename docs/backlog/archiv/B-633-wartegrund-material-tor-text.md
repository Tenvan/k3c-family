# B-633 · Wartegrund des Material-Tors heißt nicht mehr „Material der nächsten Insel“

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit B-631 verlangt das Material-Tor das Material der eigenen neuen Hub-Stufe (Hub 2: Stein). Der Wartegrund `waitingNextIsland` (`engine/sim/economy_view.go` `hubWaitState`) und sein Client-Text „Material der nächsten Insel“ passen nur noch, wenn das Material tiefer liegt (z. B. Wald → Hub 2), nicht für den Hub in der Höhle.

## Ziel

Der Wartegrund nennt das fehlende Tor-Material sachlich richtig.

## Beteiligte und Zielgruppen

Spieler, Agent CLI/SIM.

## Anforderungen

Text (und ggf. Name des Wartegrunds) so fassen, dass er für jede Stufe stimmt, z. B. „Tor: 20 Stein fehlen“; Protokoll-Änderung nur mit Eintrag in `docs/protocol.md`.

## Nicht-Ziele

Regel des Material-Tors (B-631).

## Regeln und Einschränkungen

Zwei Spieler, Protokoll kompatibel oder Versionseintrag.

## Beispiele

Höhlen-Hub, Ausbau auf Hub 3, Kupfer fehlt nur für das Tor → Anzeige nennt Kupfer statt „nächste Insel“.

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** Der Wartegrund des Tor-Anteils nennt Material und Menge (Test im Client).

## Offene Fragen

keine

## Notizen

2026-10-29 (Claude): erledigt. Texte `wait.nextIsland` und `wait.nextIslandShort` (de/en) nennen jetzt „Tor-Material fehlt ({res})“ bzw. „Tor-Material“; Name des Wartegrunds `waitingNextIsland` bleibt (Protokoll unverändert).
