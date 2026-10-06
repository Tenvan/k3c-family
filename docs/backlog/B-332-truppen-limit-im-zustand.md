# B-332 · Kämpfer-Zahl und Truppen-Limit stehen im Zustand

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** W10
- **Erstellt:** 2026-10-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-06, Chat, durch 🧑, Revision 1; B-332 Variante A, Domänen-Ausnahme SIM economy_view

## Ausgangslage

W6.1 soll den Limit-Text „Kämpfer/Limit“ (z. B. „Limit 12/20“) aus dem Server-Zustand bilden (W6/AC-01, B-126/AC-01). Der Zustand v5 nennt aber weder die Zahl der Kämpfer noch das Limit: `sim.EconomyOf` (`engine/sim/economy_view.go`) liefert nur `stockMax`, `hubLevel`, `hubUpgrade`, `danger`; `fighters(w)` und `troopLimit(w)` (`engine/sim/barracks.go`) sind unexportiert, `docs/protocol.md` › Wirtschaft und `src/model/types.ts` kennen kein Feld dafür. Der Client darf das Limit nicht selbst ausrechnen (Basis + Kaserne, welche Figuren Kämpfer sind; `src/scenes/noSim.test.ts`, W6.1 › Kontext). W6.1 ist deshalb blockiert.

## Ziel

🧑 entscheidet, wie W6 ohne Rechnung im Client zum Limit-Text kommt, damit W6.1 weiterlaufen kann.

## Beteiligte und Zielgruppen

🧑 entscheidet; Entwickler SIM/SRV (Variante A), CLI (W6.1).

## Anforderungen

- Variante A (Vorschlag): `sim.EconomyOf` bekommt `fighters` und `troopLimit` (je Stufe, aus `fighters(w)` und `troopLimit(w)`), der Server sendet sie oben in `s` wie die übrigen Wirtschaftsfelder (optional, fehlt bei älterem Server); Protokoll-Session in SRV mit Eintrag in `docs/protocol.md` › Wirtschaft und Feldern in `src/model/types.ts`. Danach setzt W6.1 fort.
- Variante B: W6.1 setzt ohne Limit-Text fort (Wartegrund, Lagerstand, Hub-Stufe, Berufe, Händler, Heilplatz); der Limit-Text wandert in eine spätere Session, die nach Variante A kommt (W6 README, AC-01, B-126 anpassen).

## Nicht-Ziele

Zeichnen des Limits im HUD (W6.3); Änderung der Limit-Regel (`docs/rules/buerger.md` § 3).

## Regeln und Einschränkungen

Der Client rechnet keine Regeln (`docs/decisions/001`, `src/scenes/noSim.test.ts`); Protokolländerung nur in einer Protokoll-Session (`docs/arbeitsweise.md` › Domänen); W6 bleibt in CLI, `engine/` gehört zu SIM/SRV.

## Beispiele

Variante A: 12 Kämpfer, eine Kaserne gebaut → Zustand `fighters: 12, troopLimit: 20` → HUD „Limit 12/20“.

## Ausnahme- und Fehlerfälle

Älterer Server ohne die Felder → kein Limit-Text, kein Fehler (wie W6 › Ausnahme- und Fehlerfälle).

## Akzeptanzkriterien

- **AC-01** Variante A oder B ist in diesem Ticket mit Datum durch 🧑 festgehalten, und W6 (README, W6.1) ist entsprechend angepasst.

## Offene Fragen

Variante A oder B (🧑). **Entschieden 2026-10-06 (🧑, Chat): Variante A → Sprint W10 (SRV, einschiebbar); Domänen-Ausnahme SIM nur für `engine/sim/economy_view*.go`. W6.1 wartet auf W10.**

## Notizen

Gefunden in W6.1 (Stand `8069fcbc`, Protokoll v5 aus W5). Alle übrigen Felder für W6.1 stehen im Zustand: `sites[].state`/`upgrade`, `hubLevel`, `hubUpgrade`, `danger`, `stock`, `stockMax`, `merchant`, `troops[].profession`; Heilplatz-Radius aus `data/buildings.json` › `healer.heal.radiusUnits`.
