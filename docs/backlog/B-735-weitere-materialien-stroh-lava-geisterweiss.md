# B-735 · Weitere Ersatz-Materialien (stroh, lava, geisterweiss, fell-grau) in der Stil-Datei

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

RD2.2 (B-712) hat `schwarz`, `eisen`, `leinen`, `kupfer`, `kristall` und die Biom-Materialien `biom-wald`, `biom-hoehle`, `biom-mine`, `biom-eisenstollen`, `biom-kristall` angelegt. Aus den D-Zeilen bleiben Ersatz-Abbildungen ohne Material: Stroh (Strohhut Bauer, D-MG3-04), Lava/Glut (Lava-Golem, Lavaschleim, Feuergeist, D-MG4-05, D-MG5-02), Geisterweiß (Grubengeist, Alphawolf, D-MG4-05), `fell-grau`/`fell-schwarz` (D-MG2-01: Wolf auf `stoff`), Eisen bei Gegnern und Bossen (`stein` für Panzer, D-MG4-05), Kristall bei Gegnern (`edelstein`/`koenigsblau`).

## Ziel

Fehlende Materialien anlegen und die genannten Assets darauf umstellen.

## Beteiligte und Zielgruppen

Ralf (🧑) prüft die Optik; Claude setzt um; Lane `stil`.

## Anforderungen

Materialien `stroh`, `lava`, `geisterweiss`, `fell-grau` in `public/grafik/stil.paper.json`; Assets aus der Ausgangslage umstellen; `task gen:paper`, Golden neu.

## Nicht-Ziele

Formänderungen; B-712 (schon erledigt).

## Regeln und Einschränkungen

Nach RD2.2; vor MG9/MG10 (Gegner, Bosse).

## Beispiele

nicht relevant: Materialpflege.

## Ausnahme- und Fehlerfälle

nicht relevant: Materialpflege.

## Akzeptanzkriterien

- **AC-01** Die vier Materialien stehen in der Stil-Datei, Schema-Test grün.
- **AC-02** Kein Asset nutzt mehr die Ersatz-Abbildungen aus der Ausgangslage; Golden aller Gruppen neu.

## Offene Fragen

keine

## Notizen

Quelle: RD2.2, B-701 D-Zeilen.
