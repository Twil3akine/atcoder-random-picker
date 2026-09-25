import { writeFile } from "node:fs/promises";

const UA =
  "atcoder-random-picker-updater (+https://github.com/Twil3akine/atcoder-random-picker)";

const endpoints = {
  problems: "https://kenkoooo.com/atcoder/resources/problems.json",
  problemModels:
    "https://kenkoooo.com/atcoder/resources/problem-models.json",
};

async function downloadJson(url) {
  const response = await fetch(url, {
    headers: {
      "User-Agent": UA,
    },
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status} for ${url}`);
  }

  return response.json();
}

function normalizeProblems(raw) {
  if (!Array.isArray(raw) || raw.length === 0) {
    throw new Error("problems.json must be a non-empty array");
  }

  const seen = new Set();

  return raw.map((problem, index) => {
    if (
      typeof problem !== "object" ||
      problem === null ||
      Array.isArray(problem)
    ) {
      throw new Error(`invalid problem at index ${index}`);
    }

    const { id, contest_id, name } = problem;

    for (const [key, value] of Object.entries({
      id,
      contest_id,
      name,
    })) {
      if (typeof value !== "string" || value.trim() === "") {
        throw new Error(
          `problem at index ${index} has invalid ${key}`,
        );
      }
    }

    if (seen.has(id)) {
      throw new Error(`duplicated problem id: ${id}`);
    }

    seen.add(id);

    return {
      id,
      contest_id,
      name,
    };
  });
}

function normalizeProblemModels(problems, raw) {
  if (
    typeof raw !== "object" ||
    raw === null ||
    Array.isArray(raw)
  ) {
    throw new Error("problem-models.json must be an object");
  }

  return Object.fromEntries(
    problems.map(({ id }) => {
      const model = raw[id];

      if (model === undefined) {
        return [id, { difficulty: null }];
      }

      if (
        typeof model !== "object" ||
        model === null ||
        Array.isArray(model)
      ) {
        throw new Error(`invalid problem model: ${id}`);
      }

      const difficulty = model.difficulty ?? null;

      if (
        difficulty !== null &&
        (typeof difficulty !== "number" ||
          !Number.isFinite(difficulty))
      ) {
        throw new Error(`invalid difficulty: ${id}`);
      }

      return [
        id,
        {
          difficulty,
        },
      ];
    }),
  );
}

async function writeJson(filename, data) {
  await writeFile(filename, `${JSON.stringify(data)}\n`);
}

const [problemsData, problemModelsData] = await Promise.all([
  downloadJson(endpoints.problems),
  downloadJson(endpoints.problemModels),
]);

const problems = normalizeProblems(problemsData);
const problemModels = normalizeProblemModels(
  problems,
  problemModelsData,
);

await writeJson("problems.json", problems);
await writeJson("problem-models.json", problemModels);

console.log(`Updated ${problems.length} problems`);
