# GR2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Grafik-Suche für Lücken
- **Projekt:** –

## Ziel

Zu jeder Lücke gibt es Kandidaten, 🧑 hat gewählt, die Assets liegen mit Credits im Repo. Am Ende sichtbar: Kandidatenseite unter `docs/funde/`, neue Packs unter `public/grafik/`, Tabelle ohne offene Lücke außer „kein Treffer“.

## Ergebnis

2026-10-04, Agent (Claude Opus) in GR2.4, leichtes Review. AC-01 bis AC-04: Ergebnisse GR2.1 bis GR2.3 (Teil 1 schon mit #126 auf `develop`; AC-02 Workshop mit 🧑); AC-06: Diff `origin/develop...origin/sprint/gr2` geprüft (20 Packs, 93 PNG, je `LICENSE.txt`, Credit und `lizenzen.html`-Eintrag, nur CC0/CC BY, Gruppe `kandidaten`, in keiner Zuordnungsdatei, Test ergänzt statt gelockert); AC-05: `task check` und `task check:go` grün. Schwere Befunde: keine. Offen für 🧑: Ansicht `grafiken.html` im Browser (GR2.3 Schritt 5). B-162 archiviert. Version: v0.6.1 vorgeschlagen (Patch: nur Bild-Assets, Cr …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Zu jeder Lücke liegt eine Kandidatenliste mit drei Einträgen vor, weniger nur mit Vermerk, … |
| AC-02 | – | 🧑 hat je Lücke gewählt oder „kein Treffer“ bestätigt (B-162/AC-02). |
| AC-03 | – | Gewählte Assets liegen unter `public/grafik/` mit Lizenzdatei, Index- und Credit-Eintrag, … |
| AC-04 | – | Die Zuordnungstabelle zeigt für jede entschiedene Lücke den neuen Status (B-162/AC-04). |
| AC-05 | – | `task check` ist grün. |
| AC-06 | – | Nicht gewählte Kandidaten mit CC0 oder CC-BY liegen ebenfalls unter `public/grafik/` (Grup … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-162 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Für die Grafik-Lücken liegen Kandidaten mit Vorschau, Lizenz und Stilbewertung v … |
