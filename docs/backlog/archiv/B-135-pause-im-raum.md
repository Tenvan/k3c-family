# B-135 · Pause im gemeinsamen Raum ist als Regel festgelegt

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** F1
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F1

## Ausgangslage

Die Aktion `pause` ist definiert (Menu bzw. Esc, `src/input/playerInput.ts`, `docs/game-design.md` › Steuerung), hat aber keine Funktion und keine Szene. Der Server pausiert einen Raum nur, wenn kein Gerät mehr verbunden ist (`engine/room/actions.go` › `afterDisconnect`). Ob ein Spieler allein pausieren darf und ob dann der ganze Raum anhält, ist nicht geregelt.

## Ziel

Eine Regel sagt, wer pausiert, was dabei anhält und wer fortsetzt. Nutzen: Die Pause-Szene (B-146) und das Protokoll können ohne Rückfrage gebaut werden.

## Beteiligte und Zielgruppen

Spieler am TV (1–4 Monarchen, oft Kinder); 🧑 entscheidet (Beschluss Q01).

## Anforderungen

- Die Regel beantwortet: Hält die Simulation des Raums an oder nur die Anzeige des pausierenden Geräts? Wer darf pausieren (jeder oder nur der erste Spieler)? Wer darf fortsetzen? Wie lange darf die Pause höchstens dauern (Zahl in Sekunden oder „unbegrenzt“)?
- Die Regel gilt für 2 bis 4 Spieler im Split-Screen und für mehrere Geräte online.
- Sie berücksichtigt, dass Menu für „Pause“ belegt ist und View + Menu gemeinsam „zurück zur Landingpage“ bedeutet (reserviert, `CLAUDE.md`).

## Nicht-Ziele

Die Pause-Szene (B-146), das Protokoll dafür, Speichern beim Verlassen (B-147).

## Regeln und Einschränkungen

Domäne REG. Der Server ist die einzige Engine (Entscheidung 001); eine raumweite Pause wäre dort umzusetzen. B bleibt unbelegt, View + Menu reserviert.

## Beispiele

Variante raumweit: Spieler 2 drückt Menu → der Raum hält an, alle Geräte zeigen „Pause von Spieler 2“, nur Spieler 2 (oder jeder) setzt fort.

## Ausnahme- und Fehlerfälle

Der pausierende Spieler verliert die Verbindung → die Regel nennt, wer dann fortsetzen darf (Verbindung: B-144).

## Akzeptanzkriterien

- **AC-01** Abschnitt „Pause“ in `docs/rules/bedienung.md` (neue Datei) beantwortet die vier Fragen aus den Anforderungen mit einer eindeutigen Aussage und, wo gefragt, einer Zahl (Sichtprüfung).
- **AC-02** Die Zeile „Pause“ in der Steuerungstabelle von `docs/game-design.md` verweist auf diesen Abschnitt; `task check` grün.

## Offene Fragen

Pause lokal oder raumweit, wer darf pausieren und fortsetzen, Höchstdauer: `docs/fragenkatalog.md` Q01, entscheidet 🧑.

## Notizen

Aus Plan Lücke 2 und Entscheidung 1 der Plan-Liste. Umsetzung danach in B-146 (S5).
