# PS1.4 · Spiel und Review-Lauf

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Lane:** abnahme
- **Modell:** –
- **Umgebung:** live
- **Branch:** sprint/ps1
- **Abhängig von:** PS1.3, US1
- **Tickets:** B-699
- **Kriterien:** AC-04

## Ziel

Ralf hat das Spiel im PC-Browser im Papierschnitt-Look abgenommen und eine echte Änderung am Monarchen nur im Bereich Assets reviewt (005/AC-11).

## Kontext

Erst nach US1 (Spiel lädt Papierschnitt-Grafik). Dienste über die Workbench starten (`svc_start`), Spiel unter `get_urls`.

## Erlaubte Dateien

keine (Notizen ins Ergebnis).

## Nicht-Ziele

Xbox-Abnahme (eigene Geräte-Abnahmen); Balancing.

## Schritte

### Spiel
1. Spiel mit 2 Spielern (Split-Screen) starten.
2. Prüfliste ✅/✏️:
   - Monarch läuft, reitet (schritt/trab/galopp), Spielerfarben unterscheidbar
   - Truppen, Gegner, Bosse in passender Größe zum Monarchen
   - Gebäude bauen und ausbauen: Stufen wechseln sichtbar
   - Hintergründe je Biom mit Parallax, keine Nähte an den Kacheln
   - Schatten auf dem Boden (D-09)
   - keine Pixel-Grafik mehr sichtbar
   - flüssig (kein spürbares Ruckeln)

### Review-Lauf
1. Claude ändert per `assets_set` ein Merkmal am Monarchen (z. B. Kronenhöhe).
2. Ralf öffnet Assets › Review: Diff, Overlay und Teil-Markierung zeigen die Änderung, ohne eine Datei zu öffnen.
3. Ralf lehnt ab oder nimmt an; Ablehnung setzt zurück.

## Fertig, wenn

Beide Prüflisten vermerkt.

## Prüfen

Ergebnis nennt je Punkt ✅ oder den Befund.

## Ergebnis

–
