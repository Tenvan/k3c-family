# B-070 · Die .gitignore ignoriert reports/, saves/ und certs/ nur an der Repo-Wurzel

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** erledigt
- **Sprint:** I1
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat („baue aus den offenen Punkten den nächsten Sprint und aktiviere ihn“, Sprint I1 Revision 1)

## Ausgangslage

`.gitignore` enthält `reports/`, `saves/` und `certs/` ohne führenden `/`. Git ignoriert damit jeden Ordner dieses
Namens im ganzen Repo, z. B. `tools/k3c-dev/internal/gamedata/testdata/reports/`. M2.2 hat die Beispieldateien deshalb
flach abgelegt; `/logs/` ist seit M1.3 schon verankert.

## Ziel

Die .gitignore ignoriert reports/, saves/ und certs/ nur an der Repo-Wurzel. Nutzen: Test- und Beispielordner mit
diesen Namen werden nicht mehr still ausgelassen.

## Beteiligte und Zielgruppen

Alle, die Tests mit Beispieldaten anlegen.

## Anforderungen

- `/reports/`, `/saves/`, `/certs/` statt der unverankerten Muster; die echten Ordner an der Wurzel bleiben ignoriert.

## Nicht-Ziele

Andere Muster; Beispieldateien von M2.2 umziehen.

## Regeln und Einschränkungen

Vorher mit `git check-ignore -v` prüfen, dass nichts Getracktes plötzlich auftaucht und die Wurzel-Ordner ignoriert bleiben.

## Beispiele

`git check-ignore -v tools/x/testdata/reports/a.json` → nicht ignoriert; `git check-ignore -v reports/a.json` → ignoriert.

## Ausnahme- und Fehlerfälle

`server/reports.mjs` schreibt nach `reports/` an der Wurzel; der Pfad bleibt ignoriert.

## Akzeptanzkriterien

- **AC-01** Die drei Muster sind verankert; `git check-ignore` zeigt die Beispiele wie oben.

## Offene Fragen

keine

## Notizen

Gefunden in M2.2 (2026-09-30).
