# B-207 · Der Client zeigt freie und gesperrte Bauplätze mit Grund (ab Hub-Stufe n, Linie fehlt)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** W11
- **Projekt:** WRT
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-16, Chat (Ralf), mit W11 Rev. 3

## Ausgangslage

Mit festen Bauplätzen (Q43, B-206) gibt es Plätze, die noch gesperrt sind: Linie k braucht Hub-Stufe k und die Linie k−1 derselben Seite (Q48), ein Tor ist nur an der äußersten gebauten Linie bezahlbar (Q47), Gebäude schalten mit der Hub-Stufe frei. Der Client (`src/scenes/worldRenderer.ts › sites.sync`, `siteView.ts`) zeichnet jeden Platz aus dem Zustand, zeigt aber nicht, warum er (noch) nicht bezahlbar ist. Der früher geplante Hinweis „Kein Platz für <Gebäude>“ (Q26) entfällt, weil kein Platz fehlen kann.
Stand 2026-10-16: Der Server schickt seit W11.2 `sites[].hubLevel` und alle Plätze bis Hub-Stufe + 1 (`docs/protocol.md` › Wirtschaft), die Stufe „ab Hub-Stufe n“ ist also am Platz bekannt. „Linie fehlt“ und „Tor nur außen“ trägt das Protokoll nicht (`Site` ohne Linie und Seite); dafür B-447.

## Ziel

Spieler sehen am Platz, ob er frei ist und was ihnen noch fehlt, statt an einem stummen Platz zu stehen.

## Beteiligte und Zielgruppen

Spieler (auch Kinder, 2+ Spieler im Split-Screen); Entwickler CLI.

## Anforderungen

- Freie Plätze sind erkennbar (Zahlziel sichtbar): `hubLevel` ≤ Hub-Stufe, oder bezahlt bzw. gebaut.
- Gesperrte Plätze zeigen den Grund „ab Hub-Stufe n“ (`sites[].hubLevel` > Hub-Stufe, vom Server bis Hub-Stufe + 1 geschickt).
- „Linie fehlt“ und „Tor nur an der äußersten Linie“ folgen mit B-447 (Server liefert Linie und Sperrgrund); der Client rechnet sie nicht aus `x` nach (Entscheidung 001). Bis dahin zeigt ein Platz ohne bekannten Grund nichts Falsches an.
- Text nach Q03 (Mindestgröße, kurz statt klein), deutsch und englisch, zentral für B-172 (`t()`).
- Liest nur Protokoll und Daten (B-208, B-209), keine eigene Regel-Logik; fehlt `hubLevel` (älterer Server), bleibt die Anzeige wie bisher.
- Die Anzeige ist eine Zeile im Preisschild des unbezahlten Platzes (`priceTagRules.ts`) und verdeckt den Wartegrund (W6.2, `siteStatusView.ts`) nicht, auch nicht an den Treppen (≥ 8 Units, B-409) und bei den GEB-Grafiken in Maßstab 1.

## Nicht-Ziele

Layout und Freischaltung selbst (B-206, SIM); Wartezeit „bezahlt bis gebaut“ (B-117, W6); Grafiken (GEB); Sperrgrund „Linie fehlt“ und „Tor nur außen“ (B-447, SRV).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 3; Client zeichnet nur (Entscheidung 001), kein `Math.random()`. Controller-Taste B bleibt frei, kein Bau-Menü (Q34). Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Hub-Stufe 1, Spieler steht am Platz der Linie 2 (`hubLevel` 2) → Hinweis „ab Hub-Stufe 2“. Hub-Stufe 1, Spieler steht an der Burg, Plätze mit `hubLevel` ≤ 1 → frei, kein Hinweis. Hub-Stufe 1 und ein Platz mit `hubLevel` 3 kommt nicht im Zustand (Server filtert, W11.2) → nichts zu zeigen.

## Ausnahme- und Fehlerfälle

Zwei Spieler an verschiedenen gesperrten Plätzen → jede Bildschirmhälfte zeigt ihren Grund. Protokoll ohne Platzdaten (alter Server) → keine Anzeige, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Test: Eine reine Funktion bildet Platz und Hub-Stufe auf „frei“ oder „ab Hub-Stufe n“ ab; ein Platz ohne `hubLevel` bleibt unbeschriftet; bezahlte und gebaute Plätze sind nie gesperrt.
- **AC-02** Gesperrte und freie Plätze werden in der Welt angezeigt, Text de/en über `t()`; `task check` grün.
- **AC-03** 🧑 hat die Anzeige am Gerät abgenommen.

## Offene Fragen

Sprint: W11 (Beschluss 🧑 2026-10-14), Session W11.4. Offen für 🧑 (2026-10-16, Entwurf): „Linie fehlt“ und „Tor nur außen“ erst mit B-447 (Vorschlag) oder jetzt mit einer SRV-Session in W11?

## Notizen

Ersetzt den verworfenen Hinweis „Kein Platz für <Gebäude>“ aus Q26 (Beschluss Q43, 2026-10-04). Abhängig von B-206 und B-208 (beide erledigt bzw. W11.2 fertig). Revision 2 (2026-10-16, Entwurf): Umfang auf „frei“ und „ab Hub-Stufe n“ begrenzt, Sperrgrund Linie nach B-447, Ausgangslage auf den Stand nach W11.2.
