# B-755 · Skill-Leiste ohne Tastentexte, Skillmenü zeigt die Taste vor dem Skill und ist größer

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Skill-Leiste unten links im HUD nennt je Slot die Taste ("Q Fireball bereit · R – · …", `src/scenes/hud/skillMenuView.ts`). Das Skillmenü zeigt keine Slot-Tasten, nur Skills mit Zustand und eine Bedienzeile; seine Schrift ist 28 px.

## Ziel

Das HUD nennt keine Tasten mehr; die Tasten stehen im Skillmenü vor dem jeweiligen Skill, und das Menü ist besser lesbar.

## Beteiligte und Zielgruppen

Spieler an Tastatur, Controller und Touch; Entwickler (🧑, Abnahme am Gerät).

## Anforderungen

- Skill-Leiste: je Slot nur Skill und Abklingzeit bzw. –, ohne Taste.
- Skillmenü: belegt ein Skill einen Slot, steht die Taste des Geräts davor (Taste, dann Skillname), z. B. "[Q] Feuer I · Fireball · gelernt".
- Menü moderat größer (Schrift, Rand); passt der Text nicht in das Spielerfeld, schrumpft die Schrift schrittweise bis zur Mindestgröße 28 px (Q03).
- Tasten aus `slotBindings`/`glyphs`, nicht doppelt hartkodiert.

## Nicht-Ziele

Kein Umbau der Menü-Bedienung, keine neuen Skills.

## Regeln und Einschränkungen

Mindestschrift Q03; 2 Spieler gleichzeitig; Texte nur in `texts.*.ts`; Abnahme der Anzeige am Gerät beim Nutzer.

## Beispiele

Controller-Spieler: Leiste "Fireball bereit · – · – · –"; Menü-Zeile "▶ [LB] Feuer I · Fireball · gelernt".

## Ausnahme- und Fehlerfälle

Skill ohne Slot: Zeile ohne Taste. Schmales Feld (Viertel): kleinere Schrift, nie unter 28 px.

## Akzeptanzkriterien

- **AC-01** Die Skill-Leiste enthält keinen Tastennamen (Test).
- **AC-02** Im Menü steht die Slot-Taste des Geräts vor dem Skillnamen belegter Skills (Test).
- **AC-03** Menü-Schrift im vollen Feld größer als 28 px, im Viertel nie darunter, Text bleibt im Feld (Prüfung im Browser).
- **AC-04** Abnahme am Gerät durch 🧑.

## Offene Fragen

keine

## Notizen

Auftrag aus der Schaltzentrale; Folge von B-741.
