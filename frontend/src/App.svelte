<script lang="ts">
  import Button from "./lib/Button.svelte";
  import Input from "./lib/Input.svelte";
  import Message from "./lib/Message.svelte";
  import Label from "./lib/Label.svelte";

  let under_diff = $state<string>("0");
  let over_diff = $state<string>("0");
  
  let errors = $derived({
    rangeError: parseInt(under_diff) > parseInt(over_diff),
    isMinusUnderDiff: parseInt(under_diff) < 0,
    isMinusOverDiff: parseInt(over_diff) < 0,
  });

  let result = $state<number | null>(null);
  let errorMessage = $state<string | null>(null);

  const sendQuery = async (): Promise<void> => {
    if (errors.rangeError || errors.isMinusUnderDiff || errors.isMinusOverDiff) {
      return;
    }

    errorMessage = null;

    try {
      const res = await fetch(
        `http://127.0.0.1:3000/?under=${under_diff}&over=${over_diff}`
      );

      if (!res.ok) {
        throw new Error(`HTTPエラー: ${res.status}`);
      }

      const text = await res.text();
      const num = parseInt(text);

      if (isNaN(num)) {
        throw new Error("サーバから数値以外が返ってきました。");
      }

      result = num;
    } catch (err) {
      errorMessage = (err as Error).message;
      result = null;
    }
  }
</script>

<div class="w-full h-full">
  <div class="container flex flex-col max-w-xl w-full gap-2">
    <h1 class="text-3xl mb-8">AtCoder Random Picker</h1>

    {#if errors.rangeError}
      <p class="text-destructive mb-2 text-sm">最低Diffが最高Diffを超えています。</p>
    {:else if errors.isMinusOverDiff}
      <p class="text-destructive mb-2 text-sm">最低Diffが負の値になっています。</p>
    {:else if errors.isMinusUnderDiff}
      <p class="text-destructive mb-2 text-sm">最高Diffが負の値になっています</p>
    {/if}

    {#if errorMessage}
      <p class="text-destructive mb-2 text-sm">{errorMessage}</p>
    {/if}

    <div class="flex items-center gap-2">
      <Input type="number" placeholder="最低Diffを入力してください。" isErrors={errors} bind:value={under_diff} />
      <Input type="number" placeholder="最高Diffを入力してください。" isErrors={errors} bind:value={over_diff} />
      <Button onclick={sendQuery} class="shrink-0">ボタン</Button>
    </div>

    {#if result !== null}
      <div class="mt-4">
        <Message variant="success">
        <div class="flex flex-col">
          <Label class="!text-base leading-tight font-medium text-3xl mb-1.5">Success</Label>
          <p class="text-base-foreground-default mb-2 text-sm">サーバーからの結果: {result}</p>
        </div>
      </Message>
      </div>
    {/if}
  </div>
</div>