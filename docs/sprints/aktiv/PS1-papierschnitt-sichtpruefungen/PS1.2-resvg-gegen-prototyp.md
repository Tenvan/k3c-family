# PS1.2 · resvg-Ausgabe gegen Prototyp

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** –
- **Umgebung:** live
- **Branch:** sprint/ps1
- **Abhängig von:** PS1.1
- **Tickets:** B-699
- **Kriterien:** AC-02

## Ziel

Ralf hat bestätigt, dass die resvg-Ausgabe des Monarchen dem Look des Prototyps entspricht, oder die Abweichungen notiert.

## Kontext

Früher die PV1-Sichtprüfung. Prototyp: `docs/decisions/005/prototyp-monarch-rig.html` (im Browser öffnen). Ausgabe: `public/grafik/figuren/monarch/*.png` und `reittiere/pferd/*.png`, in der Workbench › Assets › Review bzw. Katalog. Bekannte gewollte Abweichungen: kein Hermelin-Saum am Umhang (D-RD1-03), Umhang `smaragd` mit Länge 79.8 (D-MG1-05), Reit-Rahmen aus dem Pferd (D-RD1-05).

## Erlaubte Dateien

keine (Notizen ins Ergebnis).

## Nicht-Ziele

Andere Gruppen (PS1.3).

## Schritte

1. Prototyp und Assets › monarch nebeneinander öffnen.
2. Prüfliste je Punkt ✅/✏️:
   - Papierkanten: Falz-Licht und Schatten wie im Prototyp, keine harten Pixelkanten
   - Farben: koenigsblau, smaragd, gold, hermelin wirken wie im Prototyp
   - Proportionen: Kopf, Krone, Rumpf, Beine
   - idle und lauf: Gelenke knicken nicht falsch, Umhang schwingt
   - Reiten (schritt, trab, galopp): Sitz auf dem Pferd, Schweif ganz im Bild
   - Kleine Stufe (`*.s2.png` falls vorhanden): noch lesbar
3. Abweichungen mit Asset, Animation und Frame notieren.

## Fertig, wenn

Alle Punkte der Prüfliste vermerkt.

## Prüfen

Ergebnis nennt je Punkt ✅ oder die Abweichung.

## Ergebnis

–
