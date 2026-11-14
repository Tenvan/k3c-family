# WZ19 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Chat im Studio mit Claude Code CLI und Welt per Prompt füllen
- **Projekt:** WZG

## Ziel

Ein allgemeiner Chat rechts im Asset-Studio (assistant-ui) spricht über die Claude Code CLI mit einem Modell, speichert den Verlauf eine Woche in `.chats`, und der Button „Prompt-Grundgerüst“ füllt eine Welt: Die Antwort wird geprüft und als Entwurf geladen (`B-488 › Ziel`, Teil Chat und Prompt). **Am Ende sichtbar:** im Studio eine per Chat gefüllte Welt mit mindestens zwei Inseln als Entwurf; in …

## Ergebnis

- 2026-10-20 Review WZ19.4: AC-01, AC-02, AC-03 und AC-05 geprüft (Nachweise in WZ19.1–WZ19.3 und WZ19.4 › Ergebnis), `task check:all` und `task check:go` grün. - Getrennter Reviewer: keine schweren Befunde, Sicherheit der CLI-Anbindung bestätigt; neues Ticket B-514 (Nachbesserungen, nicht schwer). - 2026-10-20 Abnahme WZ19.5: AC-04 abgenommen von 🧑 am PC (Quelle Chat), per Chat gefüllte Welt als Entwurf; neue Tickets B-515 (Anmeldefehler doppelt, geerbte Umgebung) und B-516 (Prompt-Grundgerüst als Instruktion im Code). - Version: v0.22.0 vorgeschlagen (Minor: neues Chat- und Prompt-Werkzeug i …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-488/AC-06` Go-Test mit Test-Ersatz für die CLI: Prompt senden, Antwort fortlaufend, Zei … |
| AC-02 | geprüft | `B-488/AC-07` Chat-Verlauf in `.chats` (von git ignoriert); Einträge älter als eine Woche … |
| AC-03 | geprüft | `B-488/AC-08` Prompt-Grundgerüst aus den Daten, Antwort geprüft und als Entwurf geladen, F … |
| AC-04 | offen | `B-488/AC-09` (Teil Chat) 🧑 nimmt eine per Chat gefüllte Welt mit zwei Inseln am PC als En … |
| AC-05 | geprüft | `B-488/AC-10` Das Review weist jedes Kriterium nach; neue Abhängigkeit belegt und freigege … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-488 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 offen, AC-05 geprüft | Welten und Inseln im Asset-Studio anlegen und bearbeiten, Welt per Prompt füllen |
