# B-123 · Das Protokoll kennt Schlag, Skills, Pool, Berufe und die gültigen Aktionen je Spieler

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** S2
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S2

## Ausgangslage

Protokoll v2 kennt `moveX`, `sprint` und `pay` je Slot; keine Skills, keinen Schlag, keinen Pool, keine Berufe, keine Aktionsliste (`docs/protocol.md`).

## Ziel

Eingaben für Schlag und Skills, Zustand der Skills und des Pools je Spieler, Berufe und die **gültigen Aktionen je Spieler** (für das Aktionen-Overlay) sind im Protokoll. Nutzen: Client kann Skill-Menü, Slots und Overlay bauen (`docs/rules/monarch.md` § 4).

## Beteiligte und Zielgruppen

Entwickler (Client und Server); eigene Session laut Arbeitsweise › Protokoll.

## Anforderungen

- Eingabefelder für Schlag und Skill-Slots 1–4 (c2s `input`), Aktionen Skill verteilen, Respec, Beruf ausbilden, Tauschen (c2s).
- Zustand: Skill-Pool, Verteilung und Abklingzeiten je Spieler (s2c), Berufe der Bürger, Grabstein/Wiederbeleben.
- Gültige Aktionen je Spieler am aktuellen Ort (Taste, Aktion, Ziel) als s2c-Daten für das Overlay.
- Version erhöhen, Testdaten in `testdata/protocol/`, serverseitige Prüfung aller Eingaben.

## Nicht-Ziele

Darstellung (B-124 bis B-126), Simulation (B-118 bis B-122).

## Regeln und Einschränkungen

`docs/protocol.md` ist die Quelle; nur Protokoll und beide Enden in einer Session. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

`snap` nennt je Spieler `actions:[{key:"A",hold:true,label:"revive",target:12}]`.

## Ausnahme- und Fehlerfälle

Ungültiger Skill-Slot oder Beruf → `bad_request`.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die neuen Felder; `testdata/protocol/` hat Beispiele; beide Enden parsen sie (Tests).
- **AC-02** Der Server prüft Eingaben (Slot, Beruf, Tausch) und lehnt ungültige ab (Test).
- **AC-03** Protokollversion erhöht, ältere Clients erhalten `version`.

## Offene Fragen

Entschieden mit der Freigabe von S2 (2026-10-03): nur der Ort des Spielers.

## Notizen

Aus R3.2 und R3.3. Zusammen mit B-104 (Stufe je Spieler) planen.

Abschluss S2 (Review S2.5, 2026-10-05): umgesetzt sind Schlag, Skills, Pool, `learn`, `respec` und die Aktionsliste. Beruf ausbilden, Tauschen, Berufe der Bürger und Grabstein/Wiederbeleben (Rest von AC-02) folgen mit B-283.
