import { mkdir, writeFile, access } from "node:fs/promises";
import { constants } from "node:fs";

const DATA_DIR = "."; // 保存先ディレクトリ
const UA = "Mozilla/5.0 (atrp-updater; +https://example.com)";

const endpoints = [
  ["problems.json", "https://kenkoooo.com/atcoder/resources/problems.json"],
  ["problem-models.json", "https://kenkoooo.com/atcoder/resources/problem-models.json"],
];

async function ensureDir(dir) {
  try {
    await access(dir, constants.F_OK);
  } catch {
    await mkdir(dir, { recursive: true });
  }
}

async function download([filename, url]) {
  const res = await fetch(url, { headers: { "User-Agent": UA } });
  if (!res.ok) {
    throw new Error(`HTTP ${res.status} for ${url}`);
  }

  const arrBuf = await res.arrayBuffer();
  const buf = Buffer.from(arrBuf);
  await writeFile(`${DATA_DIR}/${filename}`, buf);
  console.log(`Saved: ${DATA_DIR}/${filename}`);
}

(async () => {
  await ensureDir(DATA_DIR);
  for (const ep of endpoints) {
    await download(ep);
  }
})();
