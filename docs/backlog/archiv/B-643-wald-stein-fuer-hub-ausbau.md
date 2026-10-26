# B-643 · Im Wald gibt es keinen Stein für den Hub-Ausbau auf Stufe 2

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** BS3
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑 (Auftrag „B-643 entscheiden“), Entscheidung Claude

## Ausgangslage

Im Referenzlauf `run-1` (BS11.4) stieg `HubLevel` im Wald nie über 1; der Vorrat blieb bei 20 Stein, der Hub-Ausbau auf Stufe 2 (3 Schritte: 25 + 25 + 50 Stein plus 20 Tor = 120 Stein, `data/hub.json`) kam nicht zustande. Der Wald hat kaum Felsen (`rocks`-Chunks, 2–4 Felsen), der Händler verkauft Stein erst ab Hub-Stufe 2.

## Ziel

Die Ursache ist geklärt und die Absteige-Regel der Bots passt zum Regelwerk.

## Beteiligte und Zielgruppen

Claude (Balancing-Läufe BS3), 🧑 entscheidet über Werte.

## Anforderungen

- Ursache klären: Wo kommen die 20 Stein her, warum nicht 25?
- Entscheidung: Wert (`hub.json` Stufe 2 auf 20 Stein, Felsen im Wald oder Händler-Angebot) – ungeprüft, Vorschlag offen.

## Nicht-Ziele

Neue Mechanik; Werte der tieferen Stufen.

## Regeln und Einschränkungen

Werte stehen in `data/`; Balancing über `sim_test`/BS3.

## Beispiele

Bot `wirtschaft`, Seed `k3c-dev`, Tag 5: Stein 20, Hub-Ausbau offen, Bot wartet → nach der Änderung steht Hub-Stufe 2 bis Tag 4.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Werte-/Quellenfrage.

## Akzeptanzkriterien

- **AC-01** Lauf `sim_test` mit `wirtschaft`, Seed `k3c-dev`: `hubLevel2Tick` liegt vor Tag 5.

## Offene Fragen

Welche Quelle bekommt der Wald (Felsen, Händler, Wert senken)? 🧑 entscheidet; Default: Händler verkauft ab Stufe 1 auch Stein.

## Notizen

**Entscheidung (Claude, Auftrag 🧑 „erst B-643 entscheiden“, 2026-10-26): kein Daten-Fehler.** Das Regelwerk legt den Stein bewusst in die Höhle (`materialien-gebaeude.md` § 1: Höhle Adern, unendlich; `stufen.md`: Wald liefert Holz, Höhle Stein; das Material-Tor „ver­langt kein Material, das auf der Stufe des neuen Hubs noch nicht abbaubar ist“), der Vorrat gehört der ganzen Insel. Der Weg zu Hub-Stufe 2 führt also **über die Höhle**. Falsch war die Absteige-Regel der Bots aus B-642 (Hub-Stufe = Tiefe + 2 zuerst); sie verlangte den Ausbau vor dem Weg zum Stein, daher der Notausstieg ab Tag 4.
**Umsetzung:** Regel in `bot_abstieg.go` geändert: Mauern der Linie 1 stehen, heller Tag, ab Tag 2; kein Hub-Ausbau, kein Notausstieg (Test `TestBotsSteigenAb`: Höhle bis Tag 2). Keine Änderung an `data/`. Ob die Bots den Stein in der Höhle dann sammeln und der Ausbau gelingt, zeigt BS3.4 (Meilenstein-Lauf). Der Notausstieg aus B-642 › Notizen (Referenzlauf) ist damit ersetzt.
