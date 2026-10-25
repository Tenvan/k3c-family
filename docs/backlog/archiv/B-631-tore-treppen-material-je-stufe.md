# B-631 · Tore, Treppen und Material-Tor nutzen nur Material der eigenen Stufe, Bau läuft durchgängig

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-25
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-25, Ralf im Chat

## Ausgangslage

Ralf (2026-10-25): Nach dem Merge von GEB17 bremst der Hub-Ausbau den Bau im Standard: Das Material-Tor von Hub-Stufe 2 verlangt Kupfer aus der Mine, und die Treppen kosten auf jeder Stufe 100 Stein. Der Mauer-Bot des Balancing-Testers kam ohne Kupfer nicht über Hub-Stufe 1 hinaus. Das Tor (Bauwerk) hat keine Ausbaustufen und kostet immer Holz.

## Ziel

Auf jeder Stufe lässt sich durchgängig bauen: Tore wachsen mit den Materialien der Stufen, Treppen und Material-Tor verlangen nur Material, das auf der Stufe des Hubs (bzw. bei der Treppe hoch auf der Stufe darüber) verfügbar ist.

## Beteiligte und Zielgruppen

Ralf (Vorgabe und Freigabe im Chat 2026-10-25), Agent SIM.

## Anforderungen

1. **Tor (Bauwerk):** Ausbaustufen wie die Mauer, ab Hub-Stufe 1 (`wallLines.gateHubLevel` 1). Stufe 1: 30 Holz + 10 Gold, HP 250, 6 s; 2: 45 Stein + 15 Gold, HP 500, 9 s; 3: 60 Kupfer + 30 Gold, HP 850, 12 s; 4: 75 Eisen + 60 Gold, HP 1350, 16 s; 5: 90 Kristall + 120 Gold, HP 2100, 20 s.
2. **Treppe runter** auf Stufe d kostet 100 Material der Stufe d + 50 Gold (Wald Holz, Höhle Stein, Mine Kupfer, Eisenstollen Eisen).
3. **Treppe hoch** auf Stufe d kostet 100 Material der Stufe d−1 + 50 Gold (Höhle Holz, Mine Stein, Eisenstollen Kupfer, Kristallhöhle Eisen).
4. **Material-Tor** des Ausbaus auf Hub-Stufe n verlangt 20 Material der Stufe n selbst (Hub 2: Stein, Hub 3: Kupfer, Hub 4: Eisen; Hub 5 ohne).
5. Regeln (`docs/rules/stufen.md` § 8, `docs/rules/materialien-gebaeude.md`), Glossar und `docs/data/*.md` angeglichen.

## Nicht-Ziele

Zwischenstufen für Verstärkungen der Tore (später eigenes Ticket); Balancing der Werte (Startwerte).

## Regeln und Einschränkungen

Werte in `data/`, Logik nur in `engine/sim`; Golden nur mit Begründung neu; zwei Spieler; deterministisch.

## Beispiele

Wald, Hub-Stufe 1: Tor der Linie 1 baubar mit Holz. Ausbau auf Hub-Stufe 2: Schritt 3 braucht 20 Stein zusätzlich, kein Kupfer. Höhle: Treppe hoch kostet 100 Holz + 50 Gold.

## Ausnahme- und Fehlerfälle

Stufe ohne tiefere Stufe: keine Treppe runter (wie heute `needsDeeper`). Fehlt ein Wert je Tiefe: Rückfall auf `cost`.

## Akzeptanzkriterien

- **AC-01** Tor hat 5 Ausbaustufen mit den Werten oben und ist ab Hub-Stufe 1 bezahlbar (Test in `engine/sim`).
- **AC-02** Treppe runter/hoch kosten je Stufe das Material laut Tabelle (Test).
- **AC-03** Material-Tor verlangt Material der eigenen Stufe; der Mauer-Bot kommt ohne Kupfer auf Hub-Stufe 2 (Test im Balancing-Tester).
- **AC-04** Regeln, Glossar und Datenbeschreibungen nennen die neuen Kosten; `task check`, `task check:go` grün.

## Offene Fragen

keine

## Notizen

Vorschlag und Freigabe im Chat 2026-10-25 (Tor-Werte, Treppen je Tiefe, Material-Tor eigene Stufe).

Umgesetzt 2026-10-25:
- AC-01 geprüft: `engine/sim/gate_level_test.go` (`TestTorStufenInDenDaten`, `TestTorAusbauAbHubStufe` mit zwei Spielern).
- AC-02 geprüft: `engine/sim/stairs_cost_test.go` (Tiefen 0–4, Treppe hoch in der Höhle kostet Holz, Rückfall), Client `economyLabels.test.ts` (`buildCost`).
- AC-03 geprüft: `TestHubMaterialTor` (20 Stein), `TestMauernZuerstBautMauerAus` ohne Kupfer grün, Validierung `checkGates`.
- AC-04 geprüft: Regeln, Glossar, `docs/data/*` angepasst, `task gen:rules`, datadoc grün; `task check:diff` grün, Review ohne schwere Befunde.
- Abweichung: Tor wird in seiner eigenen Stufe gezeichnet (statt der Stufe seiner Mauer, GEB4 AC-05); `gateRules.ts` entfernt.
- Folge-Ticket: B-633 (Text des Wartegrunds). Golden unverändert (Szenarien bauen keine Tore/Treppen).
