# B-723 · Natur hat Spielhöhe und mehrere Formen, Icons sind bei 32 px lesbar, die Münze dreht sich

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** MG11
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

D-US2-01: Bäume sind ≈ 146 statt 360 px hoch, je Art nur eine Form. PS1.3: Icons bei 32 px kaum lesbar, Münze ohne Drehung; D-MG8-04 (icon-holz, icon-kupfer, schild-axt, truhe-offen, wurzel, busch/hoehlenbusch, icon-pokal).

## Ziel

Natur in Spielhöhe mit Abwechslung, lesbare Icons, drehende Münze.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Bäume ≈ 360 px hoch, 2–3 Formen je Art (Birke, Tanne, Pappel); Fels- und Wurzel-Formen je Biom.
- Icons für 32 px vereinfacht; Punkte aus D-MG8-04.
- Münze mit Dreh-Animation.

## Nicht-Ziele

Einbau der Größe ins Spiel (B-724).

## Regeln und Einschränkungen

Assets nur über MCP `assets_*` (Entscheidung 005); Golden nur bei gewollter Änderung neu.

- **Bereich:** `public/grafik/objekte/**`, `public/grafik/icons/**`, `tools/paper/golden/**`
- **Abhängig von:** B-722
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Baum-PNGs 340–380 px hoch, je Art mindestens zwei Assets.
- **AC-02** Icons bei 32 px ohne Erklärung erkennbar (Sichtprüfung); Punkte aus D-MG8-04 erledigt.
- **AC-03** `muenze` hat eine Dreh-Animation mit mindestens 6 Frames.

## Offene Fragen

keine

## Notizen

Quelle: PS1.1 ✏️ D-US2-01, PS1.3 Punkt 8, D-MG8-04. Tanne ✅. Lane `natur`.
