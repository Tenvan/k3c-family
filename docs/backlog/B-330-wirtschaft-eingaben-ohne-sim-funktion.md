# B-330 · Für Hub-Ausbau, Tausch und Berufswahl ist entschieden, ob es eigene Eingaben gibt

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** W5
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W5.2 soll die Nachrichten `upgradeHub`, `trade` und `setProfession` anlegen, im Server prüfen und Fehler mit `bad_request` ablehnen (B-153/AC-02, B-283/AC-02). Die Sim kennt diese Aktionen aber nur als ortsgebundenes Bezahlen über `input.pay` (A halten): `payUpgrade` (`engine/sim/hub_level.go`), `payMerchant` (`engine/sim/merchant.go`), `payAnyOffer`/`payOffer` (`engine/sim/upgrades.go`, `engine/sim/professions.go`), alle unexportiert. Ohne Material wartet der Hub-Ausbau (`waitingMaterial`, Spec W1) statt abgelehnt zu werden. Der Server hat damit nichts aufzurufen und keinen Fehler, den er melden könnte; W5.2 ist blockiert.

## Ziel

🧑 entscheidet, ob die drei Aktionen eigene Eingaben bekommen oder Bezahlen am Ort bleiben, damit W5.2 Version und Bytes abschließen kann.

## Beteiligte und Zielgruppen

🧑 entscheidet; Entwickler SIM (falls Variante A) und SRV (W5.2).

## Anforderungen

- Variante A: SIM exportiert je Aktion eine Funktion mit Fehlerrückgabe (z. B. `UpgradeHub(w, p) error`, `Trade(w, p, side) error`, `SetProfession(w, p, troopID, profession) error`, Muster `LearnSkill`), Fehler für „kein Material“, „kein Händler/außer Reichweite“, „unbekannter Beruf“. Danach baut W5.2 die Nachrichten. Offen dann: Verhältnis zum Warten ohne Material (W1).
- Variante B (Vorschlag dieser Session): Die drei bleiben Bezahlen am Ort über `input.pay`, keine neuen Nachrichten. Die Fehlerfälle „Tausch ohne Händler“, „Hub-Ausbau ohne Material“, „unbekannter Beruf“ werden aus W5 (README › Ausnahme- und Fehlerfälle, AC-02) und B-153/B-283 gestrichen; W5.2 erhöht nur die Version (wegen der Felder aus W5.1) und misst die Bytes. Begründung: kein zweiter Eingabeweg für dieselbe Aktion, die Sim prüft am Ort bereits selbst.

## Nicht-Ziele

Umsetzung der Variante (folgt im Sprint, den 🧑 bestimmt); Darstellung (W6).

## Regeln und Einschränkungen

Server rechnet keine Regeln nach, nur die Sim (`docs/decisions/001`); Protokolländerung nur in einer Protokoll-Session (`docs/arbeitsweise.md` › Domänen); W5 bleibt in SRV, Funktionen in `engine/sim/` gehören zu SIM.

## Beispiele

Variante B: Spieler hält A am Händler → wie heute `traded`-Ereignis; ohne Händler → nichts passiert, keine Fehlermeldung.

## Ausnahme- und Fehlerfälle

nicht relevant: Frage-Ticket, die Fehlerfälle sind Teil der Entscheidung.

## Akzeptanzkriterien

- **AC-01** Variante A oder B ist in diesem Ticket mit Datum durch 🧑 festgehalten, und W5 (README, W5.2) sowie B-153/B-283 sind entsprechend angepasst.

## Offene Fragen

Variante A oder B (🧑). Bei A: Sprint und Reihenfolge des SIM-Teils vor W5.2.

## Notizen

Gefunden in W5.2 (Stand `c07ce5d4`). Version 4 auf beiden Enden; Bytes vorher aus W5.1: `stateB/tick` 13068, p99 18063.
