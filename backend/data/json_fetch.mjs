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

  // レスポンスをJSONとしてパース
  const data = await res.json();
  let processedData;

  // ファイル名に応じて必要なフィールドのみを抽出
  if (filename === "problems.json") {
    processedData = data.map(({ id, contest_id, name }) => ({
      id,
      contest_id,
      name,
    }));
  } else if (filename === "problem-models.json") {
    processedData = Object.fromEntries(
      Object.entries(data)
        // difficultyが存在するエントリのみに絞り込む
        .filter(([, model]) => model.difficulty !== undefined && model.difficulty !== null)
        // { difficulty: value } という形式の新しいオブジェクトを作成する
        .map(([id, model]) => [id, { difficulty: model.difficulty }])
    );
  } else {
    // 上記以外のファイルはそのまま保存（念のため）
    processedData = data;
  }

  // 抽出後のデータをJSON文字列に変換してBuffer化
  const buf = Buffer.from(JSON.stringify(processedData));
  await writeFile(`${DATA_DIR}/${filename}`, buf);
  console.log(`Saved (processed): ${DATA_DIR}/${filename}`);
}

(async () => {
  await ensureDir(DATA_DIR);
  for (const ep of endpoints) {
    await download(ep);
  }
})();