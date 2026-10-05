# B-122 · Elite-Upgrades, Rüstung, Truppen-Limit je Hub und Heilung der Truppen sind umgesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W4
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-05, Chat, durch 🧑, Revision 2, mit Sprint W4

## Ausgangslage

Krieger und Elite-Truppen gibt es nur als Daten; es gibt kein Truppen-Limit, keine Heilung außer Respawn und kein Verlust-Ereignis (`docs/rules/archiv/ist-monarch-buerger.md` § 3).

## Ziel

Schmiede liefert Elite-Upgrades, Rüstkammer Rüstung für alle Truppen, das Truppen-Limit (Basis 10, Kaserne +10, nur Kämpfer) gilt je Hub, der Heilplatz heilt Truppen und Spieler, Verluste werden als Ereignis gemeldet. Nutzen: Truppenaufbau mit Grenzen und Tiefe (`docs/rules/buerger.md` §§ 2–3).

## Beteiligte und Zielgruppen

Spieler; Messung mit B-099.

## Anforderungen

- Krieger mit Schwert (Werkstatt) und Nahkampf-Verhalten (B-014): Posten hinter der **äußersten gebauten Sperre** – mit Tor zwischen Mauer und Tor, ohne Tor hinter der Mauer; Posten sind keine Bauplätze, innere Linien bekommen keine Krieger (Beschlüsse Q46, Q54, 2026-10-04; ändert Q38); Reichweite 1; Seitenverteilung wie bei Bogenschützen (`makeArcher`) als Startwert (Q38). Schwert-Zahlziel als Anhang an der Werkstatt, `dx +4` (Beschluss Q53, 2026-10-04). Elite-Bogenschütze (100 Stein + 50 Gold) und Elite-Krieger (100 Kupfer + 50 Gold) in der Schmiede.
- Auswahl am Platz: je Angebot ein eigenes Zahlziel als Anhang mit festem `dx` am Gebäude (keine neue Taste); aufgewertet wird der nächste Bogenschütze bzw. Krieger (Beschlüsse Q34, Q52, 2026-10-04).
- Rüstungs-Upgrade in der Rüstkammer: 100 Eisen + 100 Gold je Stufe, **2 Stufen** (ab Hub-Stufe 4 und 5), +20 % / +40 % auf die Basis-HP; wirkt sofort auf alle Kämpfer (`MaxHP` und `HP` steigen um denselben Betrag, keine Vollheilung); Bauern und Landstreicher bleiben ohne Rüstung (Beschluss Q37, 2026-10-04).
- Truppen-Limit je Hub: Basis 10, Kaserne +10, nur Kämpfer zählen. W3 baut es für Bogenschützen, dieses Ticket erweitert es um Krieger und Elite; geprüft an genau einer Stelle beim Waffe-Holen, auch für Schwerter (Beschlüsse Q29 und Q40, 2026-10-04).
- Heilung am Heilplatz (Truppen und Spieler in Reichweite) und durch den Heiler-Skill (B-119); keine Regeneration. Der Heilplatz entsteht vollständig in W3 (B-116); hier Nachweis per Regressionstest (Beschluss Q40, 2026-10-04).
- Verlust-Kaskade statt Tod und Ereignis `disarmed` (Messgröße für B-099): Bürger sterben nicht, Ausrüstung fällt zu Boden, der Bürger wird Bauer, ein Bauer wird Landstreicher; `disarmed` mit `kind`, `x`, `cause`; ohne Priorität; kein Ereignis beim Burgfall (Beschlüsse Q67, Q68, Q69, 2026-10-04; ersetzen Q39).
- Umsetzung in zwei Sessions: W4.3a (Krieger, Schwert-Regal, Limit-Erweiterung, Verlust-Kaskade) und W4.3b (Elite, Rüstung, Heilplatz-Nachweis, Golden); Kriterien unverändert (Beschluss Q41, 2026-10-04).

## Nicht-Ziele

Berufe und Händler (B-121), Gegner-Werte (Regelwerk III), Darstellung (B-126).

## Regeln und Einschränkungen

`docs/rules/buerger.md`; Werte nur in `data/`; Verhalten wie heute (Bauern fliehen, Kämpfer halten Posten). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Ein Hub mit 10 Kämpfern und Kaserne: die 11. Waffe kann bis zum Limit 20 weiter ausgegeben werden; ab 20 wird der Bauer nicht mehr zum Kämpfer.

## Ausnahme- und Fehlerfälle

Limit erreicht → Hinweis (B-126), keine Verschwendung von Material (Bogen bleibt im Regal).

## Akzeptanzkriterien

- **AC-01** Test: Elite-Upgrade und Rüstung wirken mit Kosten und Werten aus den Daten.
- **AC-02** Test: Limit je Hub (Basis, Kaserne), nur Kämpfer zählen.
- **AC-03** Test: Heilplatz heilt in Reichweite, ohne Heilplatz keine Heilung.
- **AC-04** Test: Verlust-Kaskade und `disarmed` bei Verlust; Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

keine

## Notizen

Aus R3.3. Verwandt mit B-014 (Krieger, Elite) und B-116 (Gebäude-Wirkungen).
