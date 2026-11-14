# B-901 · Das native Spiel zeigt die Kristallfarbe jeder Kristall-Ader

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** BI2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sim-Knoten tragen seit B-891 die Variante (`ResourceNode.Variant`); `native/k3c-game/props/nature.go` › `variantLook` wählt bisher nur die Baumart im Wald.

## Ziel

Kristall-Adern zeigen im nativen Spiel das Asset ihrer Farbe.

## Beteiligte und Zielgruppen

Spieler der Kristallhöhle; Claude setzt um; Ralf 🧑 prüft im Abnahme-Ticket.

## Anforderungen

- `variantLook` wählt für `crystalVein` je Variante das Bild: blau → `ader-kristall`, violett → `ader-kristall-violett`, gruen → `ader-kristall-gruen`.
- `levelAssets` nennt die beiden neuen Assets (fehlende Datei = Spielende mit Pfad, art.ErrMissing).

## Nicht-Ziele

Leuchten (B-906); Browser-Client.

## Regeln und Einschränkungen

Reine Auswahl in `props/nature.go`, getestet.

- **Bereich:** `native/k3c-game/props/nature.go`, `native/k3c-game/props/assets.go`, `native/k3c-game/props/*_test.go`
- **Abhängig von:** B-900
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Ader `crystalVein` mit Variante `violett` → Bild `objekte/ader-kristall-violett`.

## Ausnahme- und Fehlerfälle

Unbekannte oder leere Variante → Bild wie ohne Variante (`ader-kristall`).

## Akzeptanzkriterien

- **AC-01** Eine Kristall-Ader mit Variante blau, violett oder gruen zeigt `ader-kristall`, `ader-kristall-violett` bzw. `ader-kristall-gruen` (Unit-Test).
- **AC-02** Die Asset-Liste jedes Levels enthält die beiden neuen Assets; eine unbekannte Variante fällt auf `ader-kristall` zurück (Unit-Test).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
