# MG9.3 · Truppen: Angriff zum Gegner, Reste aus MG3

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
