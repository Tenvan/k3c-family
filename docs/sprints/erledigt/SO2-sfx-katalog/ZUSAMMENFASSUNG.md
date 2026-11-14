# SO2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** SFX-Katalog und Einbau
- **Projekt:** SND

## Ziel

Jedes wichtige Ereignis hat einen Sound mit Quelle und Lizenz, und die Sounds sind eingebaut. Am Ende sichtbar: `docs/assets/sounds.md`, Dateien mit Credits unter `public/audio/`, hörbare Effekte am TV.

## Ergebnis

2026-10-24 (SO2.5, Ralf am PC): AC-03 abgenommen: die fünf Effekte lösen ihren Sound aus, Lautheit passt, nichts erschreckend. SO2.1 bis SO2.4 früher erledigt. Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `docs/assets/sounds.md` enthält jedes Ereignis der Liste mit Sound, Quelle, Lizenz oder al … |
| AC-02 | geprüft | Jede Sound-Datei unter `public/audio/` hat einen Credit-Eintrag (B-167/AC-02). |
| AC-03 | geprüft | Münze aufheben, Schlag, Gegner-Tod, Bauen fertig und Nacht naht lösen am TV je ihren Sound … |
| AC-04 | geprüft | Das Mapping Ereignis → Sound ändert keinen Spielzustand (B-167/AC-04). |
| AC-05 | geprüft | Eine fehlende Sound-Datei lässt das Spiel ohne Fehlermeldung weiterlaufen (B-167/AC-05). |
| AC-06 | – | `task check` ist grün. |
| AC-07 | geprüft | Nacht leise: Nachts spielen die Effekte gedämpft (Q15) (B-167/AC-06). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-167 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-07 geprüft | Jedes wichtige Ereignis hat einen Sound mit Quelle und Lizenz |
| B-011 | erledigt | – | Spiel hat Sound und Musik |
