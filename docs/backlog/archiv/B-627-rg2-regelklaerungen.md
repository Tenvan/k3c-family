# B-627 · Regelklärungen RG2.1 (Handwerker, Verlust-Korridor, Gold-Schwelle) entscheiden

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

Der Workshop RG2.1 (Sprint RG2) klärt Regelfragen zu Handwerker, Verlust-Korridor und Gold-Schwelle. Das sind Spielregel-Entscheidungen, deshalb liegen sie als kritische Frage in KRT statt autonom entschieden.

## Ziel

Ralf entscheidet die drei Punkte; RG2.2 gleicht danach die Doku an.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

- Handwerker, Verlust-Korridor, Gold-Schwelle (Details: `docs/sprints/geplant/RG2-regelwerk-klaerungen/README.md`).

## Nicht-Ziele

–

## Regeln und Einschränkungen

Regeln in `docs/rules/`.

## Beispiele

–

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** 🧑 hat die drei Punkte entschieden.

## Offene Fragen

keine. Entscheidung je Punkt (Claude im Auftrag von Ralf, 2026-10-31), Ralf kann ändern.

## Notizen

Angelegt 2026-10-25 beim autonomen Durchgang.

**Entscheidung 2026-10-31** (Claude im Auftrag von Ralf):

| Punkt | Entscheidung | Grund |
|---|---|---|
| Handwerker für Schmiede und Rüstkammer (B-295) | **Handwerker arbeiten nur in der Werkstatt** | Keine neuen `dx`-Werte und Balancing-Last; Elite-Upgrade und Rüstung bleiben über Gold und Material schnell genug. `docs/rules/buerger.md` § 1 sagt es ausdrücklich. |
| Verlust-Korridor (B-185) | **höchstens 25 % der Kämpfer** | Schon von Ralf entschieden (F1.4, `zielkorridore.md` § 4); `wirtschaft.md` § 3 wird angeglichen. |
| Gold-Schwelle (B-203) | **Kennzahl entfällt** | Der Beutel ist bei 100 gedeckelt, jede Schwelle ist in Tick 1 erreicht; die Aussage trifft „Gold am Morgen je Spieler“ (Korridor 20–70). `--gold-threshold` bleibt auf 0 (nicht messen). |

Umsetzung in RG2.2 (`docs/rules/`, nur Doku).
