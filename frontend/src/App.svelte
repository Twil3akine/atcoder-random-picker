<script lang="ts">
  import Button from "./components/Button.svelte";
  import Dialog from "./components/Dialog.svelte";
  import Input from "./components/Input.svelte";
  import Label from "./components/Label.svelte";
  import Message from "./components/Message.svelte";

  import { Loader } from "@lucide/svelte";
  import {
    type Problem,
    type ClosedRange,
    createValidRange,
  } from "./utils/types";
  import {
    cacheInput,
    loadLastInput,
    loadPickActivity,
    recordPickActivity,
    type PickActivity,
  } from "./utils/cacher";

  // ============================================================

  const MIN_DIFF: number = 0;
  const MAX_DIFF: number = 3854;
  const CONTEST_OPTIONS = [
    { label: "ABC", value: "abc" },
    { label: "ARC", value: "arc" },
    { label: "AGC", value: "agc" },
    { label: "Other", value: "other" },
  ] as const;

  let cachedInput = loadLastInput();
  let currentInput: ClosedRange | null;

  let min_diff = $state<number>(cachedInput ? cachedInput.min : MIN_DIFF);
  let max_diff = $state<number>(cachedInput ? cachedInput.max : MAX_DIFF);
  let selectedContests = $state<string[]>(cachedInput?.selectedContests ?? []);
  let contest_from = $state<string>(cachedInput?.contest_from ?? "");
  let contest_to = $state<string>(cachedInput?.contest_to ?? "");

  /*
   * バリデーションチェック
   * rangeError: Max条件がMin条件未満になっていないか
   * isMinusMinDiff: Min条件が負の数になっていないか
   * isMinusMaxDiff: Max条件が負の数になっていないか
   */
  let errors = $derived({
    rangeError: !(currentInput = createValidRange(min_diff, max_diff)),
    isMinusMinDiff: min_diff < 0,
    isMinusMaxDiff: max_diff < 0,
  });
  let contestRoundError = $derived(
    contest_from !== "" &&
      contest_to !== "" &&
      Number(contest_from) > Number(contest_to),
  );

  let result = $state<Problem | null>(null); // 取得した問題
  let errorMessage = $state<string | null>(null); // 取得できなかった場合に入るエラー
  let loading = $state<boolean>(false); // 問題を取得中か否か
  let pickActivity = $state<PickActivity>(loadPickActivity());
  let activityCells = $derived(buildActivityCells(pickActivity));

  // Backend APIを呼び出して、条件にあう問題を1問取得する
  const sendQuery = async (): Promise<void> => {
    if (
      errors.rangeError ||
      errors.isMinusMinDiff ||
      errors.isMinusMaxDiff ||
      contestRoundError
    ) {
      return;
    }

    errorMessage = null;
    loading = true;

    try {
      const API_URL = import.meta.env.VITE_API_URL;
      const params = new URLSearchParams({
        min: String(min_diff),
        max: String(max_diff),
      });

      if (selectedContests.length > 0) {
        params.set("contest", selectedContests.join(","));
      }
      if (contest_from !== "") {
        params.set("contest_from", contest_from);
      }
      if (contest_to !== "") {
        params.set("contest_to", contest_to);
      }

      const API_CONTENT = `${API_URL}/?${params.toString()}`;
      const res = await fetch(API_CONTENT);

      // 条件にあう問題がなかった場合
      if (res.status === 404) {
        const data = await res.json();
        throw new Error(
          data.message ?? "指定範囲内に該当する問題がありませんでした",
        );
      }

      // 正常に問題が返ってきた場合
      if (!res.ok) {
        throw new Error(`HTTPエラー: ${res.status}`);
      }

      const json: Problem = await res.json();

      setTimeout(() => {
        result = json;
        loading = false;
      }, 1050);
    } catch (err) {
      setTimeout(() => {
        errorMessage = (err as Error).message;
        result = null;
        loading = false;
      }, 1050);
    }
  };

  // ============================================================

  let isDialogOpen = $state(false);

  const toggleDialog = (e: MouseEvent): void => {
    e.preventDefault();
    e.stopPropagation();
    isDialogOpen = !isDialogOpen;
  };

  const clickDialog = (result: boolean): void => {
    isDialogOpen = !isDialogOpen;
  };

  const markSolved = (): void => {
    pickActivity = recordPickActivity();
  };

  const toggleContest = (contest: string): void => {
    selectedContests = selectedContests.includes(contest)
      ? selectedContests.filter((value) => value !== contest)
      : [...selectedContests, contest];
  };

  const handlePick = (): void => {
    if (currentInput === null || contestRoundError) {
      return;
    }

    cacheInput({
      min: currentInput.min,
      max: currentInput.max,
      selectedContests,
      contest_from,
      contest_to,
    });
    sendQuery();
  };

  const setDefault = (): void => {
    min_diff = MIN_DIFF;
    max_diff = MAX_DIFF;
    selectedContests = ["abc"];
    contest_from = "212";
    contest_to = "";
    cacheInput({
      min: MIN_DIFF,
      max: MAX_DIFF,
      selectedContests: ["abc"],
      contest_from: "212",
      contest_to: "",
    });
  };

  function getDateKey(date: Date): string {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");

    return `${year}-${month}-${day}`;
  }

  function buildActivityCells(activity: PickActivity) {
    const today = new Date();
    const start = new Date(today.getFullYear(), 0, 1);
    const end = new Date(today.getFullYear(), 11, 31);
    const activityDays =
      Math.floor((end.getTime() - start.getTime()) / (24 * 60 * 60 * 1000)) +
      1;

    return Array.from({ length: activityDays }, (_, index) => {
      const date = new Date(start);
      date.setDate(start.getDate() + index);
      const dateKey = getDateKey(date);
      const count = activity[dateKey] ?? 0;

      return {
        dateKey,
        count,
        level: count === 0 ? 0 : Math.min(4, count),
      };
    });
  }

  function activityClass(level: number): string {
    if (level === 0) {
      return "bg-base-container-muted";
    }
    if (level === 1) {
      return "bg-success/30";
    }
    if (level === 2) {
      return "bg-success/50";
    }
    if (level === 3) {
      return "bg-success/70";
    }

    return "bg-success";
  }
