# B-323 · Die Welt spiegelt Lager-Maximum, Hub-Ausbau mit Kosten und Wartegrund „Gefahr“ für das Protokoll

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** W9
- **Erstellt:** 2026-10-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-06, Chat, durch 🧑, Revision 1

## Ausgangslage

W5.1 (B-153) soll Hub-Stufe mit Ausbaukosten, Lagerstand mit Maximum, Wartegrund je Bauplatz (Bauer, Material, Gefahr) und Händler in `snap`/`delta` bringen. `stateOf` (`engine/net/protocol.go`) sieht nur `*sim.World`, und W5.1 darf `engine/sim/` nicht ändern. In der Welt fehlen:

- **Lager-Maximum:** `capacity(w)` (`engine/sim/island_storage.go`) ist privat und rechnet über die private `w.island`; kein Feld in `World`.
- **Hub-Ausbau:** `World.HubLevel` ist exportiert, aber `json:"-"` (B-208). Zustand und bezahltes Gold des Ausbaus stehen im privaten `World.hubSite` (`hub_level.go`), die Kosten der nächsten Stufe in der privaten `hub.Levels`.
- **Wartegrund „Gefahr“:** Bau und Ausbau warten bei Gefahr (`isDangerous`, privat); `Site.State`/`Site.Upgrade` zeigen dann weiter `waitingWorker`, Gefahr ist nicht unterscheidbar.

Schon im Zustand: `merchant` (`resource`, `leaves`, `buyPaid`), `troops[].profession`/`workSite`, `sites[].state`/`upgrade`/`upgradePaid`/`level`, `stock`.

## Ziel

W5.1 kann die fehlenden Werte ohne eigene Regel-Rechnung aus der Welt übernehmen (`docs/arbeitsweise.md` › Domänen, W5.1 › Fallstrick Inselzustand).

## Beteiligte und Zielgruppen

Entwickler (SIM, SRV); 🧑 entscheidet den Weg.

## Anforderungen

- Lager-Maximum, Hub-Ausbau (Stufe, Kosten der nächsten Stufe, bezahltes Gold, Zustand) und Wartegrund „Gefahr“ sind aus `*sim.World` lesbar (exportiertes Feld mit `json`-Tag oder exportierte Funktion).
- Deterministisch, Golden-Daten nur bei gewollter Änderung (`docs/arbeitsweise.md` › Golden aktualisieren).

## Nicht-Ziele

Protokoll, Beispiele und Client (W5.1); Darstellung (W6).

## Regeln und Einschränkungen

Domäne SIM; `engine/sim` importiert nichts aus `engine/net`. Komplexitäts-Budget.

## Beispiele

Insel mit 3 Stufen und einem gebauten Lager → die Welt nennt Maximum 1200 je Rohstoff; Hub-Ausbau auf Stufe 2 bezahlt → die Welt nennt Stufe 1, Kosten der Stufe 2 und `waitingMaterial`.

## Ausnahme- und Fehlerfälle

Welt ohne Insel (Campaign): kein Maximum (Feld fehlt). Hub-Stufe 5: keine Ausbaukosten.

## Akzeptanzkriterien

- **AC-01** Ein Go-Test in `engine/sim/` belegt, dass `json.Marshal(world)` (oder eine exportierte Funktion) Lager-Maximum, Hub-Ausbau mit Kosten und den Wartegrund „Gefahr“ liefert.

## Offene Fragen

Welcher Weg (🧑)? **Entschieden 2026-10-06 (🧑, Chat): Weg 1, eigene SIM-Session vor W5.1 → Sprint W9.**

1. **Eigene SIM-Session vor W5.1** (Vorschlag): spiegelt die Werte in `World` (z. B. `stockMax`, `hubLevel`, `hubUpgrade`, Wartegrund „Gefahr“ am Bauplatz); W5.1 läuft danach unverändert. Golden-Diff möglich, weil neue Felder im JSON stehen.
2. **W5.1 macht die Spiegelung selbst in `engine/sim/`:** Spec-Revision von W5 (Erlaubte Dateien, Domänen-Ausnahme).
3. **`engine/net` rechnet aus `data/`:** nicht empfohlen, die Regel (300 je Hub plus 300 je Lager, Gefahr) stünde doppelt außerhalb der Simulation.

## Notizen

Gefunden in W5.1 (Schritt 1, Namensprüfung). `equipmentTaken` (Q69) emittiert die Sim noch nicht; für W5.1/AC-06 reicht die Beschreibung im Protokoll, kein Blocker.
