# B-654 · Bot-Performance: bosses.json je Tick neu geparst, nodeFor/wanted mit Map-Zugriffen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-28
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Profil des coop2-Laufs (40 Tage, Seed k3c-dev) nach dem Engine-Commit e768e761: Bot-Code macht noch etwa 45 % der Laufzeit. `miniBossID` (`tools/k3c-workbench/internal/balance/bot_abstieg.go`, über `miniDefeated` aus `withDescent` je Tick) liest und parst `bosses.json` bei jedem Aufruf neu: etwa 3,4 s von 41 s (6–7 %) plus Allokationen/GC. Außerdem `wanted` (bot_wirtschaft.go, 3,3 s flat) und `nodeFor`/`payableSitesX` (~7 s, viele String-Map-Zugriffe und SiteGoldLeft je Platz je Tick). Die Dateien gehörten beim Messen einer anderen Session (BS12) und wurden nicht angefasst.

## Ziel

Der Bot-Anteil des Balancing-Laufs sinkt, ohne das Bot-Verhalten zu ändern.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `miniBossID` einmal pro Prozess (oder je Tiefe) cachen, Ergebnis unverändert (data.Files ist fest, kein UseData).
- `wanted`/`nodeFor`: Map-Zugriffe je Aufruf vermeiden (Ergebnisse je Tick merken).
- Lauf-Ausgabe (K3C_SEEDS=k3c-dev,1,2,3) und `Replay.EndHash` byte-gleich.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `go test -tags fullgame -run TestMeilensteinCoop2` mit `K3C_SEEDS=k3c-dev,1,2,3`: Ausgabe und EndHash wie vorher, Laufzeit mindestens 8 % kürzer.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

2026-10-29 (Claude): `miniBossID` liest `bosses.json` einmal je Prozess (`sync.OnceValue`); Meilenstein-Lauf byte-gleich (Stufe 4, Tag 36), Laufzeit 80 s. Offen bleibt der zweite Punkt (`wanted`/`nodeFor` mit Map-Zugriffen je Tick), kein Messwert dafür.
