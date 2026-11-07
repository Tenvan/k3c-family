# PS1.3 · Gruppen MG1–MG8 in Assets

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** –
- **Umgebung:** live
- **Branch:** sprint/ps1
- **Abhängig von:** PS1.2
- **Tickets:** B-699, B-701
- **Kriterien:** AC-03

## Ziel

Ralf hat jede Migrationsgruppe MG1–MG8 im Bereich Assets angesehen und zu jedem bekannten Lesbarkeits-Befund entschieden: so lassen oder nachbessern.

## Kontext

Die Reviews der MG-Sprints haben Befunde zurückgestellt, die nur eine Sichtprüfung entscheiden kann. Ort: Workbench › Assets › Katalog, Gruppe wählen, Animationen durchklicken.

## Erlaubte Dateien

keine (Notizen ins Ergebnis).

## Nicht-Ziele

Nachbessern selbst (Folgetickets in PS1.5).

## Schritte

Je Gruppe: alle Assets ansehen, dann die Befunde abhaken (✅ lassen / ✏️ nachbessern).

1. **figuren (MG1, Lane monarch):** Spieler 1–4 im Split-Screen unterscheidbar (Rock blau/rot/purpur/leder, D-MG1-03); Satteldecke bleibt blau beim Reiten (D-MG1-04, B-711).
2. **reittiere (MG2, reittiere):** Farben ohne eigenes Material, z. B. Rappe dunkelbraun statt schwarz (D-MG2-01); Reittiere ~4 % unterschiedlich skaliert (D-MG2-06); Wölfe, Höllenhund, Elefant im Rig `vierbeiner` (D-MG2-08).
3. **truppen (MG3, truppen):** D-MG3-08: Helm-Nasal unter dem Querbalken, Schild nur in lauf/angriff, Waffe schwingt in lauf voll mit, Stabknauf vor dem Bart, Strohhut so gelb wie die Krone; Pfeil (D-MG3-07).
4. **gegner (MG4, gegner):** D-MG4-07: Kristallwächter zu nah am Troll, Goblin-Pfeil ragt über den Bogen, Bogen schleift, Feuerball klein, Flammenkrone ähnelt Monarch-Krone, Troll-Keule verdeckt den Kopf; Größen (D-MG4-03).
5. **bosse (MG5, bosse):** D-MG5-04: Lava-Golem und Splitter-Titan schwer zu unterscheiden, Wangenpunkt bei beiden Golems, Lava-Golem-Arme in idle hinter dem Rumpf, heller dünner Schwanz der Ratten-Königin, Holzschwert des Troll-Königs kontrastarm; Größe (D-MG5-01).
6. **gebaeude (MG6, gebaeude):** D-MG6-08: Kristallstufe ohne leuchtende Fugen, kleine Turmspitze, Formen 1–2 px über dem Rahmen, Taverne-Schild wandert mit der Kette, Burg-Banner-Reste, Mittelbau ohne Quaderfugen; Materialkette der Stufen (D-MG6-02).
7. **hintergruende (MG7, hintergruende):** D-MG7-06: Gerüst in `hg-mine-fern-*` kaum sichtbar, Mine/Eisenstollen in der Tiefe flach, `hg-eisenstollen-fern-v1/v2` fast gleich, Glut schwach und Rost als Ovale, Felsblöcke in `hg-hoehle-nah-*` wie Kuppeln, Tannen in `hg-wald-fern-*` verschwinden; Varianten v3–v5 nachziehen? (D-MG7-01); Paletten (D-MG7-04).
8. **MG8 (Natur, Objekte, Münzen, Icons):** sobald gemerged, ganze Gruppe ansehen; Befunde frei notieren.

## Fertig, wenn

Jede Gruppe angesehen, jeder genannte Befund mit ✅ oder ✏️.

## Prüfen

Ergebnis hat acht Abschnitte, je Gruppe einen.

## Ergebnis

2026-11-07, Ralf im Dialog anhand von `assets_preview`-Bildern. Alle Gruppen ✏️:

1. **figuren:** spieler4 eine kontrastreiche Farbe statt `leder`, weil der Rock auf dem braunen Pferd verschwimmt. Die Satteldecke folgt der Rockfarbe (B-711). Sonst ✅.
2. **reittiere:** Pfoten bzw. Füße je Tier und eine eigene Wolfssilhouette, als Teil des Rig-Tickets (D-MG2-08). Material und Maßstab wie in PS1.1.
3. **truppen:** Der Krieger greift zum Gegner hin an (bisher verdeckt der Schwertarm das Gesicht). Dazu alle Punkte aus D-MG3-08 und der Pfeil (D-MG3-07).
4. **gegner:** alle Punkte aus D-MG4-07. Dazu wirkt der Troll im Angriff einbeinig, und die Kristallspinne ist mit 97 × 59 px zu klein.
5. **bosse:** komplett neu gestalten. Jeder Boss bekommt eine eigene Silhouette statt der Krieger-Grundform, das braucht Rig-Proportionen je Figur. Dazu alle Punkte aus D-MG5-04.
6. **gebaeude:** Die Turmspitzen der Burg sind in Stufe 5 abgeschnitten, ein Rahmen-Bug. Kristall soll leuchten und Fugen zeigen. Die Stufen sollen sich auch in der Form steigern, nicht nur in der Farbe. Dazu alle Punkte aus D-MG6-08.
7. **hintergruende:** neu gestalten. Mehr Tiefe und Details je Ebene, Höhlenfelsen nicht mehr wie Kuppelzelte, eine Rückwand im Eisenstollen, die Glut nicht mehr wie rote Vögel. Dazu die Varianten v3–v5 und alle Punkte aus D-MG7-06, nach der Material-Erweiterung.
8. **MG8:** Die Icons sind bei 32 px kaum lesbar und werden nachgebessert. Die Münze bekommt eine Dreh-Animation. Dazu alle Punkte aus D-MG8-04. Die Tanne ✅.
