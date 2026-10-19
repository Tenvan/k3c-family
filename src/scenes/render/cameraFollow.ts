/** Der Teil einer Phaser-Kamera, den die Nachführung braucht (für Tests ersetzbar). */
export interface FollowCam {
  startFollow(target: object, roundPixels?: boolean, lerpX?: number, lerpY?: number): unknown;
  stopFollow(): unknown;
}

/**
 * Führt eine Kamera ihrem Ziel nach (B-392): Die View einer Figur entsteht oft erst nach dem Aufbau der Kameras
 * (Beitritt, Stufenwechsel) oder wird neu gezeichnet. Aufgerufen wird sie jeden Frame; sie startet nur bei Änderung neu.
 * Gibt das Ziel zurück, dem die Kamera jetzt folgt.
 */
export function refollow(cam: FollowCam, current: object | undefined, wanted: object | undefined): object | undefined {
  if (wanted === current) return current;
  if (wanted === undefined) cam.stopFollow();
  else cam.startFollow(wanted, true, 0.1, 0.1);
  return wanted;
}
