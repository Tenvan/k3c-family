# B-351 · Der Client schreibt FPS, Latenz und Puffer regelmäßig als Diagnose-Zeile ins Client-Log

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** PM1
- **Projekt:** LST
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`sim_test` mit `clients` 1–4 (B-348/AC-04, TR1.3) liest FPS und Latenz der Clients aus dem Log `k3c-client` (`/api/clientlog`). B-349 setzt voraus, dass der Client sie „wie bisher“ meldet; heute zeigt sie nur das Debug-Overlay (`src/scenes/debugOverlay.ts`), im Client-Log steht keine solche Zeile. Der Status eines Laufs zeigt deshalb „keine Diagnose-Zeilen im Client-Log“.

## Ziel

Jeder Testlauf mit Clients zeigt FPS, Latenz und Puffer der Clients ohne manuelles Ablesen.

## Beteiligte und Zielgruppen

Agenten und 🧑 bei Testläufen (`sim_test`), B-334 (Performance-Modus) als späterer Nutzer derselben Werte.

## Anforderungen

- Der Client schickt im laufenden Spiel alle 5 s eine Meldung (`info`) über `clientLog`, deren `ctx` die Felder `fps`, `latencyMs` und `bufferMs` (Zahlen) trägt; die Werte sind dieselben wie im Debug-Overlay.
- Die URL der Meldung trägt wie heute `room=<Code>`, damit `sim_test` sie dem Lauf zuordnet.

## Nicht-Ziele

Performance-Modus mit Bericht und Einbrüchen (B-334), neue Anzeige.

## Regeln und Einschränkungen

Domäne CLI. Lesender Teil steht in `tools/k3c-dev/internal/mcpsrv/simtest_clientlog.go` (Format: `ctx` als JSON mit `fps`, `latencyMs`, optional `bufferMs`). Die Drosselung von `clientLog` (gleiche Meldung binnen `REPEAT_MS` wird nur gezählt, der Schlüssel enthält `ctx` nicht) darf die Zeile nicht schlucken.

## Beispiele

`sim_test {action: start, mode: online, clients: 1, players: 2, duration: 1m}` → Status zeigt `Client-FPS min …, Mittel …; Latenz max … ms`.

## Ausnahme- und Fehlerfälle

Ohne Server (`installClientLog()` nicht aufgerufen) passiert nichts.

## Akzeptanzkriterien

- **AC-01** Test: Die Diagnose-Meldung entsteht im Takt mit `ctx` `{fps, latencyMs, bufferMs}`.
- **AC-02** `sim_test` mit einem Client zeigt FPS und Latenz im Status (Nachweis im Browser durch 🧑).

## Offene Fragen

Ob die Werte Teil von B-334 werden oder eigenständig bleiben, entscheidet 🧑 bei der Planung.

## Notizen

Gefunden in TR1.3 (2026-10-07).
