# MG9.3 · Truppen: Angriff zum Gegner, Reste aus MG3

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** truppen
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/mg9
- **Abhängig von:** –
- **Tickets:** B-718
- **Kriterien:** AC-03

## Ziel

Truppen sind in allen Animationen lesbar: der Krieger schlägt vor dem Körper zum Gegner, alle Punkte aus D-MG3-08 sind behoben; Truppen haben `give`, `hit`, `death`, `respawn`.

## Kontext

Ticket B-718 und B-717/AC-02 (Truppen-Anteil). Specs `public/grafik/truppen/*.paper.json` (Rig `figur`). Befunde D-MG3-08: Helm-Nasal unter dem Querbalken (Krieger, Elite), Schild nur in lauf/angriff sichtbar, Waffe schwingt in `lauf` voll mit, Stabknauf vor dem Bart (Landstreicher), Strohhut so gelb wie die Krone; dazu der Schwertarm, der im Angriff das Gesicht verdeckt (PS1.3). Mechanik `sichtbar` je Keyframe aus RD2.3.

## Erlaubte Dateien

- `public/grafik/truppen/**`
- `tools/paper/golden/**`
- `docs/sprints/aktiv/MG9-papier-figuren-nachbessern/**`

## Nicht-Ziele

Pfeil (B-714, erledigt); Monarch (MG9.2).

## Schritte

1. Krieger und Elite-Krieger: Angriffspose mit Klinge vor dem Körper, Gesicht frei; Nasal über den Querbalken.
2. Schild in allen Animationen sichtbar, Waffenschwung in `lauf` gedämpft, Stabknauf hinter den Bart, Strohhut helleres Strohgelb.
3. `give`, `hit`, `death`, `respawn` für alle Truppen.
4. `task gen:paper` für `truppen`, Vorschau ansehen, Golden schreiben, Punkte aus D-MG3-08 im Ergebnis abhaken.

## Fertig, wenn

- [ ] AC-03: Krieger `angriff`: Klinge vor dem Körper Richtung Gegner, Gesicht frei (B-718/AC-01).
- [ ] AC-03: Jeder Punkt aus D-MG3-08 erledigt, Liste im Ergebnis (B-718/AC-02).
- [ ] AC-03: `give`, `hit`, `death`, `respawn` für alle Truppen (B-717/AC-02).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

2026-11-07, Claude. Sichtprüfung per `assets_preview` und vergrößertem Ausschnitt (Claude); die Abnahme der Optik bleibt bei Ralf. Test: `tools/paper/golden/truppen.test.ts`.

- AC-03 / B-718/AC-01 umgesetzt: Krieger und Elite-Krieger `angriff`: Der Zwischenschlag liegt nicht mehr über dem Kopf (`oberarm-v` −45 statt −165, Unterarm nach vorn, Klinge waagrecht), Schwertarm und Klinge vor dem Körper, Gesicht frei.
- AC-03 / B-718/AC-02, Punkte aus D-MG3-08: (1) Helm-Nasal: Querbalken und Nasal sitzen am Helmrand über den Augen, das Nasal reicht senkrecht über den Nasenrücken statt als T unter dem Balken über dem Mund (Krieger, Elite) – erledigt. (2) Schild: im Stand nach vorn gehalten (`idle`-Keyframes `oberarm-h`, `unterarm-h`), damit er nicht hinter dem Rumpf verschwindet, sichtbar in idle, lauf, angriff – erledigt. (3) Waffe schwingt in `lauf` nicht mehr voll mit: Schwertarm schwingt nur noch um 10° (Keyframes), Klinge bleibt in Trageposition – erledigt. (4) Stabknauf: Stock des Landstreichers um ein Viertel kürzer (alle Pfade nach unten skaliert), Knauf liegt unter dem Bart – erledigt. (5) Strohhut: Material `leinen` statt `gold` – erledigt.
- AC-03 / B-717/AC-02 für die Truppen umgesetzt: `give`, `hit`, `death`, `respawn` für alle sechs Truppen (gleiche Keyframes wie der Monarch, Rumpf-Drehung −52°, damit der Kopf im Rahmen bleibt); Sterben und Aufstehen sind einfache Körperdrehungen, optisch schlicht.
