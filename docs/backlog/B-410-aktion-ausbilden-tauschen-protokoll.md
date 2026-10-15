# B-410 · Aktionen-Overlay zeigt Ausbilden und Tauschen nicht, der Server liefert sie nicht in actions

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** W11
- **Projekt:** WRT
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

W6.3 (B-126/AC-02) soll Ausbilden und Tauschen im Aktionen-Overlay zeigen. `PlayerAction` kennt nur `attack`, `skill`, `learn`, `respec`; `docs/protocol.md` hat keine Aktion für Ausbilden oder Tauschen (nur das Zahlziel am Händler über `buyPaid`). Der Client erfindet nichts, die Einträge sind in W6.3 `verschoben`.

## Ziel

Der Server nennt Ausbilden (Beruf) und Tauschen (Händler) als Einträge in `actions`; danach zeigt das Aktionen-Overlay sie (`actionHints.ts`, CLI).

## Beteiligte und Zielgruppen

Spielende am TV (bis 2 lokal, PC und Xbox) sehen im Aktionen-Overlay, was A halten am Ort bewirkt. Agent in SRV (W11.3) liefert die Einträge, Agent in CLI (W11.4) zeigt sie. 🧑 entscheidet die Offenen Fragen und nimmt am Gerät ab.

## Anforderungen

- `actions` je Spieler enthält `{ "action": "train", "kind": k, "gold": g, "paid": n }`, wenn A halten am Ort dieses Spielers gerade auf ein Berufs-Angebot zahlt (`kind`: `miner`, `builder`, `craftsman`; `gold`: Preis, `paid`: schon bezahlt).
- `actions` enthält `{ "action": "trade", "side": s, "resource": r, "material": m, "gold": g }`, wenn A halten gerade am Händler zahlt (`side`: `buy` oder `sell`; `resource`: sein Material; `material`/`gold`: der Kurs, z. B. 10 Material = 5 Gold).
- Maßgeblich ist genau das Ziel, das A halten jetzt treffen würde (`findPayTarget` der Sim). Steht ein anderes Zahlziel näher oder hat Vorrang (Bauplatz, Schwert), fehlen `train` und `trade`. Höchstens einer der beiden Einträge je Spieler.
- Ein Eintrag erscheint nur, wenn die Zahlung jetzt wirken kann: lebender Spieler; `train` und `trade/buy` nur mit Gold in der Börse, `trade/buy` nicht bei vollem Lager oder voll bezahltem Kauf; `trade/sell` nur mit genug Material im Vorrat.
- Feste Reihenfolge: `attack`, `skill` nach Slot, `learn`, `train`, `trade`.
- Rein lesend: Das Berechnen ändert die Welt nicht (deterministisch, Golden unverändert). Jeder Spieler bekommt seine eigene Liste (2 Spieler an verschiedenen Orten).
- Kosten: kein neuer Bandbreiten-Ausreißer, Budget Q08 (`stages_bench_test.go`) bleibt grün.

## Nicht-Ziele

Anzeige im Aktionen-Overlay (W11.4, CLI); andere Angebote wie Bogen, Schwert, Elite, Rüstung und Bauplätze als Einträge (später, Ticket bei Bedarf); neue Eingaben oder Nachrichten (gezahlt wird weiter per A halten über `input.pay`); Berufswahl per Taste (Glossar › Angebot: keine Auswahl per Taste); `respec` (B-270); Regeländerungen an Preisen und Kurs.

## Regeln und Einschränkungen

Domäne SRV (`engine/net/actions.go`), dazu eine nur lesende Sicht in `engine/sim` (Vorbild `site_view.go`, `economy_view.go`); Schichtgrenzen: `engine/sim` importiert nichts aus `engine/net`. Protokoll additiv, **keine neue Version** (Vorbild W11.2): Ein Client ohne W11.4 ignoriert unbekannte Einträge. `docs/protocol.md` › Skills und Aktionen, Beispiel unter `testdata/protocol/`, Typ `PlayerAction` in `src/model/types.ts`. Datei ≤ 400, Funktion ≤ 60 Zeilen. Kein `math/rand`.

## Beispiele

- Spieler 1 steht mit 12 Gold am Bergmann-Angebot der Werkstatt, 5 von 20 bezahlt → `actions` enthält `{ "action": "train", "kind": "miner", "gold": 20, "paid": 5 }`.
- Händler handelt Holz, Spieler 2 steht am „Verkaufen“, Vorrat 30 Holz → `{ "action": "trade", "side": "sell", "resource": "wood", "material": 10, "gold": 5 }`; Spieler 1 am anderen Ende des Hubs hat keinen `trade`-Eintrag.
- Spieler steht am „Kaufen“ mit 0 Gold → kein `trade`-Eintrag.

## Ausnahme- und Fehlerfälle

Kein Händler da, Angebot voll bezahlt oder ohne Platz (Handwerker), Gebäude nicht gebaut → kein Eintrag. Spieler tot (`RespawnIn` > 0) → kein `train`/`trade`. Älterer Client → ignoriert die Einträge (kein Fehler). Lager voll beim Kaufen, Vorrat zu klein beim Verkaufen → kein Eintrag.

## Akzeptanzkriterien

- **AC-01** Go-Test in `engine/net`: Ein Spieler mit Gold am bezahlbaren Berufs-Angebot hat genau einen `train`-Eintrag mit `kind`, `gold`, `paid`; am Händler je Seite ein `trade`-Eintrag mit `side`, `resource`, `material`, `gold`; ohne Gold, bei vollem Lager, zu kleinem Vorrat, ohne Händler, voll bezahltem oder vollem Angebot fehlt der Eintrag.
- **AC-02** Go-Test: Liegen Bauplatz und Angebot beide in Reichweite, folgt die Liste dem Ziel von A halten (`findPayTarget`); zwei Spieler an verschiedenen Orten bekommen verschiedene Listen; die Berechnung ändert die Welt nicht (Golden und `task check:go` grün).
- **AC-03** `docs/protocol.md` › Skills und Aktionen beschreibt `train` und `trade` samt Reihenfolge, ein Beispiel unter `testdata/protocol/` enthält beide, `PlayerAction` in `src/model/types.ts` kennt sie; `task check` grün.

## Offene Fragen

Vorschläge der Planung, 🧑 bestätigt oder ändert sie mit der Freigabe:
1. Namen `train` und `trade` mit den Feldern oben (statt z. B. `profession`/`merchant`).
2. Nur Beruf und Händler; Bogen, Schwert, Elite, Rüstung und Bauplätze bleiben ohne Eintrag (das Preisschild zeigt sie schon).
3. Eintrag nur, wenn die Zahlung jetzt wirkt (Gold, Lager, Vorrat), statt immer am Ort.
4. Keine neue Protokoll-Version.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
