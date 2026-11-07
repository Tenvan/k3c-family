# PS1.1 · B-701 durchgehen

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/ps1
- **Abhängig von:** –
- **Tickets:** B-701
- **Kriterien:** AC-01

## Ziel

Ralf hat jede D-Zeile in B-701 bestätigt oder geändert; Änderungen sind als Folge-Ticket oder als geänderter Default vermerkt.

## Kontext

B-701 › Notizen sammelt rund 100 Defaults, die die Agenten in PV1, RD1, WB1, RV1, MC1 und MG1–MG7 ohne Rückfrage gesetzt haben. Die Liste unten gruppiert sie nach Kritikalität, also danach, wie teuer eine spätere Änderung wäre. Die Lesbarkeits-Befunde (D-MG3-08, D-MG4-07, D-MG5-04, D-MG6-08, D-MG7-06) gehören zu PS1.3 und werden hier nur übersprungen.

## Erlaubte Dateien

`docs/backlog/B-701-pap-gesammelte-fragen.md` (nur Spalte Status, durch Claude in PS1.5 nach Ralfs Notizen).

## Nicht-Ziele

Korrekturen umsetzen; Lesbarkeit der Assets (PS1.3).

## Schritte

Ralf geht die Gruppen von oben nach unten durch und notiert je Zeile ✅ (bestätigt) oder ✏️ mit der Änderung.

### 1. Kritisch: Verträge, Maße und Daten für US1

Eine Änderung hier zieht Neu-Renders oder Code in US1 nach sich.

| Lane | Zeilen |
|---|---|
| renderer (PV1, RD1) | D-01, D-03, D-04, D-05, D-13, D-15, D-RD1-05, D-RD1-06 |
| mcp (MC1) | D-06, D-MC1-02, D-MC1-10 |
| US1 | D-09, D-11 |
| reittiere (MG2) | D-MG2-06 |
| truppen (MG3) | D-MG3-01, D-MG3-02 |
| gegner (MG4) | D-MG4-03 |
| bosse (MG5) | D-MG5-01 |
| gebaeude (MG6) | D-MG6-04, D-MG6-05 |
| hintergruende (MG7) | D-MG7-02, D-MG7-03 |

### 2. Mittel: Look, Farben, Rigs

Änderbar per `assets_set` je Asset.

| Lane | Zeilen |
|---|---|
| renderer | D-07, D-08, D-14, D-17, D-RD1-03, D-RD1-04 |
| monarch (MG1) | D-MG1-03, D-MG1-04, D-MG1-05 (schon entschieden) |
| reittiere | D-MG2-01, D-MG2-08 |
| truppen | D-MG3-03, D-MG3-04, D-MG3-07 |
| gegner | D-MG4-01, D-MG4-02, D-MG4-05, D-MG4-06 |
| bosse | D-MG5-02, D-MG5-03 |
| gebaeude | D-MG6-01, D-MG6-02 |
| hintergruende | D-MG7-01, D-MG7-04 |

### 3. Niedrig: Werkzeuge und Ablauf

Betrifft nur Workbench, Tests oder Arbeitsweise.

| Lane | Zeilen |
|---|---|
| renderer | D-02, D-16, D-RD1-01, D-RD1-02, D-RD1-07, D-RD1-08 |
| ui (WB1) | D-10, D-12, D-18, D-19, D-20, D-21, D-22 |
| review (RV1) | D-RV1-1 … D-RV1-9, D-MG2-07 |
| mcp | D-MC1-01, D-MC1-03 … D-MC1-09, D-MC1-11 … D-MC1-15 |
| monarch | D-MG1-01, D-MG1-02, D-MG2-02 |
| reittiere | D-MG2-03, D-MG2-04, D-MG2-05 |
| truppen | D-MG3-05, D-MG3-06 |
| gegner | D-MG4-04 |
| gebaeude | D-MG6-03, D-MG6-06 (erledigt), D-MG6-07 |
| hintergruende | D-MG7-05 |

## Fertig, wenn

Jede Zeile der drei Gruppen hat ✅ oder ✏️ in Ralfs Notiz (Ergebnis).

## Prüfen

Zählung: Zeilen in B-701 › Notizen = Zeilen mit Vermerk (ohne die fünf Lesbarkeits-Zeilen aus PS1.3).

## Ergebnis

2026-11-07, Ralf im Dialog. Alle Zeilen ✅ außer diesen ✏️ (Folge-Tickets in PS1.5):

- **D-RD1-06** renderKey des Reiters enthält den Hash der Reittier-Spec.
- **D-MG2-06** einheitlicher Maßstab aller Reittiere, Rahmen wächst ohne Skalierung.
- **Materialien** (D-MG2-01, D-MG3-04, D-MG4-05, D-MG5-02, D-MG7-04, D-MG8-02): Stil um schwarz, eisen, leinen, kupfer, kristall und Biom-Töne erweitern, betroffene Assets umstellen.
- **D-MG2-08** Rig `vierbeiner`: Beinlänge und Rumpf je Tier, Gangarten je Tier (Elefant ohne Galopp).
- **D-MG3-07, D-MG4-06** Keyframe-Feld `sichtbar` je Teil; Bogenschützen und Lavaschleim nachziehen.
- **D-US2-01** Natur in alter Spielhöhe (Bäume ≈ 360 px) und 2–3 Formen je Art.
- **D-US2-03 / D-US1-02** Monarch mit `<gangart>-<reittier>` für alle Reittiere (nach Rig-Ausbau).
- **D-US1-03** Keyframes für give, hit, death, respawn (Monarch, Truppen).

Zusätzlich in PS1.3/PS1.4 ansehen: D-09/D-US1-05 (Schatten), D-MG6-05 (Burg-Stufen), D-MG1-03/04 (Spielerfarben, Satteldecke). Kleinbefunde D-MG8-04, D-US1-06, D-US2-02/04 laufen nach PS1.3/CU1.