</script>

<div class="w-full h-full">
  <div class="container flex flex-col max-w-xl w-full gap-2">
    <h1 class="text-3xl mb-8">AtCoder Random Picker</h1>

    {#if errors.rangeError}
      <p class="text-destructive mb-2 text-sm">
        最低Diffが最高Diffを超えています。
      </p>
    {:else if errors.isMinusMaxDiff}
      <p class="text-destructive mb-2 text-sm">
        最高Diffが負の値になっています。
      </p>
    {:else if errors.isMinusMinDiff}
      <p class="text-destructive mb-2 text-sm">
        最低Diffが負の値になっています
      </p>
    {:else if contestRoundError}
      <p class="text-destructive mb-2 text-sm">
        開始回が終了回を超えています。
      </p>
    {/if}

    <div class="flex items-center gap-2">
      <Input
        type="number"
        placeholder="最低Diffを入力してください。"
        isErrors={errors}
        bind:value={min_diff}
      />
      <Input
        type="number"
        placeholder="最高Diffを入力してください。"
        isErrors={errors}
        bind:value={max_diff}
      />
      <Button
        onclick={handlePick}
        class="shrink-0 w-24 h-12 flex justify-center items-center"
        disabled={loading ||
          errors.rangeError ||
          errors.isMinusMinDiff ||
          errors.isMinusMaxDiff ||
          contestRoundError}
      >
        {#if loading}
          <div class="animate-spin [animation-duration: 1.05s]">
            <Loader size="1.5rem" />
          </div>
        {:else}
          Pick
        {/if}
      </Button>
    </div>

    <div
      class="mt-3 flex items-center justify-between gap-2"
      aria-label="Contest filters"
    >
      <div class="flex flex-wrap gap-2">
        {#each CONTEST_OPTIONS as contest}
          <label
            class="inline-flex items-center gap-2 rounded-md border border-base-stroke-default px-3 py-2 !text-[1rem] text-base-foreground-default"
          >
            <input
              type="checkbox"
              class="accent-primary"
              checked={selectedContests.includes(contest.value)}
              onchange={() => toggleContest(contest.value)}
            />
            <span class="!text-[1rem]">{contest.label}</span>
          </label>
        {/each}
      </div>

      <Button
        onclick={setDefault}
        class="shrink-0 w-24 h-12 flex justify-center items-center"
        variant="danger"
        tone="ghost"
      >
        Reset
      </Button>
    </div>

    <div class="grid grid-cols-2 gap-2 mt-3">
      <Input
        type="number"
        min="1"
        placeholder="開始回 例: 212"
        bind:value={contest_from}
      />
      <Input
        type="number"
        min="1"
        placeholder="終了回 任意"
        bind:value={contest_to}
      />
    </div>

    {#if !loading && result !== null}
      <div class="mt-4">
        <Message variant="success">
          <div class="flex flex-col">
            <!-- 問題記号（idの末尾）と問題名 -->
            <Label class="leading-tight font-medium text-lg mb-1.5">
              {result.id.split("_").slice(-1)[0].toUpperCase()} - {result.name}
            </Label>

            <!-- URL 表示 -->
            <p class="text-base-foreground-default mb-1 text-sm">
              URL:
              <a
                class="text-blue-600 underline"
                href={`https://atcoder.jp/contests/${result.contest_id}/tasks/${result.id}`}
                target="_blank"
              >
                {result.id}
              </a>
            </p>

            <!-- Diff表示 -->
            <Button
              size="tiny"
              variant="danger"
              tone="ghost"
              class="mt-8"
              onclick={toggleDialog}>Show Difficulty</Button
            >
            <Dialog
              class="max-w-lg w-[80vw] m-auto"
              enableClose
              dismissible={true}
              bind:open={isDialogOpen}
              onClick={(result) => clickDialog(result)}
            >
              <Label class="!text-lg !font-semibold"
                >Difficulty: {Math.floor(result!.difficulty)}</Label
              >
            </Dialog>

            <Button
              size="tiny"
              variant="success"
              tone="ghost"
              class="mt-2"
              onclick={markSolved}>解いた</Button
            >
          </div>
        </Message>
      </div>
    {:else if errorMessage}
      <div class="mt-4">
        <Message variant="error">
          <div class="flex flex-col">
            <Label class="leading-tight font-medium text-lg mb-1.5">
              Failed Picking...
            </Label>

            <p class="text-base-foreground-default mb-1 text-sm">
              {errorMessage}
            </p>
          </div>
        </Message>
      </div>
    {/if}

    <div class="relative left-1/2 mt-4 flex w-screen -translate-x-1/2 flex-col items-center gap-2">
      <Label class="!text-[1rem] !font-medium">Activity</Label>
      <div class="w-screen overflow-x-auto pb-1">
        <div class="mx-auto grid w-max grid-flow-col grid-rows-7 gap-1">
          {#each activityCells as cell}
            <div
              class={`h-3 w-3 rounded-sm border border-base-stroke-default ${activityClass(cell.level)}`}
              title={`${cell.dateKey}: ${cell.count}`}
              aria-label={`${cell.dateKey}: ${cell.count}`}
            ></div>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <footer class="fixed bottom-4 left-0 flex w-full justify-center">
    <a
      class="!text-[0.875rem] text-base-foreground-muted underline underline-offset-4 hover:text-base-foreground-default"
      href="https://github.com/Twil3akine/atcoder-random-picker/issues/new/choose"
      target="_blank"
      rel="noreferrer"
    >
      お問い合わせはこちら
    </a>
  </footer>
</div>
