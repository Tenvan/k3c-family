# B-638 · Das Balancing misst die Ressourcen für das Boot

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BS3
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑 mit BS2/BS3

## Ausgangslage

Das Balancing (`tools/k3c-workbench/internal/balance`, Zielkorridore `data/balance-targets.json`, Hinweise `balance-hints.json`) misst Gold, Holz, Mauern, Nächte und Bosse, aber kein Schiff; Hafen und Schiff gibt es noch nicht (B-637).

## Ziel

Die Ressourcen für das zu bauende Schiff (Münzen und Material je Bauabschnitt) stehen von Anfang an in der Werte-Basis, und der Balancing-Bericht zeigt, wann die Bots sie beisammen haben – schon in Meilenstein 1 als **Bedarf gegen Bestand** (ohne Mechanik), später mit echtem Bau und Ampel.

## Beteiligte und Zielgruppen

Claude (Werte-Runden), 🧑 (Abnahme der Korridore).

## Anforderungen

- Schiffskosten je Abschnitt in `data/` (Vorschlag eigener Eintrag `ship` in `buildings.json` mit `stages`: Münzen auf Skala B-635, Material Holz/Stein/Eisen …).
- **Stufe 1 (Meilenstein 1):** Bericht rechnet aus dem Vorratsverlauf, an welchem Tag die Bots jeden Abschnitt hätten bezahlen können („bezahlbar ab“), ohne dass gebaut wird.
- **Stufe 2 (nach B-637):** Bots (`coop2` u. a.) bauen Abschnitte, wenn Ressourcen reichen und Verteidigung steht; Bericht: Tag je Abschnitt, Bedarf vs. Bestand, Ampel gegen Korridor.
- Zielkorridor in `balance-targets.json` (Vorschlag: Schiff fertig zwischen Endboss-Sieg −2 und +1 Tage).
- Tuning-Hinweis in `balance-hints.json`, wenn das Schiff zu früh/zu spät bezahlbar ist.

## Nicht-Ziele

Boot-Mechanik selbst (B-637).

## Regeln und Einschränkungen

Läufe nur über `sim_test`; während BAS Seed `k3c-dev` (B-634).

## Beispiele

Meilenstein-Lauf `coop2` standard → „Schiff (Bedarf): Kiel bezahlbar T4, Rumpf T7, Segel T9 (Ziel T8–T11) ✅“. Nach B-637 dieselbe Zeile mit „gebaut“.

## Ausnahme- und Fehlerfälle

Boot nie fertig → Ampel rot mit fehlendem Material im Hinweis.

## Akzeptanzkriterien

- **AC-01** Schiffskosten je Abschnitt stehen in `data/`, ein Daten-Test prüft Skala und Schema.
- **AC-02** Meilenstein-Lauf `coop2` standard, Seed `k3c-dev`: Bericht nennt je Abschnitt „bezahlbar ab Tag“ mit Ampel.
- **AC-03** Nach B-637: Bericht nennt je Abschnitt den Bau-Tag; Schiff wird fertig und Insel 1 wird verlassen.

## Offene Fragen

- Zielkorridor für „Boot fertig“ → 🧑, Default: Vorschlag oben.

## Notizen

Wunsch 🧑 2026-10-26: „Ressourcen für das Boot mit ins Balancing aufnehmen“, ergänzt: „Die Ressourcen für das zu bauende Schiff auch mit aufnehmen.“ Begriff im Glossar: **Schiff** (Boot als Synonym).
