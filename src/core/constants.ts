/** Interne Render-Auflösung. Phaser skaliert per FIT auf den Bildschirm (Xbox: 1080p/4K). */
export const GAME_WIDTH = 1920;
export const GAME_HEIGHT = 1080;

/** 1 Welt-Unit (aus den Biom-/Balancing-Daten) in Pixeln. 60 Units passen auf einen Vollbild-Screen. */
export const UNIT_PX = 32;

/** Y-Position der Bodenlinie in Welt-Pixeln. */
export const GROUND_Y = 900;

/** Couch-Koop: maximale Spielerzahl. MVP = 2 (Split-Screen oben/unten, wie K2C). */
export const MAX_PLAYERS = 2;

export const PLAYER_COLORS = [0x3a86ff, 0xff006e, 0xfb5607, 0x8338ec] as const;
