import type { ToolTextKey } from './texts.de';

/** Englische Texte der Werkzeug-Seiten (PL2, B-322); ein fehlender Schlüssel fällt auf Deutsch zurück. */
export const en: Partial<Record<ToolTextKey, string>> = {

  // Monitoring page (monitor.html)
  'monitor.pageTitle': 'K3C Monitor',
  'monitor.connecting': 'Connecting …',
  'monitor.saveToken': 'Save token',
  'monitor.tab.overview': 'Overview',
  'monitor.tab.history': 'History',
  'monitor.tab.events': 'Events',
  'monitor.h.tick': 'Tick duration (max per s)',
  'monitor.h.rtt': 'RTT per device',
  'monitor.h.server': 'Server: heap and CPU',
  'monitor.legend': 'Red dots: outliers. Marks below: diagnostic events. Dashed: restart. ▼: selected event.',
  'monitor.keys': 'Controller: D-pad/stick selects, A confirms. Touch and mouse: tap.',
  'monitor.kind.crash': '💥 Crash',
  'monitor.kind.slow': '🐢 Slow tick',
  'monitor.kind.drop': '👋 Disconnect',
  'monitor.kind.client': '📨 Client error',
  'monitor.rooms': 'Rooms (last 5 min)',
  'monitor.noRoom': 'No active room.',
  'monitor.noPong': 'no pong',
  'monitor.roomLine': 'Tick p99 {tick} · RTT {rtt} · Errors {errors}',
  'monitor.serverLine': 'Heap {heap} MB · CPU {cpu} · Goroutines {goroutines}',
  'monitor.restart': 'Restart',
  'monitor.heapMB': 'Heap MB',
  'monitor.all': 'all',
  'monitor.roomFilter': 'Room: {room}',
  'monitor.kindFilter': 'Kind: {kind}',
  'monitor.window1h': 'Window 1 h',
  'monitor.window5m': 'Window 5 min',
  'monitor.overBudget': 'Budget exceeded',
  'monitor.ticks': '{n} ticks',
  'monitor.noEvents': 'No events in the last hour.',
  'monitor.eventRoom': ' · Room {room}',
  'monitor.tokenAsk': '🔒 Enter token (K3C_STATUS_TOKEN of the server).',
  'monitor.tokenBad': '🔒 Token missing or wrong, please enter it again.',
  'monitor.ok': '✅ As of {now} · Server since {since}',
  'monitor.off': '🚫 Diagnostics off, set K3C_STATUS_TOKEN on the server.',
  'monitor.offline': '❌ Server not reachable, next attempt in {wait} s.',

  // Graphics page (grafiken.html) and credits (lizenzen.html)
  'credits.work': 'Work',
  'credits.author': 'Author(s)',
  'credits.license': 'License',
  'credits.source': 'Source',
  'credits.attribution': 'attribution',

  // Developer page (dev.html) and its tiles; the tiles also appear in the game menu
  'dev.pageTitle': 'K3C Development',
  'dev.title': 'Development',
  'dev.sub': 'Tools for development, performance and balancing. Most of them need the Go server running.',
  'dev.section.dev': 'Development',
  'dev.section.perf': 'Performance',
  'dev.section.balance': 'Balancing',
  'dev.empty': 'No entries yet.',
  'tool.opening': 'Opening “{title}” …',
  'tile.gamedev.title': 'Game (Dev)',
  'tile.gamedev.desc': 'Game with debug overlay (?dev=1) · window.game for the console',
  'tile.monitor.title': 'Monitor',
  'tile.monitor.desc': 'Server state: traffic light per room, history with percentiles, error timeline (Go server with K3C_STATUS_TOKEN)',

  // Selection bar of the reference pages
  'sprite.ours': 'Our side',
  'sprite.enemy': 'Enemy side',
  'sprite.mountsKing': 'Mounts (with king 1)',
  'sprite.monarchs': 'Monarchs',
  'sprite.troops': 'Troops',
  'sprite.enemies': 'Enemies',
  'sprite.mountsLater': 'Mounts (not in the game yet)',
  'sprite.reserve': 'Reserve (no role yet)',
  'sprite.noRole': 'no role yet',
  'sprite.tinted': 'tinted {tint}',
  'sprite.translucent': 'translucent',
  'sprite.player': 'Player {n}',
  'sprite.depth': '{name} · depth {depth}',
  'sprite.height': '{id} · {px} px tall in the game',

  // Licenses page (lizenzen.html); texts with HTML are set via data-t-html
  'lic.pageTitle': 'K3C Licenses & Credits',
  'lic.title': 'Licenses & Credits',
  'lic.sub':
    'Family Three Crowns (K3C) is a family project and stands on the shoulders of many generous people. This page lists the license our game runs under and whom we thank for graphics, libraries and tools. Scroll with stick or D-pad.',
  'lic.h.ours': 'Our license',
  'lic.boxCode':
    '<strong>Source code:</strong> <a href="https://polyformproject.org/licenses/noncommercial/1.0.0" target="_blank" rel="noopener">PolyForm Noncommercial License 1.0.0</a> · file <code>LICENSE</code><br /><strong>Our own graphics, animations, sounds and music:</strong> <a href="https://creativecommons.org/licenses/by-nc/4.0/deed.en" target="_blank" rel="noopener">CC BY-NC 4.0</a> · file <code>LICENSE-ASSETS</code>',
  'lic.boxCopy':
    'Copyright © 2026 Ralf Stich. The source code is openly viewable (<em>source-available</em>), but <strong>not</strong> free for commercial use.',
  'lic.ok1': 'Playing, learning, trying out, modifying and sharing for private, family, hobby, research and educational purposes.',
  'lic.ok2': 'Sharing with attribution and with the license text.',
  'lic.no1': 'Selling, renting, advertising or paid models, building it into a commercial product or a paid service.',
  'lic.dim1': 'Commercial use is only possible with written permission from Ralf Stich. The respective license text applies; this summary does not replace it.',
  'lic.dim2':
    'The non-commercial clause only applies to our own work. The third-party material below keeps its original license and stays usable for everyone as its authors permitted.',
  'lic.h.graphics': 'Graphics',
  'lic.graphics':
    'All graphics in the game are <strong>our own paper-cut art</strong>, generated from specs (<code>tools/paper</code>). There is no third-party artwork in it, so there are no graphics credits here.',
  'lic.creditsNote':
    'The entries are in <code>public/audio/README.md</code>; this page reads them from there.',
  'lic.h.sound': 'Sound & Music',
  'lic.sound':
    'Sound effects and music come from free sources or are our own recordings; authors, license and source are listed below, our own recordings under CC BY-NC 4.0.',
  'lic.h.software': 'Software',
  'lic.softwareIntro': 'These building blocks run in the game or build it. Thanks to everyone who maintains them.',
  'lic.th.part': 'Component',
  'lic.th.purpose': 'Purpose',
  'lic.th.license': 'License',
  'lic.sw.phaser': 'Game engine in the browser',
  'lic.sw.ws': 'WebSocket for online mode (Node server)',
  'lic.sw.coder': 'WebSocket in the Go server',
  'lic.sw.vite': 'Dev server and build',
  'lic.sw.ts': 'Programming language',
  'lic.sw.tests': 'Tests and lint',
  'lic.sw.go': 'Server and the game engine of the future',
  'lic.softwareNote':
    'Further tools used only for development (e.g. in <code>tools/k3c-workbench</code>) are not shipped with the game; their licenses are in the respective packages.',
  'lic.h.thanks': 'Thank you',
  'lic.thanks':
    'From the heart to <strong>LuizMelo</strong>, <strong>ansimuz</strong>, <strong>bluecarrot16</strong>, <strong>Jordan Irwin (AntumDeluge)</strong>, <strong>lawnjelly</strong>, <strong>Calciumtrice</strong>, <strong>lemmling</strong>, <strong>Bonsaiheldin</strong>, <strong>vico</strong>, <strong>Rusher_go</strong> and <strong>WolfTech</strong>, to the platforms <strong>itch.io</strong> and <strong>OpenGameArt.org</strong> and to all the open-source developers above. If you enjoy their work: drop by and support them if you can.',
  'lic.thanksNote': 'Is a credit missing or wrongly assigned? Please let us know, we will fix it right away.',
};
