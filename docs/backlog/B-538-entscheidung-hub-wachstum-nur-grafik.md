# B-538 · Entscheidung D1: Hub-Wachstum bleibt reine Grafik, keine feinere Stufenfolge im Server

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KR2
- **Projekt:** GEB
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB11 lässt den Hub vom Lagerfeuer bis zur Burg wachsen, rein als Grafik aus `hubLevel` und `hubUpgrade` (`src/scenes/render/hubGrowth.ts`). Vorlage mit Optionen A/B/C: `docs/sprints/aktiv/GEB11-hub-waechst-kleinteilig/README.md` › Offene Fragen, GEB12 › D1.

## Ziel

🧑 wählt A, B oder C.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Protokoll und Spielstand ändern nur mit Entscheidung von 🧑; „Linie k ab Hub-Stufe k“ (`docs/rules/stufen.md`).

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A, B oder C gewählt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Soll der Hub zwischen den Hub-Stufen echte Ausbauschritte bekommen, die etwas freischalten?

- **A (Default, gilt):** so lassen, Wachstum nur als Grafik. Kein Risiko; die Zwischenlooks schalten nichts frei.
- **B:** Unterstufen als zusätzliche `levels` in `data/hub.json`; bricht „Linie k ab Hub-Stufe k“, Claude rät ab.
- **C:** Ausbauschritte je Stufe mit eigener Freischaltung (`levels[n].steps`, `sites[].hubStep`), Protokollfeld `hubStep`, Spielstand. SIM, SRV, CLI, Studio-Planer, Golden, Balancing.
- **Wenn 🧑 C wählt:** eigener Sprint im Projekt GEB (SIM/REG „Hub-Ausbauschritte“), danach Client-Anpassung von `hubLookOf`. Bei A nichts weiter.
