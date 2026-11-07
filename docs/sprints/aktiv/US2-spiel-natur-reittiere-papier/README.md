# US2 · CLI · Spiel lädt Natur, Objekte, Münzen, Icons und Reittiere als Papierschnitt

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** bb688941
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-07, autonom, Claude

## Ausgangslage

Nach US1 lädt das Spiel Figuren, Gebäude und Hintergründe als Papierschnitt; Natur, Objekte, Münzen und Icons blieben bis MG8 alte Pixel-Grafik (B-701 D-US1-04). MG8 liefert `public/grafik/objekte/` und `public/grafik/icons/`. Der berittene Monarch zeigt immer das Pferd (D-US1-02).

## Ziel

Das Spiel lädt Natur, Objekte, Münzen und Icons aus den MG8-Gruppen; der berittene Monarch nutzt das Reittier aus `data/monarch.json` (Zuordnung nach `reittiere/*`), wenn das Monarch-Asset eine Animation dafür hat, sonst das Pferd.

## Beteiligte und Zielgruppen

Spieler sehen die Welt einheitlich im Papierschnitt; Claude setzt um; 🧑 prüft in PS1.

## Anforderungen

Entscheidung 005; B-701 D-US1-02, D-US1-04.

## Nicht-Ziele

Atlas und alte Packs entfernen (CU1); Monarch-Spec ändern (MG1); Xbox-Skalierung.

## Regeln und Einschränkungen

Zuordnung und Frame-Maße in `data/paper.json`, ein Test prüft sie gegen die Manifeste. Papier 1:1 (32 Einheiten je Kachel).

## Beispiele

- Wald-Knoten Baum → `objekte/tanne` (animiert, 8 Frames).
- Ader Kupfer → `objekte/ader-kupfer`.
- `data/monarch.json` Reittier `horse` → `reittiere/pferd` → Monarch-Animation `schritt`.

## Ausnahme- und Fehlerfälle

- Reittier ohne Monarch-Animation → Pferd, eine Warnung im Client-Log.
- Objekt ohne Papier-Asset (z. B. Gerüst) → alte Grafik bis CU1, als D-Zeile.

## Akzeptanzkriterien

- **AC-01** Kein Lade-Pfad für Natur, Objekte, Münzen und Icons zeigt auf alte Packs oder `gebaeude/*-32.png`, außer den in B-701 genannten Lücken (Test).
- **AC-02** Das Reittier aus `data/monarch.json` wählt die Monarch-Animation für `reittiere/<id>`; fehlt sie, das Pferd (Test).
- **AC-03** `task check:diff` grün, Review ohne offene wichtige Befunde.

## Offene Fragen

keine (Defaults in B-701 als D-US2-xx)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| US2.1 | `US2.1-natur-objekte-icons.md` | Umsetzung | autonom | offen |
| US2.2 | `US2.2-reittier.md` | Umsetzung | autonom | offen |
| US2.3 | `US2.3-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
