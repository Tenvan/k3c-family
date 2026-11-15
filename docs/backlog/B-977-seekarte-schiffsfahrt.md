# B-977 · Beim Inselwechsel fährt das Schiff im Vollbild über die Seekarte zur Zielinsel

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GS2
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Inselwechsel erfolgt gemeinsam, wenn alle lebenden Spieler am Steg stehen (`docs/rules/stufen.md` § 1, Ereignis `islandSwitch`); danach baut die Engine die Zielinsel und das Spiel zeigt den Ladebildschirm (`native/k3c-game/game/loading.go`, `loadscreen.go`). Die Wahl des Ziels mit A im Overlay (B-962) löst selbst noch keinen Wechsel aus. B-964 liefert das Diorama und die Assets (Meer, Miniaturen, Schiff). Abgespalten aus B-964 (🧑 2026-11-15, Schiff und Anzeige).

## Ziel

Der Inselwechsel zeigt statt des bloßen Ladebildschirms die Seekarte im Vollbild: Das Schiff fährt über die Papierwellen von der aktuellen zur Zielinsel, danach beginnt die neue Insel.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen). Claude setzt um, 🧑 gibt frei und nimmt am TV ab.

## Anforderungen

Aus B-964 (🧑 2026-11-15, wörtlich):

- **Schiff: fährt die Route.** Das Schiff liegt an der aktuellen Insel; nach der Wahl mit A fährt es als kurze Animation über die Papierwellen zur Zielinsel, danach lädt die Insel (dient als Übergang).
- **Anzeige: Overlay, Fahrt im Vollbild.** Die Wahl läuft als Overlay in der Hälfte des Spielers am Steg (wie B-962), der andere spielt weiter; erst die Schiffsfahrt nach A übernimmt den ganzen Bildschirm.
- Funktioniert mit 2 Spielern im Split-Screen.

Ergänzt:

- **R1 Auslöser:** Die Fahrt startet mit dem Inselwechsel der Engine (`islandSwitch`, alle lebenden Spieler am Steg), nicht schon beim Druck auf A; A legt nur das Ziel fest (Regel B-961). Ziel der Fahrt ist das Ziel des Wechsels (`ChosenTarget`).
- **R2 Bild:** Vollbild ohne Split-Screen: dasselbe Diorama wie im Overlay (B-964), Schiff fährt auf einem leichten Bogen von der aktuellen zur Zielinsel, die vordere Wellenschicht bewegt sich, die abgeschlossene Insel bekommt beim Ablegen ihre Fahne.
- **R3 Dauer und Laden:** Die Fahrt dauert mindestens die Fahrzeit (Startwert 3 s, vorläufig) und endet frühestens, wenn die Zielinsel geladen ist; lädt sie länger, wartet das Schiff an der Zielinsel mit bewegten Wellen. Sie ersetzt den Ladebildschirm beim Inselwechsel; beim Laden eines Spielstands und beim neuen Spiel bleibt der bisherige Ladebildschirm.
- **R4 Eingabe:** Während der Fahrt wirkt keine Spiel-Eingabe; sie ist nicht abbrechbar. Die Zeit kommt aus der Uhr des Spiels, die Simulation rechnet währenddessen nicht weiter (wie heute beim Laden).
- **R5 Rückfall:** Fehlen die Assets (B-964/R5), erscheint der bisherige Ladebildschirm.
- **R6 Danach:** Die neue Insel beginnt wie heute im Split-Screen; die Ankunftszeile der Geschichte (B-974) blendet erst nach der Fahrt ein.

## Nicht-Ziele

Diorama und Assets (B-964), Regeln der Zielwahl (B-961), Fahrt beim Laden eines Spielstands, Ton der Fahrt (eigenes Ticket bei Bedarf), Andenken (B-966).

## Regeln und Einschränkungen

- Das Spiel zeichnet nur; der Wechsel selbst bleibt in der Engine.
- Fachprüfung `k3c-hud` vor dem Commit.

- **Bereich:** `native/k3c-game/game/**`, `native/k3c-game/hud/**`, `native/k3c-game/view/**`
- **Abhängig von:** B-964
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Sengoku zu zweit, Ziel Bambuswälder gewählt, beide reiten auf den Steg: Der Split-Screen weicht der Karte im Vollbild, die Reisfeld-Ebene bekommt ihre Fahne, das Schiff fährt in 3 s zu den Bambuswäldern, dann beginnt die neue Insel im Split-Screen und die Ankunftszeile blendet ein.

## Ausnahme- und Fehlerfälle

- Kein Ziel gewählt → Fahrt zum ersten Ziel der Liste (Regel B-961).
- Laden dauert länger als die Fahrzeit → Schiff wartet am Ziel, kein Standbild ohne Bewegung.
- Laden der Zielinsel schlägt fehl → Verhalten wie heute beim Ladebildschirm (Fehlermeldung, `game/fault.go`).
- Nur ein Spieler → gleiche Fahrt im Vollbild.
- Assets fehlen → bisheriger Ladebildschirm.

## Akzeptanzkriterien

- **AC-01** Go-Test in `native/k3c-game/game`: Auf `islandSwitch` wechselt das Spiel in die Fahrt-Szene mit Start (aktuelle Insel) und Ziel (`ChosenTarget`); ohne Wahl ist das Ziel das erste der Liste.
- **AC-02** Go-Test: Die Fahrt endet frühestens nach der Fahrzeit und erst, wenn das Laden fertig ist (Fall Laden schneller und Fall Laden langsamer); Eingaben während der Fahrt lösen keine Aktion aus.
- **AC-03** Go-Test: Laden eines Spielstands und neues Spiel zeigen den bisherigen Ladebildschirm; ohne Assets zeigt auch der Inselwechsel ihn.
- **AC-04** Go-Test: Die Position des Schiffs ist eine reine Funktion der Zeit (gleiche Zeit, gleiche Position), keine Allokation je Frame.
- **AC-05** TV (🧑): Inselwechsel zu zweit: Fahrt im Vollbild ist flüssig, das Ziel ist erkennbar, danach startet die neue Insel im Split-Screen.

## Offene Fragen

- „Nach der Wahl mit A fährt es“: A wählt heute nur das Ziel, der Wechsel braucht alle Spieler am Steg. → Default: Die Fahrt startet mit dem Inselwechsel (alle am Steg), A legt das Ziel fest (vorläufig, 🧑 bestätigt). Alternative: A eines gekrönten Spielers löst den Wechsel sofort aus – das wäre eine Regeländerung an B-961 und ein eigenes Ticket.
- Fahrzeit → Default: 3 s (vorläufig, Abstimmung am TV).
- Abbrechbar? → Default: nein (vorläufig).

## Notizen

Abgespalten aus B-964 (2026-11-15, Zerlegung).
