# B-586 · Im Bau zeigt der Platz ein Gerüst in der Form des neuen Gebäudes

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sobald ein Bauplatz bezahlt ist und gebaut wird, verschwindet das Bauschild (B-584), bis das Gebäude fertig ist. Im Bau zeigt `siteView.ts` nur den Fortschrittsbalken. Für die Burg gibt es ein Baugerüst (`HUB_SCAFFOLD_TEXTURE`, GEB11). Ralf (2026-10-24): Es soll im Bau ein Gerüst in der Form des neuen Gebäudes stehen.

## Ziel

Ein Platz im Bau zeigt ein Gerüst in Größe und Umriss des entstehenden Gebäudes, mit Baufortschritt; das Bauschild verschwindet nicht ohne Ersatz.

## Beteiligte und Zielgruppen

Spielende; 🧑 entscheidet über das Aussehen und nimmt ab; Grafik über das Asset-Studio (GEB).

## Anforderungen

- Gerüst als Pixel-Grafik aus dem Asset-Studio (Entscheidung Ralf, 2026-10-24), je Gebäudeart in der Form des fertigen Gebäudes (Größe aus `SITE_SIZE` bzw. dem Gebäude-Sprite).
- Gilt für die Zustände `waitingMaterial`, `waitingWorker` und den Bau mit Fortschritt.
- Der Baufortschritt bleibt sichtbar.

## Nicht-Ziele

Das Burg-Gerüst (GEB11), Ausbau-Gerüste bestehender Gebäude (B-545).

## Regeln und Einschränkungen

Nur zeichnen (ADR 001); Asset-Weg über das Studio, Regeln aus GEB; Datei ≤ 400 Zeilen.

## Beispiele

Werkstatt wird gebaut → Holzgerüst in Werkstatt-Größe statt leerer Fläche.

## Ausnahme- und Fehlerfälle

Fehlt die Gerüst-Grafik einer Gebäudeart → einfache gezeichnete Form in `SITE_SIZE` als Rückfall.

## Akzeptanzkriterien

- **AC-01** Ein Platz im Bau zeigt ein Gerüst in der Form des Gebäudes (Beobachtung am Gerät).
- **AC-02** Das Schild und das Gerüst erscheinen nie gleichzeitig.

## Offene Fragen

keine (Entscheidung: Pixel-Grafik aus dem Asset-Studio, 2026-10-24). Offen bleibt nur der Umfang der Grafiken (eine je Gebäudeart oder wenige Größenklassen); Default: Größenklassen, 🧑 bestätigt bei der Spec.

## Notizen

Aus der Abnahme am 2026-10-24, Folge von B-584. Spec: Entwurf.
