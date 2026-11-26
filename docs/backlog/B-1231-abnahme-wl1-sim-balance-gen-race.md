# B-1231 · Abnahme-Läufe WL1: Sim, Balance, Gen samt Golden, Race

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WK1
- **Projekt:** NAC
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint WL1 (Welten im Paket, Gamepad-Hotplug, Zäune und Mauern, Spielstart als Dienste) lief ohne Sim-, Balance-, Gen- und Race-Läufe (Beschluss 2026-11-11, B-742). Die Review-Session WL1.5 hat sie nicht ausgeführt.

## Ziel

Die ausgesetzten Läufe für WL1 sind nachgeholt und grün oder als Tickets festgehalten.

## Beteiligte und Zielgruppen

Claude (autonom, nachts); Ralf nur bei Roten.

## Anforderungen

- Sim und Balance über `sim_test` (sobald die Sperre aufgehoben ist), kein Wert außerhalb der Zielkorridore.
- `check_run task:test` samt `gen:paper` und Grafik-Golden für Zaun und Mauer aus WL1.4 grün.
- `check_run task:check:race` grün.

## Nicht-Ziele

Neue Funktionen; rote Läufe werden eigene Tickets.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Sim, Balance und Gen; Sim-Sperre bis auf Widerruf.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `docs/backlog/B-1231-abnahme-wl1-sim-balance-gen-race.md`
- **Abhängig von:** WL1
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Sim und Balance grün (`sim_test`).
- **AC-02** `task test` mit `gen:paper` und Golden grün.
- **AC-03** `task check:race` grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
