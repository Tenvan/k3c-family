# SO1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Audio-Kern
- **Projekt:** SND

## Ziel

Ein Audio-Kern mit Mixer, Entsperren per Geste und Lautstärke je Gerät steht bereit, an dem Effekte (SO2) und Musik (SO4) hängen. Am Ende sichtbar: ein Demo-Ton auf Ereignis am TV, getrennt regelbare Lautstärken, Verhalten im Split-Screen.

## Ergebnis

2026-10-04, Agent (Claude Opus 5.5) in SO1.4, leichtes Review. AC-01, AC-02: Ergebnis SO1.1; AC-03, AC-05: Ergebnis SO1.2; AC-06 bis AC-08: Ergebnis SO1.3 (Tests, kein Browser-Pane). AC-04: angenommen, Validierung offen (SO1.5, Hörprobe am TV); ebenso die Browser-Pane-Beobachtung zu AC-03 und AC-07 (SO1.2, SO1.3: keine Freigabe durch 🧑). Befunde: keine schweren (kein Audio vor der ersten Eingabe, `localStorage` und `AudioContext` mit try/catch, kein `Math.random()`, Töne selbst erzeugt: kein Credit nötig). Neue Tickets: keine. B-011 bleibt eingeplant (SO2, SO4). `task check:go`: `TestSavesDele …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Der Mixer hat getrennte Busse Musik, Effekte und Ambient mit eigener Lautstärke (B-011/AC- … |
| AC-02 | – | Die Lautstärken werden je Gerät im `localStorage` gespeichert und beim Start gelesen (Test … |
| AC-03 | – | Vor der ersten Eingabe läuft das Spiel ohne Audio-Fehler, nach der ersten Taste startet de … |
| AC-04 | – | Das Audio-Format entspricht dem Ergebnis aus B-166, mit Fallback (Beobachtung am TV). |
| AC-05 | – | Die Sounds liegen in einem Sound-Atlas, die Zahl der Audio-Requests beim Start ist dokumen … |
| AC-06 | geprüft | Im Split-Screen mit 2 Spielern hört jeder seinen Bereich, Warnungen sind global (Test auf … |
| AC-07 | geprüft | Mindestens ein Demo-Ereignis löst einen Ton aus (B-011/AC-01, erster Schritt). |
| AC-08 | geprüft | `task check` ist grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-011 | erledigt | AC-01 –, AC-03 –, AC-07 geprüft | Spiel hat Sound und Musik |
