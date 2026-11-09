// Writes the updater manifest (<dist>/latest.json) for a signed Windows installer.
// Usage: node make-latest-json.mjs <tag> <owner/repo> <dist> <notes-file>
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const [tag, repo, dist, notesFile] = process.argv.slice(2);
if (!tag || !repo || !dist || !notesFile) {
  console.error('Usage: make-latest-json.mjs <tag> <owner/repo> <dist> <notes-file>');
  process.exit(1);
}

const version = tag.replace(/^v/, '');
const conf = JSON.parse(readFileSync('src-tauri/tauri.conf.json', 'utf8'));
if (conf.version !== version) {
  console.error(`Tag ${tag} does not match tauri.conf.json version ${conf.version}`);
  process.exit(1);
}

const asset = `Virtual.Copy.Paste_${version}_x64-setup.exe`;
const entry = {
  signature: readFileSync(join(dist, `${asset}.sig`), 'utf8').trim(),
  url: `https://github.com/${repo}/releases/download/${tag}/${asset}`,
};

const manifest = {
  version,
  notes: readFileSync(notesFile, 'utf8').trim(),
  pub_date: new Date().toISOString(),
  platforms: {
    'windows-x86_64': entry,
    'windows-x86_64-nsis': entry,
  },
};

writeFileSync(join(dist, 'latest.json'), `${JSON.stringify(manifest, null, 2)}\n`);
console.log(`Wrote ${join(dist, 'latest.json')} for ${tag}`);
