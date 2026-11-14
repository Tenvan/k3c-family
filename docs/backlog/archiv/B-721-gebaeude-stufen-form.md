# B-721 · Gebäude steigern sich in Form und Material, Kristall leuchtet, nichts wird abgeschnitten

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** MG11
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

PS1.3: Turmspitzen der Burg in Stufe 5 abgeschnitten (Rahmen-Bug); Stufen steigern sich nur in der Farbe; Kristall leuchtet nicht. D-MG6-08: Formen 1–2 px über dem Rahmen, Taverne-Schild wandert mit der Kette, Banner-Reste, Mittelbau ohne Quaderfugen.

## Ziel

Jede Ausbaustufe ist an der Form erkennbar, nichts ist abgeschnitten.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Rahmen-Bug beheben; Stufen mit Formzuwachs (Zinnen, Höhe, Anbauten); Kristall mit leuchtenden Fugen; Punkte aus D-MG6-08.

## Nicht-Ziele

Neue Gebäude.

## Regeln und Einschränkungen

Assets nur über MCP `assets_*` (Entscheidung 005); Golden nur bei gewollter Änderung neu.

- **Bereich:** `public/grafik/gebaeude/**`, `tools/paper/golden/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test: kein deckendes Pixel in Randzeile oder -spalte eines Gebäude-PNGs.
- **AC-02** Stufe n+1 unterscheidet sich von Stufe n in der Silhouette (Sichtprüfung).
- **AC-03** Kristallstufe mit Leuchten und Fugen; Punkte aus D-MG6-08 erledigt.

## Offene Fragen

keine

## Notizen

Quelle: PS1.3 Punkt 6, D-MG6-08. Nach B-712. Lane `gebaeude`.

Umgesetzt in MG11.1 (2026-11-14): Rahmen mit Rand, Kette Holz → Stein → Kupfer → Eisen → Kristall, Formzuwachs je Stufe, leuchtende Fugen, D-MG6-08 erledigt (Burg-Ausbau-Variante im Client: B-881). **Sichtprüfung durch Ralf offen** (AC-02, Review der Workbench, Gruppe `gebaeude-paper`).
