/**
 * Raum-Pause des Geräts (Protokoll v8, docs/protocol.md; docs/rules/bedienung.md § 1, BD7.3): Solange das Pause-Menü
 * (Optionen-Szene) offen ist, soll das Gerät am Server pausiert sein. Reine Entscheidung, was zu senden ist:
 * `wanted` = Menü offen, `sent` = zuletzt gesendeter Zustand in diesem Raum-Beitritt, `inRoom` = Client im Raum.
 * Außerhalb des Raums wird nichts gesendet; nach einem Abbruch beendet der Server die Pause selbst, der Client setzt
 * `sent` beim Wiederbeitritt zurück und sendet bei offenem Menü erneut `pause` (Default B-535).
 */
export type PauseMessage = 'pause' | 'resume';

export function pauseMessage(wanted: boolean, sent: boolean, inRoom: boolean): PauseMessage | null {
  if (!inRoom || wanted === sent) return null;
  return wanted ? 'pause' : 'resume';
}
