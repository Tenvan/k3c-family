# B-313 · W4.3b kann den Vermerk „Wirkung offen“ nur mit einer Änderung an sites_test.go ersetzen

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** W7
- **Projekt:** –
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W4.3b (Sprint W4) soll in Schritt 2 den Vermerk „Wirkung offen“ an Schmiede und Rüstkammer in `data/buildings.json`
ersetzen (Erlaubte Dateien: „Vermerk ‚Wirkung offen‘ an Schmiede und Rüstkammer ersetzen“). Der W3.2-Test
`TestSchmiedeUndRuestkammerZerstoerbarWirkungOffen` (`engine/sim/sites_test.go:120`) verlangt aber genau diesen Text:
`strings.Contains(notes[kind].Notes, "Wirkung offen")` für `smithy` und `armory`. Ersetzt W4.3b den Vermerk, wird
`task check:go` rot; `engine/sim/sites_test.go` steht nicht in den Erlaubten Dateien.

## Ziel

W4.3b kann den Vermerk ersetzen und `task check:go` bleibt grün.

## Beteiligte und Zielgruppen

Entwickler (SIM); 🧑 entscheidet über die Erweiterung der Erlaubten Dateien.

## Anforderungen

- Nach W4.3b beschreiben die Notizen von Schmiede und Rüstkammer ihre Wirkung (Elite, Rüstung).
- Der W3.2-Test prüft weiter Bau und Zerstörung beider Gebäude.

## Nicht-Ziele

Weitere Änderungen an W3-Tests; Elite und Rüstung selbst (W4.3b).

## Regeln und Einschränkungen

Session nur in ihren Erlaubten Dateien (`docs/arbeitsweise.md`); Domäne SIM bleibt gewahrt.

## Beispiele

Notiz der Schmiede lautet nach W4.3b etwa „Elite-Bogenschütze am Platz, Elite-Krieger als Angebot …“ → der W3.2-Test
bleibt grün, weil er den Vermerk nicht mehr verlangt.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Freigabe-Frage, kein Laufzeitfall.

## Akzeptanzkriterien

- **AC-01** W4.3b darf in `engine/sim/sites_test.go` › `TestSchmiedeUndRuestkammerZerstoerbarWirkungOffen` die Prüfung
  auf „Wirkung offen“ entfernen (Test dann sinngemäß `TestSchmiedeUndRuestkammerZerstoerbar`), oder 🧑 legt einen
  anderen Weg fest; W4.3b wird danach fortgesetzt.

## Offene Fragen

Entschieden 2026-10-06 (🧑, Chat): Weg (a). `engine/sim/sites_test.go` kommt in die Erlaubten Dateien, nur dieser
Test: Vermerk-Prüfung raus, Testname ohne „WirkungOffen“. W7 ändert keine W4-Dateien; W7.1 prüft, ob W4.3b schon
erledigt ist, und ändert den Test sonst selbst.

## Notizen

Gefunden zu Beginn von W4.3b (Schritt 1, Code-Stand prüfen); umgesetzt ist in W4.3b noch nichts.

Sonst ist der Code-Stand vollständig (geprüft 2026-10-06): Schmiede (Angebot `eliteWarrior` dx +4, `craftSeconds`),
Rüstkammer (`craftSeconds.armor`), Heilplatz (`healing.go`), Krieger und Verlust-Kaskade (W4.3a), Hub-Stufen 3/4 für
Schmiede und Rüstkammer (`data/hub.json`). Geplanter Weg in den Erlaubten Dateien: Gold der Produkte am Platz über
`sitePayable`/`paySite`/`refundSite` (Elite-Bogenschütze, Rüstung), Elite-Krieger über `OfferPaid` des Angebots
(`refundOffer` passt), Logik in `upgrades.go`, Rüstung als `World.ArmorLevel` (`omitempty`), neue Kämpfer über
`makeFighter`. Risiko: `engine/sim/economy.go` hat 396 von 400 Zeilen, der Weg braucht dort etwa 3.
