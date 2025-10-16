<script lang="ts">
  import Button from "./components/Button.svelte";
  import Input from "./components/Input.svelte";
  import Label from "./components/Label.svelte";
  import Message from "./components/Message.svelte";

  import { Loader } from "@lucide/svelte";
  import { type Problem, type ClosedRange, createValidRange } from "./utils/types";
  import { cacheInput, loadLastInput } from "./utils/cacher";

  const MIN_DIFF: number = 0;
  const MAX_DIFF: number = 3854;

  let cachedInput : ClosedRange | null = loadLastInput();
  let currentInput : ClosedRange | null;

  let min_diff = $state<number>(cachedInput ? cachedInput.min : MIN_DIFF);
  let max_diff = $state<number>(cachedInput ? cachedInput.max : MAX_DIFF);
  
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

  let result = $state<Problem | null>(null);        // 取得した問題
  let errorMessage = $state<string | null>(null);   // 取得できなかった場合に入るエラー
  let loading = $state<boolean>(false);             // 問題を取得中か否か

  // Backend APIを呼び出して、条件にあう問題を1問取得する
  const sendQuery = async (): Promise<void> => {
    if (errors.rangeError || errors.isMinusMinDiff || errors.isMinusMaxDiff) {
      return;
    }

    errorMessage = null;
    loading = true;

    try {
      const API_URL = import.meta.env.VITE_API_URL;
			const API_CONTENT = `${API_URL}/?min=${min_diff}&max=${max_diff}`;
      const res = await fetch(API_CONTENT);

			// 条件にあう問題がなかった場合
      if (res.status === 404) {
        const data = await res.json();
        throw new Error(data.message ?? "指定範囲内に該当する問題がありませんでした");
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
  }
</script>

<div class="w-full h-full">
  <div class="container flex flex-col max-w-xl w-full gap-2">
    <h1 class="text-3xl mb-8">AtCoder Random Picker</h1>

    {#if errors.rangeError}
      <p class="text-destructive mb-2 text-sm">最低Diffが最高Diffを超えています。</p>
    {:else if errors.isMinusMaxDiff}
      <p class="text-destructive mb-2 text-sm">最高Diffが負の値になっています。</p>
    {:else if errors.isMinusMinDiff}
      <p class="text-destructive mb-2 text-sm">最低Diffが負の値になっています</p>
    {/if}

    <div class="flex items-center gap-2">
      <Input type="number" placeholder="最低Diffを入力してください。" isErrors={errors} bind:value={min_diff} />
      <Input type="number" placeholder="最高Diffを入力してください。" isErrors={errors} bind:value={max_diff} />
       <Button onclick={() =>{sendQuery(), cacheInput(currentInput!)}} class="shrink-0 w-24 h-12 flex justify-center items-center" disabled={loading}>
        {#if loading}
          <div class="animate-spin [animation-duration: 1.05s]">
            <Loader size="1.5rem" />
          </div>
        {:else}
          Pick
        {/if}
      </Button>
    </div>

    {#if !loading && result !== null}
    <div class="mt-4">
      <Message variant="success">
        <div class="flex flex-col">
          <!-- 問題記号（idの末尾）と問題名 -->
          <Label class="leading-tight font-medium text-lg mb-1.5">
            {result.id.split('_').slice(-1)[0].toUpperCase()} - {result.name}
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
          <Button size="tiny" variant="danger" tone="ghost" class="mt-8" onclick={() => alert(`Difficulty: ${Math.floor(result!.difficulty)}`)}>
            Show Difficulty
          </Button>
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
  </div>
</div>