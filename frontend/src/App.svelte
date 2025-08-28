<script lang="ts">
  import Button from "./components/Button.svelte";
  import Input from "./components/Input.svelte";
  import Label from "./components/Label.svelte";
  import Message from "./components/Message.svelte";

  import { Loader } from "@lucide/svelte";
  import { type Problem, type ClosedRange, createValidRange } from "./utils/types";
  import { cacheInput, loadLastInput } from "./cacher";

  let cachedInput : ClosedRange | null = loadLastInput();
  let currentInput : ClosedRange | null;

  let under_diff = $state<number>(cachedInput ? cachedInput.min : 0);
  let over_diff = $state<number>(cachedInput ? cachedInput.max : 3854);
  
  let errors = $derived({
    rangeError: under_diff > over_diff,
    isMinusUnderDiff: under_diff < 0,
    isMinusOverDiff: over_diff < 0,
    invalidRange: !(currentInput = createValidRange(under_diff, over_diff))
  });

  let result = $state<Problem | null>(null);
  let errorMessage = $state<string | null>(null);
  let loading = $state<boolean>(false);

  const sendQuery = async (): Promise<void> => {
    if (errors.rangeError || errors.isMinusUnderDiff || errors.isMinusOverDiff) {
      return;
    }

    errorMessage = null;
    loading = true;

    try {
      const API_URL = import.meta.env.VITE_API_URL;
      const res = await fetch(
        `${API_URL}/?under=${under_diff}&over=${over_diff}`
      );

      if (res.status === 404) {
        const data = await res.json();
        throw new Error(data.message ?? "指定範囲内に該当する問題がありませんでした");
      }

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
    {:else if errors.isMinusOverDiff}
      <p class="text-destructive mb-2 text-sm">最高Diffが負の値になっています。</p>
    {:else if errors.isMinusUnderDiff}
      <p class="text-destructive mb-2 text-sm">最低Diffが負の値になっています</p>
    {/if}

    <div class="flex items-center gap-2">
      <Input type="number" placeholder="最低Diffを入力してください。" isErrors={errors} bind:value={under_diff} />
      <Input type="number" placeholder="最高Diffを入力してください。" isErrors={errors} bind:value={over_diff} />
      <!--TODO: ここのonClickに入力値保存機能をつける-->
       <Button onclick={() =>{sendQuery(), cacheInput(currentInput!)}} class="shrink-0 w-24 h-12 flex justify-center items-center" disabled={loading}>
      <!--ここまで-->
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