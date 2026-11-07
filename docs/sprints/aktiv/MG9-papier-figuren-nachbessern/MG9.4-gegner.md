# MG9.4 · Gegner: unterscheidbar, richtig groß, Reste aus MG4

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gegner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/mg9
- **Abhängig von:** –
- **Tickets:** B-719
- **Kriterien:** AC-04

## Ziel

Gegner sind auf einen Blick zu unterscheiden und richtig groß: Troll mit zwei sichtbaren Beinen im Angriff, Kristallspinne mindestens so hoch wie der Goblin, alle Punkte aus D-MG4-07 behoben.

## Kontext

Ticket B-719. Specs `public/grafik/gegner/*.paper.json`. Befunde D-MG4-07: Kristallwächter zu nah am Troll (gleiches Rig, Gesicht, Gürtel, Stiefel; kurze Arme), Pfeil des Goblin-Bogenschützen ragt über den Bogen, Bogen schleift im Lauf, Feuerball klein und am Stiel, Flammenkrone ähnelt der Monarch-Krone, Keule des Trolls verdeckt im Lauf den Kopf, Hand-Scherbe des Splitterlings lose, Elite-Bogenschütze so groß wie der Goblin; PS1.3: Troll im Angriff einbeinig, Kristallspinne 97 × 59 px zu klein. Rig-Proportionen aus RD2.4 (`proportionen`), Größen D-MG4-03.

## Erlaubte Dateien

- `public/grafik/gegner/**`
- `public/grafik/truppen/elite-bogenschuetze*` (nur Größe)
- `tools/paper/golden/**`
- `docs/sprints/aktiv/MG9-papier-figuren-nachbessern/**`

## Nicht-Ziele

Bosse (B-720, MG10).

## Schritte

1. Troll `angriff`: beide Beine sichtbar; Keule im Lauf neben dem Kopf.
2. Kristallwächter über `proportionen` (lange Arme, eigene Gürtel/Stiefel/Gesicht) vom Troll absetzen.
3. Kristallspinne größer (Höhe ≥ Goblin, Messwert), Goblin-Bogenschütze (Pfeil, Bogen), Feuerball größer, Flammenkrone anders als die Monarchenkrone, Splitterling-Scherbe fest, Elite-Bogenschütze größer.
4. `task gen:paper` für `gegner`, Vorschau ansehen, Golden schreiben, Punkte im Ergebnis abhaken.

## Fertig, wenn

- [ ] AC-04: Troll `angriff` mit zwei sichtbaren Beinen (B-719/AC-01).
- [ ] AC-04: Kristallspinne mindestens so hoch wie der Goblin, Messwert im Ergebnis (B-719/AC-02).
- [ ] AC-04: Jeder Punkt aus D-MG4-07 erledigt, Liste im Ergebnis (B-719/AC-03).

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
