# PS1.4 · Spiel und Review-Lauf

- **Status:** fertig
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

2026-11-07, Ralf im Browser-Pane (neuer Spielstand) und in der Workbench.

### Spiel ✏️
- Der Bogenschütze schwebt deutlich über dem Turm. Er muss immer auf der Plattform stehen, auch wenn das Gebäude wechselt. Bild: [Bogenschütze über dem Turm](../../../playtests/ps1/ps1.4-bogenschuetze-turm.webp)
- Die Animation des berittenen Monarchen passt nicht zur Laufgeschwindigkeit.
- Auf dem Pferd ist kein Idle zu sehen.
- Die Bäume sind zu klein. Die rodbaren Birken, Tannen und Pappeln sind kaum von den Bäumen im Hintergrund zu unterscheiden. Der Hintergrund soll deshalb keine Bäume zeigen, sonst passt das Roden nicht (siehe D-US2-01). Bild: [Bäume und Hintergrund](../../../playtests/ps1/ps1.4-baeume-hintergrund.webp)
- Das Tor bzw. Portal ist zu klein, gemessen am berittenen Monarchen. Bild: [Tor und Portal](../../../playtests/ps1/ps1.4-tor-portal.webp)
- Pixel-Reste: nur die bekannten (Gerüste, Plantage) ✅

### Review-Lauf ✅
Claude hat per `assets_set` den Umhang des Monarchen auf 66 gekürzt. Diff, Overlay und Teil-Markierung waren unter Assets › Review zu sehen, ohne eine Datei zu öffnen. Ralf hat abgelehnt, die Änderung ist zurückgesetzt (005/AC-11 erfüllt).
