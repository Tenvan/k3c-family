# GEB17 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Hub-Ausbau: Angebot vor der Burg, Ausbaustufen je Inselstufe, unterscheidbare Wachstumsreihe
- **Projekt:** GEB

## Ziel

Der Hub wächst sichtbar und spielbar: Vor der Burg steht das Ausbau-Angebot, jede Inselstufe hat ihre Ausbauschritte mit Material-Tor zur nächsten Inselstufe, und jeder Schritt hat eine eigene Grafik. Am Ende sichtbar: Spieler steht vor der Burg → „Ausbauen“ mit Kosten; nach dem Ausbau eine sichtbar größere, andere Burg; der letzte Schritt einer Inselstufe wartet auf „Material der nächsten Insel“.

## Ergebnis

2026-10-25, Claude (GEB17.7): `task check`, `task check:go` und `check:studio` grün; `check:race` lokal übersprungen (kein C-Compiler). AC-01: GEB17.1 (Angebot vor der Burg). AC-02: GEB17.3, vorentschieden über B-623 (KRT). AC-03: GEB17.4 (hubStep, Material-Tor; Review Opus APPROVE, kein Deadlock). AC-04: GEB17.5. AC-05: GEB17.6 (15 Ansichten). Gestapelt auf KB1 (#481), Golden unverändert. 2026-10-31: AC-06 von Ralf (🧑) am Gerät abgenommen, im Chat bestätigt; Claude hat die Prüfung nicht selbst gesehen. Offen: Bestätigung der Stufenfolge (AC-02) über B-623, deshalb bleibt der Sprint aktiv. 202 …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Vor der Burg erscheint das Ausbau-Angebot (Test der Anzeige-Regel und Beobachtung am Gerät … |
| AC-02 | geprüft | Regeldatei `docs/rules/stufen.md` beschreibt die Stufenfolge mit den Schritten je Inselstu … |
| AC-03 | geprüft | Daten, Simulation und Protokoll setzen sie um (Go-Tests, Golden mit Begründung) (B-592/AC- … |
| AC-04 | geprüft | Der Client zeigt Schritt und Wartegrund (B-592/AC-03). |
| AC-05 | geprüft | Jeder Schritt der Reihe hat eine eigene, unterscheidbare Grafik in der Studio-Vorschau (B- … |
| AC-06 | – | 🧑 sieht im Spiel Angebot und Reihe als echtes Wachstum (B-590/AC-02, B-591/AC-01 am Gerät) … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-591 | erledigt | AC-01 geprüft, AC-06 – | Vor der Burg erscheint kein Ausbau-Angebot, wenn der Spieler davor steht |
| B-592 | erledigt | AC-02 geprüft, AC-03 geprüft, AC-04 geprüft | Der Hub hat zwei bis drei Ausbaustufen je Inselstufe, weiter mit dem Material de … |
| B-590 | erledigt | AC-05 geprüft, AC-06 – | Die Wachstumsreihe der Burg zeigt echte Unterschiede statt derselben Burg in Far … |
