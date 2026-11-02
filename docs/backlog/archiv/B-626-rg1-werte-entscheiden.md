# B-626 · Wertvorschläge aus RG1.1 und BR2.1 entscheiden

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** KRT
- **Erstellt:** 2026-10-25
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Antwort an Claude delegiert)

## Ausgangslage

Die Messungen RG1.1 und BR2.1 (run-7, 2026-10-25, `docs/balancing/messung-2026-10-25.md`) ergeben Wertvorschläge. Spielwerte ändert Claude nicht ohne Ralf, deshalb liegt das als kritische Frage in KRT.

## Ziel

Ralf entscheidet die Vorschläge (Workshop RG1.2, BR2.2).

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

Vorschläge:
- Hub-Stufe 5, Schritt 3: 200 → 100 Gold (nach GEB17).
- Adern `percentPerGather` 1 → 2 (Wellenabstand ca. 10,6 statt 19,4 min, rechnerisch).
- `goblin.damage` 12 → 10, alternativ Wellen 1–5 mit [4, 9] statt [5, 10] Gegnern (zerstörte Gebäude 2 statt höchstens 1).
- Kupfer-Tor auf Hub-Stufe 2 (GEB17): für den Bot in Tiefe 0 nicht erreichbar, gewollt? (siehe B-623).

## Nicht-Ziele

–

## Regeln und Einschränkungen

Werte in `data/`, Golden mit Begründung.

## Beispiele

–

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** 🧑 hat jeden Vorschlag angenommen oder verworfen; RG1.2 und BR2.3 setzen die Beschlüsse um.

## Offene Fragen

keine. Entscheidung je Vorschlag (Claude im Auftrag von Ralf, 2026-10-31); die Wirkung ist nicht gemessen, jede Anpassung prüft eine Delta-Runde (`B-640`) gegen die Baseline.

## Notizen

Herkunft: RG1.1, BR2.1 (2026-10-25). Messung: `docs/balancing/messung-2026-10-25.md`.

**Entscheidung 2026-10-31** (Claude im Auftrag von Ralf, Ralf kann ändern):

| Vorschlag | Entscheidung | Grund |
|---|---|---|
| Hub-Stufe 5, Schritt 3: 200 → 100 Gold | **verworfen, vertagt** | Hub-Zeit ist laut Messung kein Gold-Problem, kein Lauf erreicht Stufe 2 (Stein, Kupfer-Tor). Erst messen mit ganzer Insel (BAL9), dann Kosten. |
| Adern `percentPerGather` 1 → 2 | **angenommen** | Wellenabstand rechnerisch 19,4 → 10,6 min, Korridor 4–12 min; kleine, gezielte Änderung. Wirkung per Delta-Runde prüfen (KB6/B-579). |
| `goblin.damage` 12 → 10 | **angenommen** | Kleinste Stellschraube gegen 2 statt höchstens 1 zerstörtes Gebäude je Welle; die Wellengröße bleibt unberührt. Bei Fail weiter mit Wellen [4, 9]. |
| Kupfer-Tor auf Hub-Stufe 2 für den Bot in Tiefe 0 unerreichbar | **gewollt** | Das Tor soll den Abstieg erzwingen (B-623). Der Bot braucht den Abstieg, nicht das Tor geändert: BS11 (ganze Insel) ist dafür gebaut. |

Umsetzung der angenommenen Werte: RG1.2 und BR2.3 (`data/`, Golden mit Begründung).
