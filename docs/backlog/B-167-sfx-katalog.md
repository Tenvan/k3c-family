# B-167 · Jedes wichtige Ereignis hat einen Sound mit Quelle und Lizenz

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SO2
- **Projekt:** SND
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint SO2

## Ausgangslage

Das Spiel hat keinen Ton (B-011). SO1 liefert den Audio-Kern (Mixer, Entsperren, Lautstärke je Gerät); die Ereignisse kommen als Feedback-Events vom Server (B-139, B-140). Ein Katalog Ereignis → Sound → Quelle → Lizenz existiert nicht; `public/` hat kein Audio-Verzeichnis.

## Ziel

Ein Katalog ordnet jedem wichtigen Ereignis einen Sound mit Quelle und Lizenz zu, und die Sounds sind eingebaut. Nutzen: Rückmeldung für Münze, Schlag, Bauen und Nacht, ohne dass der Client rechnet.

## Beteiligte und Zielgruppen

🧑 wählt Quellen und Stil (Q15 in `docs/fragenkatalog.md`); der Agent erstellt den Katalog und baut ein; Spieler am TV und am Handy.

## Anforderungen

- Katalog (`docs/assets/sounds.md`, neu): Ereignis, Sound-Datei, Quelle, Urheber, Lizenz, Status (zugeordnet oder Lücke). Ereignisse (Feedback-Events aus Q08): Treffer, Gegner-Tod, Münze aufheben, Münze geben, Pfeil, Schlag, Bau-Fortschritt, Bau fertig, Tod, Skill, Nacht naht, Portal öffnet; dazu getrennt `revive` (Respawn nach der Wartezeit) und `revived` (Wiederbeleben durch einen Mitspieler, Q62).
- Quellen: Kenney (CC0), OpenGameArt, freesound (CC0 oder CC-BY); als Lückenfüller selbst erzeugte Retro-SFX per Web Audio.
- Dateien unter `public/audio/` im Format, das SO1 aus B-166 wählt; Credits sofort in einer CREDITS-Datei dort und auf der Credits-Seite (B-165).
- Einbau über den Audio-Kern: Event aus dem Server → Sound; Positions-Dämpfung im Split-Screen (jeder hört seinen Bereich, Warnungen global), Regel aus SO1.
- Nacht leise (Q15): Nachts spielen die Effekte gedämpft, kindgerecht statt erschreckend; der Pegel ist ein Wert des Audio-Kerns.
- Nur das Ereignis-Mapping im Client; keine Spiel-Logik.

## Nicht-Ziele

Audio-Kern (B-011/SO1), Musik (B-168), Hörprobenseite (B-169), eigene Kompositionen.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots und rechnet nichts; Lizenz CC0 oder CC-BY mit Credits; B nicht belegen; 2 Spieler gleichzeitig. Voraussetzung: B-166, SO1, F3 und F4 (Feedback-Events).

## Beispiele

Spieler hebt eine Münze auf → Münz-Sound; Nacht naht → Warn-Sound bei beiden Spielern.

## Ausnahme- und Fehlerfälle

Sound-Datei fehlt oder lässt sich nicht dekodieren → Ereignis bleibt stumm, Eintrag im Konsolen-Log, kein Absturz. Browser blockiert Audio vor der ersten Eingabe → Ton startet nach der ersten Taste (B-011).

## Akzeptanzkriterien

- **AC-01** `docs/assets/sounds.md` enthält jede Zeile der Ereignisliste oben mit Sound, Quelle, Lizenz oder als Lücke mit Ticket (Test auf Vollständigkeit).
- **AC-02** Jede Sound-Datei unter `public/audio/` hat einen Credit-Eintrag (Test, wie B-165/AC-01).
- **AC-03** Beobachtung am TV: Münze aufheben, Schlag, Gegner-Tod, Bauen fertig und Nacht naht lösen je ihren Sound aus.
- **AC-04** Test: Das Mapping Ereignis → Sound liest nur Events und ändert keinen Spielzustand.
- **AC-05** Eine fehlende Sound-Datei lässt das Spiel ohne Fehlermeldung am Bildschirm weiterlaufen.
- **AC-06** Test: Nachts spielen die Effekte leiser als am Tag (Q15, Nacht leise).

## Offene Fragen

- Hub-Ausbau, Boss-Auftritt und UI-Klicks: Woher kommen die Auslöser? Q08 nennt dafür kein Feedback-Event; bis das geklärt ist, stehen sie nicht in der Ereignisliste.

## Notizen

Quelle: `docs/plan-weiterentwicklung.md` Schiene A, SO2. Vorbild für die Tabelle ist B-161.
