import { mkdir, writeFile, access } from "node:fs/promises";
import { constants } from "node:fs";

const DATA_DIR = "."; // 保存先ディレクトリ
const UA = "Mozilla/5.0 (atrp-updater; +https://example.com)";

const endpoints = {
  problems: "https://kenkoooo.com/atcoder/resources/problems.json",
  problemModels: "https://kenkoooo.com/atcoder/resources/problem-models.json",
};

async function ensureDir(dir) {
  try {
    await access(dir, constants.F_OK);
  } catch {
    await mkdir(dir, { recursive: true });
  }
}

async function downloadJson(url) {
  const res = await fetch(url, { headers: { "User-Agent": UA } });
  if (!res.ok) {
    throw new Error(`HTTP ${res.status} for ${url}`);
  }

  return res.json();
}

async function writeJson(filename, data) {
  const buf = Buffer.from(JSON.stringify(data));
  await writeFile(`${DATA_DIR}/${filename}`, buf);
  console.log(`Saved (processed): ${DATA_DIR}/${filename}`);
}

(async () => {
  await ensureDir(DATA_DIR);

  const problemsData = await downloadJson(endpoints.problems);
  const problemModelsData = await downloadJson(endpoints.problemModels);

  const problems = problemsData.map(({ id, contest_id, name }) => ({
    id,
    contest_id,
    name,
  }));

  const problemModels = Object.fromEntries(
    problems.map(({ id }) => [
      id,
      { difficulty: problemModelsData[id]?.difficulty ?? null },
    ])
  );

  await writeJson("problems.json", problems);
  await writeJson("problem-models.json", problemModels);
})();
