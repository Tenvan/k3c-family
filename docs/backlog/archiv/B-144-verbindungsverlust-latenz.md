# B-144 · Verbindungsverlust und Eingabe-Latenz haben eine Regel mit Zahlen

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** F1
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F1

## Ausgangslage

Der Raum pausiert und speichert sofort, wenn kein Gerät mehr verbunden ist (`engine/room/actions.go` › `afterDisconnect`). Was mit dem Monarchen eines getrennten Geräts passiert, solange andere Geräte weiterspielen, und wie lange ein Platz reserviert bleibt, ist nicht als Regel festgelegt. Für die Zeit zwischen Tastendruck und Bild im WLAN gibt es kein Ziel und keine Messung.

## Ziel

Eine Regel nennt, wie lange ein Gerät getrennt sein darf, was mit dem Monarchen in dieser Zeit geschieht (schutzlos, unsichtbar, KI-gehalten), und ein Latenz-Ziel (Eingabe bis Bild in ms). Nutzen: Server und Client haben ein Verhalten und ein Maß für Pi und WLAN.

## Beteiligte und Zielgruppen

Spieler mit Handy, Tablet oder PC im Heimnetz neben der Xbox; 🧑 entscheidet (Beschluss Q04).

## Anforderungen

- Zahl: Zeit in Sekunden, die ein Platz nach Verbindungsverlust reserviert bleibt.
- Verhalten des Monarchen des getrennten Geräts, getrennt für Tag und Nacht.
- Zahl: Ziel für Eingabe bis Bild in ms (Mittel und p95) im Heimnetz; Messweg (wer misst mit welchem Werkzeug) steht dabei.
- Die Regel gilt für 2 bis 4 Spieler.

## Nicht-Ziele

Umsetzung im Server oder Client, Messung selbst (Messung im Spieleabend P1, B-151), Reconnect-UI.

## Regeln und Einschränkungen

Domäne REG; Server ist die Wahrheit (Entscheidung 001), Netz-Ziele des Pi: B-142. Protokoll nur über eigene Protokoll-Session (`docs/arbeitsweise.md` › Grenzfälle).

## Beispiele

Ein Handy fällt in der Nacht aus dem WLAN → laut Regel bleibt der Platz N s reserviert, der Monarch wird auf das Verhalten X gesetzt, danach wird der Platz frei.

## Ausnahme- und Fehlerfälle

Alle Geräte getrennt → bleibt wie heute: Raum pausiert und speichert sofort (nur bestätigen oder ändern).

## Akzeptanzkriterien

- **AC-01** Abschnitt „Verbindung“ in `docs/rules/bedienung.md` nennt die Reservierungszeit in Sekunden und das Verhalten des Monarchen bei Tag und Nacht (Sichtprüfung).
- **AC-02** Derselbe Abschnitt nennt das Latenz-Ziel in ms (Mittel und p95) und den Messweg; `task check` grün.

## Offene Fragen

Reservierungszeit, Verhalten des getrennten Monarchen, Latenz-Ziel: `docs/fragenkatalog.md` Q04, entscheidet 🧑.

## Notizen

Aus Plan Lücke 13 („Monarch schutzlos?“). Messung später im Spieleabend (P1).
