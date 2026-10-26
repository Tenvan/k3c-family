# B-643 · Im Wald gibt es keinen Stein für den Hub-Ausbau auf Stufe 2

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Hub-Ausbau auf Stufe 2 braucht 25 Stein (`hub.json` › `levels`). Im Wald (Tiefe 0) gibt es nur Bäume (Seed `k3c-dev`: 159 Bäume, 0 Felsen); der Vorrat bleibt bei 20 Stein (Quelle ungeprüft, vermutlich Händler oder Drop). Gefunden in BS11.2: Bots `coop2` und `wirtschaft` bauen den Hub in 8 Tagen nie aus, `HubLevel` bleibt 1.

## Ziel

Der Hub-Ausbau auf Stufe 2 ist im Wald mit den vorhandenen Quellen (Felsen, Händler, Drops) erreichbar, bevor die Insel-Läufe der Balancing-Tester die Höhle erreichen sollen (Korridor „Höhle vor Tag 6“).

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

Bis dahin steigen die Bots der Insel-Läufe per Notausstieg ab Tag 4 ab (`bot_abstieg.go`, BS11.2).
