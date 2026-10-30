# B-657 · Die Kennzahl castleHeld misst Nacht 1 bis 5 über den ganzen Lauf, nicht nur über das Fenster des Ziels

- **Domäne:** DEV
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

Die Kennzahl `castleHeld` in `tools/k3c-workbench/internal/balance/targets.go:33` ist `CastleFallTick == nil` über den ganzen Lauf. Das Fenster `days: 5` der Ziele in `data/balance-targets.json` wird dafür nicht ausgewertet. Belegt durch `run-1` (40 Tage, 8 Seeds: 0 %) und `run-2` (5 Tage, 1 Seed: 100 %, Ziel 99 bis 100 %). Der Fehler steht seit mindestens `M2` in den Baselines und hat B-656 scheinbar blockiert. Details in B-656 › Notizen.

## Ziel

Die Kennzahl `castleHeld` wertet nur Nacht 1 bis 5 aus, wie der Name des Ziels und das Fenster `days: 5` es sagen. Ein Burgfall nach Tag 5 zählt damit nicht mehr als „nicht gehalten“.

## Beteiligte und Zielgruppen

Claude setzt um. Ralf entscheidet, ob Baselines nach der Änderung neu gesichert werden.

## Anforderungen

- Fallende Burg in Nacht 1 bis 5 bleibt als Verletzung sichtbar.
- Ein Burgfall nach Tag 5 verletzt dieses Ziel nicht.
- Die Änderung ist nur Mess-Logik im Balancing-Bericht, keine Werteänderung.

## Nicht-Ziele

Keine Balancing-Werte ändern. Keine Baseline neu sichern. Andere Kennzahlen bleiben unverändert.

## Regeln und Einschränkungen

Logik in `tools/k3c-workbench/internal/balance` mit Test. Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen.

## Beispiele

Burg fällt Tag 41, Ziel Nacht 1 bis 5 → Ziel erfüllt (100 %). Burg fällt Tag 3 → Ziel verletzt (0 %).

## Ausnahme- und Fehlerfälle

Lauf kürzer als 5 Tage → Ziel nicht auswertbar, nicht als verletzt werten. Das muss im Test stehen.

## Akzeptanzkriterien

- **AC-01** Test: Burgfall nach Tag 5 ergibt `castleHeld` = 1 für Ziel mit `days: 5`.
- **AC-02** Test: Burgfall an Tag 3 ergibt `castleHeld` = 0.
- **AC-03** `sim_test start bots=coop2 world=standard days=40 seeds=1 players=2` zeigt für `castleHeld` Nacht 1 bis 5 den Wert der Nächte, nicht den des ganzen Laufs.

## Offene Fragen

Ob die Inselstatistik (`castleFell` 57,5 % bei Tiefe 0 in run-1) denselben Fehler hat, ist ungeprüft. Entscheidung: Ralf.

## Notizen

–
