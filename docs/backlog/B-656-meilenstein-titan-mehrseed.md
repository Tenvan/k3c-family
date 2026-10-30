# B-656 · Meilenstein 1 coop2 standard: Titan über mehrere Seeds, holz und schiff grün

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-29, Chat, durch Ralf, Revision 1

## Ausgangslage

BS8 AC-02 (`holz` grün) und BS9 AC-02 (`schiff` grün) sind offen: `coop2` standard, Seed `k3c-dev`, 40 Tage bleibt in Stufe 4 (Tag 36) stecken, der Titan fällt seit BS12.6 nur in 2 von 4 Seeds (vorher Tag 26). Baselines M1 bis M4 liegen vor (M4 mit 8 Seeds, Abschluss-Lauf BS6.4); das Zeitlimit von 1 h reicht nicht für 101 Seeds.

## Ziel

Der Meilenstein-Lauf wird über 8 Seeds beurteilt, `M4` ist die Vergleichsbasis. `holz` und `schiff` werden dabei grün oder mit Begründung neu gefasst, danach `frozen` um beide Gruppen ergänzt.

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

- **AC-01** `sim_test start bots=coop2 world=standard days=40 seeds=8 baseline=M4` läuft für alle 8 Seeds durch und liefert einen Delta-Bericht je Werte-Gruppe.
- **AC-02** Gruppe `holz`: Ampel grün über alle 8 Seeds, oder eine Begründung im Ticket, warum sie nicht grün wird.
- **AC-03** Gruppe `schiff`: Ampel grün über alle 8 Seeds, oder eine Begründung im Ticket, warum sie nicht grün wird; die Insel 1 wird per Schiff verlassen (Bau-Tag je Seed im Bericht).
- **AC-04** Nach der Justierung ist `frozen` um `holz` und `schiff` ergänzt; die Baseline `M5` ist gesichert.

## Offene Fragen

Entschieden (🧑, Chat, 2026-10-29): Kriterien auf 8 Seeds umstellen, `M4` als Vergleichsbasis. Das Zeitlimit für 101 Seeds wird nicht angehoben. Keine offenen Fragen mehr.

## Notizen

**Lauf run-1** (2026-10-29, `sim_test start bots=coop2 world=standard days=40 seeds=8 baseline=M4`): Ergebnis **Fail**, 16m21s, Bericht `reports/simtest-run-1/simtest.md`.

- Endboss Insel 1 in 0 von 8 Seeds besiegt (Ziel: Tag ≤ 40). Stufe 4 erreicht, dann Stillstand bei Tag 36; Seed k3c-dev: Burg am Tag 41 gefallen.
- Burg hält Nacht 1 bis 5 in 0 % der Seeds (40-Tage-Lauf).
- Hub-Stufe 2 vor Tag 5 in 0 %, Hub-Stufe 3 vor Tag 10 in 0 %.
- 13 von 16 Zielen verletzt, 0 knapp, 3 im Korridor.
- Keine Werte geändert, keine Baseline gesichert.

**Ursache:**
- `M4` (7ce5b590, BS6.4) und `M2` zeigen dasselbe. Seit `M4` hat `1524f866` nur Daten und Grafik geändert.
- **Messfehler bestätigt (Lauf run-2, 5 Tage, 1 Seed, `players 2`):** Burg hält Nacht 1 bis 5 zu **100 %** (Ziel 99 bis 100 %, Pass). Die Burg steht also in den ersten Nächten. Der 0-%-Wert in run-1 kommt aus Fällen **nach** Nacht 5, also aus dem 40-Tage-Lauf.
- **Code-Ursache:** `tools/k3c-workbench/internal/balance/targets.go:33` misst `castleHeld` als `CastleFallTick == nil` über den ganzen Lauf, ohne das Fenster `days: 5`. Ein Burgfall an Tag 36 bis 41 zählt als „nicht gehalten“.
- **Offen:** Die Inselstatistik in run-1 (`castleFell` 57,5 % bei Tiefe 0) passt nicht sauber zu 0 % je Seed; das ist nicht geklärt.
- **Vorschlag (nicht umgesetzt):** Ticket für das Mess-Fenster anlegen, `castleHeld` auf Tag ≤ 5 begrenzen. Balancing-Werte bleiben unverändert, bis das Mess-Fenster korrekt ist.
