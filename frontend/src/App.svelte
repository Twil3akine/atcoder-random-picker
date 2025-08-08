<script lang="ts">
  import Button from "./lib/Button.svelte";
  import Input from "./lib/Input.svelte";

  let value = $state("");
  let isError = $derived(value === "");
</script>

<div class="flex flex-col w-full">
  {#if isError}
    <p class="text-destructive mb-2 text-sm">エラーメッセージです。</p>
  {/if}
  <Input type="text" placeholder="プレースホルダー" required {isError} bind:value />

  <Button>aaa</Button>
</div>