// Regenerates the bundled app icons in src-tauri/icons from the two SVG sources:
// app-icon.svg (48 px and up) and app-icon-small.svg (16–32 px, no digits).
// `tauri icon` takes a single source, so the multi-size .ico and the small .icns
// entries are assembled here.
// Usage: bun run icons
import { spawnSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = fileURLToPath(new URL('..', import.meta.url));
const ICONS = join(ROOT, 'src-tauri/icons');
const TMP = join(ROOT, 'target/icons');
const TAURI_CLI = join(ROOT, 'node_modules/@tauri-apps/cli/tauri.js');
/** Files referenced by tauri.conf.json `bundle.icon`, plus icon.png (default window icon). */
const KEEP = ['128x128.png', '128x128@2x.png', 'icon.png'];
const ICO_SMALL = [16, 24, 32];
const ICO_LARGE = [48, 64, 256];

function tauriIcon(source: string, out: string, sizes: number[] = []) {
  const args = ['--bun', TAURI_CLI, 'icon', source, '-o', out];
  for (const size of sizes) args.push('-p', String(size));
  const result = spawnSync('bun', args, { stdio: 'inherit', cwd: ROOT });
  if (result.status !== 0) throw new Error(`tauri icon failed for ${source}`);
}

/** Packs PNG images into a .ico (PNG-compressed entries, supported since Windows Vista). */
function buildIco(images: { size: number; png: Buffer }[]): Buffer {
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0); // reserved
  header.writeUInt16LE(1, 2); // type: icon
  header.writeUInt16LE(images.length, 4);
  const entries: Buffer[] = [];
  let offset = 6 + 16 * images.length;
  for (const { size, png } of images) {
    const entry = Buffer.alloc(16);
    entry.writeUInt8(size >= 256 ? 0 : size, 0); // 0 means 256
    entry.writeUInt8(size >= 256 ? 0 : size, 1);
    entry.writeUInt8(0, 2); // palette colours
    entry.writeUInt8(0, 3); // reserved
    entry.writeUInt16LE(1, 4); // colour planes
    entry.writeUInt16LE(32, 6); // bits per pixel
    entry.writeUInt32LE(png.length, 8);
    entry.writeUInt32LE(offset, 12);
    entries.push(entry);
    offset += png.length;
  }
  return Buffer.concat([header, ...entries, ...images.map((image) => image.png)]);
}

/**
 * Takes the .icns `tauri icon` built from the detailed source and swaps its 16 and 32 px
 * images for the small source. The legacy RLE entries (is32/s8mk 16 px, il32/l8mk 32 px)
 * are replaced by their PNG equivalents (icp4, icp5; macOS 10.7+); ic11 is 16 pt @2x.
 */
function withSmallIcns(icns: Buffer, png16: Buffer, png32: Buffer): Buffer {
  const DROP = new Set(['is32', 's8mk', 'il32', 'l8mk', 'ic11']);
  const chunks: Buffer[] = [];
  for (let offset = 8; offset < icns.length; ) {
    const type = icns.toString('latin1', offset, offset + 4);
    const length = icns.readUInt32BE(offset + 4);
    if (!DROP.has(type)) chunks.push(icns.subarray(offset, offset + length));
    offset += length;
  }
  for (const [type, png] of [
    ['icp4', png16],
    ['icp5', png32],
    ['ic11', png32],
  ] as const) {
    const head = Buffer.alloc(8);
    head.write(type, 0, 'latin1');
    head.writeUInt32BE(8 + png.length, 4);
    chunks.push(head, png);
  }
  const body = Buffer.concat(chunks);
  const head = Buffer.alloc(8);
  head.write('icns', 0, 'latin1');
  head.writeUInt32BE(8 + body.length, 4);
  return Buffer.concat([head, body]);
}

rmSync(TMP, { recursive: true, force: true });
const full = join(TMP, 'full');
const small = join(TMP, 'small');
const sized = join(TMP, 'sized');
mkdirSync(TMP, { recursive: true });

// Full platform set from the detailed source; only KEEP (and the patched .icns) is used.
tauriIcon(join(ICONS, 'app-icon.svg'), full);
for (const file of KEEP) copyFileSync(join(full, file), join(ICONS, file));

tauriIcon(join(ICONS, 'app-icon-small.svg'), small, ICO_SMALL);
tauriIcon(join(ICONS, 'app-icon.svg'), sized, ICO_LARGE);
const smallPng = (size: number) => readFileSync(join(small, `${size}x${size}.png`));
copyFileSync(join(small, '32x32.png'), join(ICONS, '32x32.png'));

const ico = buildIco([
  ...ICO_SMALL.map((size) => ({ size, png: smallPng(size) })),
  ...ICO_LARGE.map((size) => ({ size, png: readFileSync(join(sized, `${size}x${size}.png`)) })),
]);
writeFileSync(join(ICONS, 'icon.ico'), ico);

const icns = withSmallIcns(readFileSync(join(full, 'icon.icns')), smallPng(16), smallPng(32));
writeFileSync(join(ICONS, 'icon.icns'), icns);
console.log(`Wrote ${KEEP.length + 3} icons to ${ICONS}`);
