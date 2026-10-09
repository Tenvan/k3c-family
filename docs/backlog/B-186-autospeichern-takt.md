# B-186 · Der Server speichert alle 60 s und bei Tagesanbruch, das HUD zeigt „gesichert“

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** ST1
- **Projekt:** LST
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beschluss Q10 (`docs/fragenkatalog.md` › Beschlüsse) verlangt: Speichern beim Verlassen jedes Geräts **und alle 60 s**, zusätzlich bei Tagesanbruch und Stufenwechsel; das HUD zeigt „gesichert“. S2 (B-147) setzt nur das Speichern beim Verlassen um; 🧑 hat bei der Freigabe von S2 am 2026-10-03 entschieden, den Rest als eigenes Ticket zu führen.

## Ziel

Ein Abbruch mitten in der Nacht oder ein Absturz des Servers kostet höchstens 60 s Spiel, und die Spieler sehen, dass gespeichert wurde.

## Beteiligte und Zielgruppen

Spieler (Kinder brechen mitten in der Nacht ab), Betreiber des Pi, 🧑 (Takt bestätigt in Q10).

## Anforderungen

- Der Raum speichert alle 60 s Spielzeit sowie bei Tagesanbruch und Stufenwechsel, solange mindestens ein Gerät verbunden ist.
- Speichern blockiert den Tick nicht spürbar (Ziel aus B-147: unter einem Tick, 33 ms, sonst Warnung im Log).
- Das Protokoll meldet einen gelungenen Speichervorgang; der Client zeigt kurz „gesichert“ (CLI, eigene Session oder Folge-Ticket).
- Deterministisch: Speichern ändert den Spielverlauf und den Golden-Hash nicht.

## Nicht-Ziele

Speichern beim Verlassen (B-147, S2), Backup und Rotation (B-142), Spielstand-Liste im Client.

## Regeln und Einschränkungen

Domäne SRV; die HUD-Anzeige gehört zu CLI. Protokolländerung nur in einer eigenen Session, beide Enden gemeinsam (`docs/arbeitsweise.md` › Protokoll). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

2 Spieler spielen 5 min in der Nacht, der Server stürzt ab → der geladene Stand ist höchstens 60 s alt. Tagesanbruch → Spielstand geschrieben, HUD zeigt kurz „gesichert“.

## Ausnahme- und Fehlerfälle

Schreibfehler → Log, vorheriger Stand bleibt, kein „gesichert“; Raum ohne Geräte → kein Takt.

## Akzeptanzkriterien

- **AC-01** Ein Go-Test zeigt: Nach 60 s Spielzeit, bei Tagesanbruch und bei Stufenwechsel liegt ein neuer Spielstand in `saves/`.
- **AC-02** Der Golden-Hash eines Laufs mit und ohne Takt ist gleich; `task check:go` ist grün.
- **AC-03** Nach einem Speichervorgang zeigt das HUD am TV kurz „gesichert“ (Abnahme 🧑, Hardware-Bahn).

## Offene Fragen

Takt in Spielzeit oder Wanduhr (Zeitraffer im Dev-Raum): Vorschlag Spielzeit, 🧑 bestätigt mit der Freigabe.

## Notizen

Entstanden bei der Freigabe von S2 (2026-10-03); Q10, B-147.
