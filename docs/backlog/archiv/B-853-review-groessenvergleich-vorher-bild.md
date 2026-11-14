# B-853 · Das Review zeigt den Größenvergleich auch für das Vorher-Bild

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ58
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: alle offenen angehen)

## Ausgangslage

WZ43.3 (B-733/AC-03): Das Review schaltet zwischen Vorher/Nachher und dem Größenvergleich des Arbeitsstands um (Default aus #514). Für die Basis fehlen Größe und Frames mit `basis`.

## Ziel

Der Größenvergleich steht auch für das Vorher-Bild bereit.

## Beteiligte und Zielgruppen

🧑 Ralf entscheidet, ob die umschaltbare Lösung reicht.

## Anforderungen

- Größe und Frames der Basis im Review.
- Vergleich für Vorher und Nachher nebeneinander.

## Nicht-Ziele

Katalog.

## Regeln und Einschränkungen

- **Bereich:** `tools/k3c-workbench/internal/review/**`, `tools/k3c-workbench/frontend/src/assets/review/**`
- **Abhängig von:** WZ43
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Asset geändert → Vorher und Nachher je mit Monarch zum Größenvergleich.

## Ausnahme- und Fehlerfälle

Basis ohne Asset → nur Nachher.

## Akzeptanzkriterien

- **AC-01** Entscheidung 🧑; falls gewünscht: Test zeigt Größenvergleich für die Basis.

## Offene Fragen

keine. Entschieden (autonom Claude, 2026-11-14): die umschaltbare Lösung reicht; WZ43.4 ist verworfen.

## Notizen

Vorschlag aus WZ43.5 (2026-11-14).
