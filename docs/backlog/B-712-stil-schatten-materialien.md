# B-712 · Stil zeigt Papierschatten wie der Prototyp und kennt Metall-, Kristall- und Biom-Materialien

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RD2
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-07, Chat (mit Sprint RD2)

## Ausgangslage

PS1.2: Drop-Shadow und Papierstruktur der resvg-Ausgabe sind deutlich schwächer als im Prototyp `docs/decisions/005/prototyp-monarch-rig.html`. PS1.1: Die Stil-Datei hat kein Schwarz, Metall, Leinen, Kupfer, Kristall und keine Biom-Töne; MG2–MG8 behelfen sich mit Ersatz-Materialien (D-MG2-01, D-MG3-04, D-MG4-05, D-MG5-02, D-MG7-04, D-MG8-02).

## Ziel

Papier-Look wie im Prototyp und eigene Materialien statt Ersatz.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Stil-Werte `lage`, `auflage`, `struktur` an den Prototyp angleichen.
- Neue Materialien `schwarz`, `eisen`, `leinen`, `kupfer`, `kristall` und Biom-Töne (wald, hoehle, mine, eisenstollen, kristall).
- Betroffene Assets aus den genannten D-Zeilen auf die neuen Materialien umstellen; alle Gruppen neu rendern (`task gen:paper`).

## Nicht-Ziele

Formänderungen an Assets (eigene Tickets).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Monarch idle 4 neben dem Prototyp: Schatten und Struktur gleich stark (Sichtprüfung Ralf).
- **AC-02** `public/grafik/stil.paper.json` enthält die fünf Materialien und je Biom einen Ton; Schema-Test grün.
- **AC-03** Kein Asset nutzt mehr die Ersatz-Materialien aus D-MG2-01, D-MG3-04, D-MG4-05, D-MG5-02, D-MG7-04, D-MG8-02 (Liste im Ergebnis).
- **AC-04** Golden aller Gruppen neu, `bun test` in `tools/paper` grün.

## Offene Fragen

keine

## Notizen

Quelle: PS1.1 (Materialien), PS1.2 (Papierkanten). Lane `stil`; vor allen Asset-Tickets umsetzen.
