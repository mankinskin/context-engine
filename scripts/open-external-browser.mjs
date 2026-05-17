#!/usr/bin/env node

import { existsSync } from 'node:fs';
import { spawn, spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';

const HELP_TEXT = `Usage: node scripts/open-external-browser.mjs [--help] <url>

Open a URL in an external Chromium-family browser using a dedicated window
that starts fullscreen.

Supported browser families:
- Google Chrome
- Chromium
- Microsoft Edge
`;

function printHelp(exitCode) {
  const stream = exitCode === 0 ? process.stdout : process.stderr;
  stream.write(HELP_TEXT);
  process.exit(exitCode);
}

function parseArgs(argv) {
  let url = null;

  for (const arg of argv) {
    if (arg === '--help' || arg === '-h') {
      printHelp(0);
    }
    if (arg.startsWith('-')) {
      printHelp(1);
    }
    if (url !== null) {
      printHelp(1);
    }
    url = arg;
  }

  if (url === null) {
    printHelp(1);
  }

  try {
    return new URL(url).toString();
  } catch {
    console.error(`Invalid URL: ${url}`);
    process.exit(1);
  }
}

function commandOnPath(command) {
  const lookup = process.platform === 'win32' ? 'where' : 'which';
  const result = spawnSync(lookup, [command], { stdio: 'ignore' });
  return result.status === 0;
}

function win32Candidates() {
  const candidates = [
    'chrome.exe',
    'chromium.exe',
    'msedge.exe',
  ];

  const roots = [
    process.env.PROGRAMFILES,
    process.env['PROGRAMFILES(X86)'],
    process.env.LOCALAPPDATA,
  ].filter(Boolean);

  const absoluteCandidates = [
    ['Google', 'Chrome', 'Application', 'chrome.exe'],
    ['Chromium', 'Application', 'chrome.exe'],
    ['Microsoft', 'Edge', 'Application', 'msedge.exe'],
  ];

  for (const root of roots) {
    for (const parts of absoluteCandidates) {
      candidates.push(path.join(root, ...parts));
    }
  }

  return candidates;
}

function macOsCandidates() {
  return [
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
    '/Applications/Chromium.app/Contents/MacOS/Chromium',
    '/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge',
  ];
}

function linuxCandidates() {
  return [
    'google-chrome',
    'google-chrome-stable',
    'chromium-browser',
    'chromium',
    'microsoft-edge',
    'microsoft-edge-stable',
  ];
}

function browserCandidates() {
  switch (process.platform) {
    case 'win32':
      return win32Candidates();
    case 'darwin':
      return macOsCandidates();
    default:
      return linuxCandidates();
  }
}

function resolveBrowserCommand() {
  const tried = [];

  for (const candidate of browserCandidates()) {
    tried.push(candidate);
    if (path.isAbsolute(candidate)) {
      if (existsSync(candidate)) {
        return { command: candidate, tried };
      }
      continue;
    }
    if (commandOnPath(candidate)) {
      return { command: candidate, tried };
    }
  }

  return { command: null, tried };
}

function main() {
  const url = parseArgs(process.argv.slice(2));
  const { command, tried } = resolveBrowserCommand();
  if (command === null) {
    console.error('No Chromium-family browser found. Tried:');
    for (const candidate of tried) {
      console.error(`- ${candidate}`);
    }
    process.exit(1);
  }

  const browserArgs = ['--new-window', '--start-fullscreen', url];
  const child = spawn(command, browserArgs, {
    detached: true,
    stdio: 'ignore',
  });
  child.unref();
}

main();