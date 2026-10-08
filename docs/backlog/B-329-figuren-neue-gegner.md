# B-329 · Die sechs neuen Gegner zeigen eigene Figuren statt Platzhalter

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GRA
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

K1.3 hat sechs neue Gegner angelegt (`lavaSlime`, `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling`, `crystalGuardian`). Nach B-326 zeigen sie vorhandene Sheets mit Tönung als Platzhalter (`data/sprites.json`, Vermerk in `docs/assets/zuordnung-objekte.md`): Lavaschleim und Splitterwicht `mushroom`, Eisenkäfer und Kristallspinne `hell-hound`, Feuergeist `ghost`, Kristallwächter `hell-gato`. Im Spiel sehen sie wie Rattenschwarm, Wolf, Minengeist und Höhlentroll aus.

## Ziel

Jeder der sechs Gegner ist im Spiel an einer eigenen Figur erkennbar.

## Beteiligte und Zielgruppen

Spieler (Erkennbarkeit), Entwicklung (CLI); 🧑 wählt die Figuren.

## Anforderungen

- Je Gegner eine eigene Figur unter `public/sprites/` mit Credit, Eintrag in `data/sprites.json` und Zeile in `docs/assets/zuordnung-objekte.md` ohne Platzhalter-Vermerk.

## Nicht-Ziele

Suche nach Kandidaten selbst (Verfahren B-193), Namen der Gegner, Werte (BR2).

## Regeln und Einschränkungen

Verfahren und Lizenzregel wie B-193 (Q14, Q70), Grundstil Q13.

## Beispiele

Kristallwächter: 🧑 wählt eine Figur → Ordner unter `public/sprites/`, `data/sprites.json › enemies.crystalGuardian` zeigt darauf, Zeile in der Zuordnung ohne „Platzhalter“.

## Ausnahme- und Fehlerfälle

Keine passende Figur gefunden → Platzhalter bleibt, Zeile nennt dieses Ticket.

## Akzeptanzkriterien

- **AC-01** Keine Zeile in `docs/assets/zuordnung-objekte.md` enthält „Platzhalter (B-326)“; `task check` grün.

## Offene Fragen

Welche Figuren? 🧑

## Notizen

Herkunft: Entscheidung zu B-326 (2026-10-06).
