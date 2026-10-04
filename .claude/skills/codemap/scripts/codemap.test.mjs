// Run: node --test "<absolute-SKILL_FOLDER>/scripts/codemap.test.mjs"
import assert from 'node:assert/strict';
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, test } from 'node:test';

import {
  cmdChanges,
  cmdInit,
  computeFileHash,
  computeFolderHash,
  loadState,
  PatternMatcher,
  saveState,
  selectFiles,
  STATE_DIR,
  STATE_FILE,
} from './codemap.mjs';

const tempDirs = [];

function createTempDir() {
  const dir = mkdtempSync(path.join(os.tmpdir(), 'codemap-'));
  tempDirs.push(dir);
  return dir;
}

// Captures console.log output of fn so CLI commands stay quiet in the test run.
function captureLog(fn) {
  const lines = [];
  const original = console.log;
  console.log = (...args) => lines.push(args.join(' '));
  try {
    return { result: fn(), lines };
  } finally {
    console.log = original;
  }
}

afterEach(() => {
  for (const dir of tempDirs.splice(0)) {
    rmSync(dir, { force: true, recursive: true });
  }
});

describe('PatternMatcher', () => {
  test('matches expected paths', () => {
    const matcher = new PatternMatcher([
      'node_modules/',
      'dist/',
      '*.log',
      'src/**/*.ts',
    ]);

    assert.equal(matcher.matches('node_modules/foo.js'), true);
    assert.equal(matcher.matches('vendor/node_modules/bar.js'), true);
    assert.equal(matcher.matches('dist/main.js'), true);
    assert.equal(matcher.matches('src/dist/output.js'), true);
    assert.equal(matcher.matches('error.log'), true);
    assert.equal(matcher.matches('logs/access.log'), true);
    assert.equal(matcher.matches('src/index.ts'), true);
    assert.equal(matcher.matches('src/utils/helper.ts'), true);
    assert.equal(matcher.matches('README.md'), false);
    assert.equal(matcher.matches('tests/test.py'), false);
  });
});

describe('hash helpers', () => {
  test('computes file hash', () => {
    const dir = createTempDir();
    const filePath = path.join(dir, 'file.txt');
    writeFileSync(filePath, 'test content');

    assert.equal(computeFileHash(filePath), '9473fdd0d880a43c21b7778d34872157');
  });

  test('computes stable folder hash', () => {
    const fileHashes = {
      'src/a.ts': 'hash-a',
      'src/b.ts': 'hash-b',
      'tests/test.ts': 'hash-test',
    };

    const hash1 = computeFolderHash('src', fileHashes);
    const hash2 = computeFolderHash('src', fileHashes);
    const hash3 = computeFolderHash('src', {
      'src/a.ts': 'hash-a-modified',
      'src/b.ts': 'hash-b',
    });

    assert.equal(hash1, hash2);
    assert.notEqual(hash1, hash3);
  });
});

describe('selectFiles', () => {
  test('respects include and exclude patterns', () => {
    const root = createTempDir();
    mkdirSync(path.join(root, 'src'));
    mkdirSync(path.join(root, 'node_modules'));
    writeFileSync(path.join(root, 'src', 'index.ts'), 'code');
    writeFileSync(path.join(root, 'src', 'index.test.ts'), 'test');
    writeFileSync(path.join(root, 'node_modules', 'foo.js'), 'dep');
    writeFileSync(path.join(root, 'package.json'), '{}');

    const selected = selectFiles(
      root,
      ['src/**/*.ts', 'package.json'],
      ['**/*.test.ts', 'node_modules/'],
      [],
      [],
    ).map((filePath) =>
      path.relative(root, filePath).split(path.sep).join('/'),
    );

    assert.deepEqual(selected, ['package.json', 'src/index.ts']);
  });
});

describe('state', () => {
  test('loadState returns null without state and round-trips saveState', () => {
    const root = createTempDir();
    assert.equal(loadState(root), null);

    const state = { metadata: { version: '1.0.0' }, file_hashes: {} };
    saveState(root, state);

    assert.deepEqual(loadState(root), state);
    assert.ok(existsSync(path.join(root, STATE_DIR, STATE_FILE)));
  });
});

describe('cli flow', () => {
  test('init creates state and codemap.md files, changes detects a modified file', () => {
    const root = createTempDir();
    mkdirSync(path.join(root, 'src'));
    writeFileSync(path.join(root, 'src', 'index.ts'), 'a');
    writeFileSync(path.join(root, 'package.json'), '{}');

    const init = captureLog(() =>
      cmdInit({ root, include: ['src/**/*.ts', 'package.json'], exclude: [] }),
    );
    assert.equal(init.result, 0);
    assert.ok(existsSync(path.join(root, STATE_DIR, STATE_FILE)));
    assert.ok(existsSync(path.join(root, 'codemap.md')));
    assert.ok(existsSync(path.join(root, 'src', 'codemap.md')));
    assert.match(
      readFileSync(path.join(root, 'src', 'codemap.md'), 'utf8'),
      /^# src\/\n/,
    );

    const unchanged = captureLog(() => cmdChanges({ root }));
    assert.equal(unchanged.result, 0);
    assert.ok(unchanged.lines.includes('No changes detected.'));

    writeFileSync(path.join(root, 'src', 'index.ts'), 'b');
    const changed = captureLog(() => cmdChanges({ root }));
    assert.equal(changed.result, 0);
    assert.ok(changed.lines.some((line) => line.includes('~ src/index.ts')));
    assert.ok(changed.lines.some((line) => line.trim() === 'src/'));
  });
});
