# B-656 · Meilenstein 1 coop2 standard: Titan über mehrere Seeds, holz und schiff grün

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BS8 AC-02 (`holz` grün) und BS9 AC-02 (`schiff` grün) sind offen: `coop2` standard, Seed `k3c-dev`, 40 Tage bleibt in Stufe 4 (Tag 36) stecken, der Titan fällt seit BS12.6 nur in 2 von 4 Seeds (vorher Tag 26). Baselines M1 bis M4 liegen vor (M4 mit 8 Seeds, Abschluss-Lauf BS6.4); das Zeitlimit von 1 h reicht nicht für 101 Seeds.

## Ziel

Der Meilenstein-Lauf wird über mehrere Seeds beurteilt (Vorschlag: 8 Seeds, `M4` als Vergleichsbasis), `holz` und `schiff` werden dabei grün oder mit Begründung neu gefasst, danach `frozen` um beide Gruppen ergänzt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- 🧑 entscheidet: Kriterien auf Mehr-Seed umstellen, Zeitlimit für 101 Seeds anheben oder beides.
- Ob die Bots die Hafen-Stufe betreten und das Segel bezahlen, per Lauf belegen (Kennzahl Bau-Tag des Schiffs fehlt im Bericht).
- Schiff verbraucht gemeinsames Material des Hub-Ausbaus: Ping-Pong prüfen.

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

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
