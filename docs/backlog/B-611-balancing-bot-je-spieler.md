# B-611 · Ein Balancing-Lauf kann für jeden Spieler einen eigenen Bot wählen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-24) bei der Abnahme der Balancing-Hilfen (WZ28.10): Er möchte je Spieler einen Bot wählen. Heute gilt ein Bot für alle Spieler eines Laufs; nur `coop2`/`coop4` teilen fest nach Spieler-Index in Mauern und Wirtschaft auf (`tools/k3c-workbench/internal/balance/bots.go`).

## Ziel

Ein Offline-Balancing-Lauf (Seite und CLI) kann jedem Spieler ein eigenes Bot-Profil geben, z. B. Spieler 1 `saver`, Spieler 2 `passive`.

## Beteiligte und Zielgruppen

Ralf beim Balancing; Agent DEV.

## Anforderungen

- Mischung als Bot-Name `profil+profil+…` (Spieler 1, 2, …); das Komma bleibt Trenner der Matrix `--bots`.
- Anzahl der Profile = Spielerzahl; `coop2`/`coop4` nicht in einer Mischung (sie verteilen selbst).
- Gültig überall, wo der Offline-Lauf einen Bot nimmt: `--bot` (Ziele), `--bots` (Matrix), `simrun.Spec.bot`, Ziele in `data/balance-targets.json`.
- Seite: Bot-Auswahl „je Spieler“ zeigt eine Auswahlbox je Spieler; ändert sich die Spielerzahl, passt sich die Mischung an.

## Nicht-Ziele

Online-Testläufe (`sim_test` mit Clients/Räumen); neue Bot-Profile.

## Regeln und Einschränkungen

Deterministisch wie bisher; Atomic Design über `@webkit`.

## Beispiele

`--targets --bot saver+passive --players 2` → Spieler 1 spart und baut, Spieler 2 steht am Hub.

## Ausnahme- und Fehlerfälle

Anzahl passt nicht zur Spielerzahl, unbekanntes Profil oder `coop*` in der Mischung → Fehler mit Grund.

## Akzeptanzkriterien

- **AC-01** Go-Test: Mischung steuert je Spieler das richtige Profil; ungültige Mischungen werden abgelehnt (Lauf, Ziel, Spec).
- **AC-02** Vitest: Seite wählt je Spieler einen Bot und erzeugt `bot: "a+b"`.

## Offene Fragen

keine. Default (Claude): Trenner `+`, Online-Tests außen vor.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
