# B-112 · Der Hub wird in fünf Stufen ausgebaut, Mauern und Türme haben fünf Materialstufen

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** W1
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Hub hat feste Bauplätze mit je einem Gebäude (`data/hub.json`), Mauer und Turm haben nur eine Stufe (Holz); es gibt weder Hub-Ausbau noch Reparatur (`docs/rules/archiv/ist-material-gebaeude.md`).

## Ziel

Jeder Hub hat Ausbaustufen 1 bis 5 (Gold plus Material der neuen Stufe), Mauer und Turm werden am selben Platz auf Stufe 1 bis 5 (Holz bis Kristall) ausgebaut, Gebäude schalten mit der Hub-Stufe frei. Nutzen: Fortschritt über Tiefe und Material (`docs/rules/materialien-gebaeude.md` §§ 2–4).

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen) und Bauern; Werte pflegt REG, Feintuning mit B-099 und B-015.

## Anforderungen

- `data/hub.json` › `levels`: Ausbau auf Stufe 2: 100 Stein + 50 Gold, 3: 150 Kupfer + 100 Gold, 4: 200 Eisen + 200 Gold, 5: 250 Kristall + 400 Gold.
- `data/buildings.json`: Mauer und Turm mit Stufen 1–5 (Kosten, HP, Bauzeit laut `materialien-gebaeude.md` § 3.1); Ausbau läuft am selben Bauplatz (Gold zahlen, Material abbuchen, Bauer baut).
- Gebäude und Mauer-/Turm-Stufen der Stufe n sind erst mit Hub-Stufe n baubar (Liste in `materialien-gebaeude.md` § 3).
- Zerstörung: Mauer/Turm verliert die Stufe (neu ab Holzstufe), die Hub-Stufe bleibt; Gold und Material sind verloren.
- Reparatur: Bauern reparieren beschädigte Gebäude zwischen den Wellen kostenlos (Anteil der Bauzeit).
- Ereignisse `upgraded` und `repaired` für Anzeige und Messung.

## Nicht-Ziele

Neue Gebäude-Wirkungen (B-116), Lager und Material (B-113), Anzeige (B-117).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md`; Werte nur in `data/`. Voraussetzung: Insel-Vorrat aus B-100/B-113. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Hub-Stufe 1, Vorrat 100 Stein: Spieler zahlen 50 Gold → Material wird abgebucht → Bauer baut → Hub-Stufe 2 → Steinmauer baubar.

## Ausnahme- und Fehlerfälle

Material fehlt → Ausbau wartet (Wartezeit sichtbar, B-117). Ausbau während einer Welle → erlaubt, Bauer baut erst nach Gefahr.

## Akzeptanzkriterien

- **AC-01** Test: Ausbau auf Stufe 2 bis 5 bucht Gold und Material laut `hub.json` ab.
- **AC-02** Test: Mauer-Stufe 2 am selben Platz mit Kosten und HP laut Daten; Stufe 3 braucht Hub-Stufe 3.
- **AC-03** Test: Gebäude einer höheren Stufe sind vor dem Hub-Ausbau nicht bezahlbar.
- **AC-04** Test: Zerstörung setzt die Mauer-Stufe zurück, die Hub-Stufe bleibt.
- **AC-05** Test: Reparatur stellt HP zwischen den Wellen her, ohne Gold und Material.
- **AC-06** Spielstand speichert Hub-Stufe und Gebäude-Stufen; Golden-Daten sind aktualisiert; `task check:go` grün.

## Offene Fragen

keine

## Notizen

Aus R2.2 und R2.3. Abhängig von B-100 und B-113. Zahlen sind Startwerte (B-015).
