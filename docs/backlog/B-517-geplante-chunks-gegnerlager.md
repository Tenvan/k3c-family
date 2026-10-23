# B-517 · Geplante Chunks tragen Gegnerlager mit stationärer Wache

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ27
- **Projekt:** WZG
- **Erstellt:** 2026-10-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Teil „Gegnerlager“ aus B-468, beim Regel-Workshop WZ14.1 (2026-10-21, Ralf im Chat) als eigener Folge-Sprint nach WZ14 (Ruinen) ausgelagert. Gegner kommen heute nur aus Portalen als Wellen (`engine/sim/enemies_waves.go`, `docs/rules/gegner.md` § 3); `stepEnemies` (`engine/sim/enemies.go`) lässt jeden Gegner zur Burg laufen. Es gibt keinen stationären Wächter, keinen Aggro-Radius und keinen Heimatpunkt außer `HomeX` (Flucht ins Portal). Das Feld `enemies` des geplanten Chunks (`engine/level/planned.go`) ist reserviert und wird abgelehnt.

## Ziel

Ein geplanter Chunk kann ein **Gegnerlager** tragen: feste Gegner aus dem Biom stehen als Wache an fester Lage, greifen Spieler und Truppen in Reichweite an, kehren danach zum Lager zurück und sind besiegbar. Ein geräumtes Lager bleibt geräumt (kein Respawn) und steht im Spielstand. Die Wellen bleiben unverändert.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Beschlossen im Regel-Workshop WZ14.1 (2026-10-21, Ralf im Chat); Details legt die Spec des Folge-Sprints fest.

- **Verhalten:** stationäre Wache; Gegnerarten aus dem Biom der Stufe, Skalierung je Tiefe über `spawnScaled`; Angriff in einem Aggro-Radius, Rückkehr zum Lager; besiegbar, **kein Respawn**; keine Wirkung auf Aggressionspool, Nacht-Welle und Wellen-Balancing.
- **Format:** Feld `enemies` des geplanten Chunks lesen und prüfen (Art im Biom, Anzahl, Lage, Abstände), Ablehnung mit Insel, Tiefe, Platz und Grund.
- **Spielstand:** Zustand „geräumt“ je Lager, voraussichtlich `IslandSaveVersion` 9 (keine Rücksicht auf alte Spielstände).
- **Protokoll:** Version 7 bleibt, soweit additive Felder reichen (z. B. Kennung „Wache“ an `enemies[]`).
- **Grafik und Editor:** Lager-Grafik (CLI), Editor und Vorschau des Studios (DEV).
- Deterministisch über eigene Ströme, mit 2 Spielern; Golden ohne Planung byte-gleich.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Für die Spec des Folge-Sprints (entscheidet 🧑): Zahl der Wächter je Lager, Aggro-Radius, Belohnung beim Räumen (z. B. Münzen oder Truhe), Verhalten bei Nacht (bleibt Wache oder schließt sich der Welle an), Verhalten beim Burgfall (Lager wieder besetzt oder geräumt bleibt).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
