# BS12 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Mauern und Türme verteilt, Tor ist die äußerste Mauer
- **Projekt:** BAS

## Ziel

Verteilte Mauer- und Turm-Plätze je Chunk (1–3, per Seed, Türme zum Teil nah an Mauern), Tor = äußerste gebaute Mauer, Bauplatz erst nach Freiraum von Bäumen und Felsen bezahlbar; Bots und Client laufen damit, der Meilenstein-Lauf `coop2` läuft durch (Ergebnis offen).

## Ergebnis

2026-10-28 (Claude, autonom): AC-01 bis AC-05 nachgewiesen (BS12.1–BS12.6, Ergebnisse dort), Review ohne schwere Befunde (BS12.5). Messung coop2, 4 Seeds: Titan 2/4, Lava-Golem 3/4, Endboss 0/4 (Meilenstein 1 bis Titan, BS3). Optik und Bedienung der neuen Plätze im Browser nimmt 🧑 ab (zusammen mit BS13).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | B-652/AC-01, AC-02 (kein Tor mehr, Tor-Regel). |
| AC-02 | geprüft | B-652/AC-03, AC-04 (Plätze je Chunk, Türme nah). |
| AC-03 | geprüft | B-652/AC-05 (Freiraum). |
| AC-04 | – | B-652/AC-06 (Bots, Meilenstein-Lauf läuft durch). |
| AC-05 | – | B-652/AC-07 (Regeln, Glossar, Protokoll, check:diff grün). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
