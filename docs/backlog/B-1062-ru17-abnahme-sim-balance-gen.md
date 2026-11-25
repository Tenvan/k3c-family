# B-1062 · RU17 Abnahme: Sim, Balance, Gen und Sichtprüfung Effekte und Ton

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** BAL11
- **Projekt:** BAL
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU17 portiert `fx` und `audio` nach Rust. Sim-, Balance-, Gen-Läufe und `check:race` sind in den Sessions gesperrt und laufen hier.

## Ziel

RU17 ist abgenommen: gesperrte Läufe gelaufen, Sichtprüfung „Nacht mit Gegnern, Effekte und Ton“ am PC erfolgt.

## Beteiligte und Zielgruppen

Claude führt `check:race` aus, Ralf (🧑) prüft im Spiel am PC.

## Anforderungen

`check:race`; Golden-Vergleiche nur als Tests (Ausnahme B-1014); Port-Tests grün.

Prüfpunkte für RU39 (Ruckler-Messung, ungemessen): Aura-Closure `to_vec`, Sheet-Klon im Draw, Bank lädt synchron beim ersten hörbaren Mix, Musik wird im Frame des Wechsels dekodiert (Review Ton: MITTEL), Stimmengrenze 32 ohne Priorität (Musik kann bei 2 Spielern im Kampf wegfallen).

Bekannt: Atlas-Ausschnitte B-1060 (RU30); `fx/pending.rs` hält `Aura`/`aura_of` bis RU18 (B-1053 räumt in RU30 auf); `aura_of` liefert bis dahin eine leere Liste.

## Nicht-Ziele

Neue Funktionen; Atlas-Ausschnitte (B-1060).

## Regeln und Einschränkungen

Sim-Sperre vom 2026-11-11 gilt, Golden nie neu schreiben, `native/` und `engine/` unverändert.

## Beispiele

nicht relevant: Abnahme-Ticket ohne neues Verhalten.

## Ausnahme- und Fehlerfälle

nicht relevant: Abnahme-Ticket ohne neues Verhalten.

## Akzeptanzkriterien

- **AC-01** `task check:race` ist grün oder als nicht lauffähig begründet (kein gcc).
- **AC-02** Ralf hat im Spiel geprüft: Ruckler beim ersten Ton, Musikwechsel, Hufschlag im Split-Screen (sobald verdrahtet), Musik läuft im 2-Spieler-Kampf weiter, Hörprobe der Atlas-Slots als Einzeldateien (Pegel, Schnitt), Nacht mit Gegnern mit Effekten und Ton.

## Offene Fragen

keine

## Notizen

Lane `zweit`: Der Second Worker fährt dieses Ticket (Sim, Balance und Gen; die Sichtprüfung von Effekten und Ton bleibt bei Ralf).
