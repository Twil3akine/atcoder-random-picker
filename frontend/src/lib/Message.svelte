<!--
@component
## 概要
- ユーザーに重要な情報や通知を伝えるためのコンポーネントです

## 機能
- 見た目を変更するためのいくつかのスタイル用Propsが追加されています(詳細はPropsセクションを参照)

## Props
- variant: 通知の意味を指定すると、それに合わせたスタイルになります

## Usage
```svelte
  <Message variant="default">
    <div class="flex flex-col">
      <Label class="!text-base leading-tight font-medium mb-1.5">ラベル</Label>
      <p class="text-base-foreground-default text-sm">ここに補足文が入ります。</p>
    </div>
  </Message>
```
-->

<script module lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import type { Snippet } from 'svelte';

  export const messageVariants = cva(
    'flex items-start gap-3 w-full p-4 border rounded-lg',
    {
      variants: {
        /** 通知の使用用途 */
        variant: {
          error: ['bg-destructive/10 border-destructive'],
          warning: ['bg-warning/10 border-warning'],
          success: ['bg-success/10 border-success'],
          default: ['bg-primary/10 border-primary'],
        },
      },
      defaultVariants: {
        variant: "default",
      }
    },
  );

  export const messageIconVariants = cva('', {
    variants: {
      variant: {
        error: ['text-destructive'],
        warning: ['text-warning'],
        success: ['text-success'],
        default: ['text-primary'],
      },
      defaultVariants: {
        variant: "default",
      }
    },
  });

  export type MessageVariants = VariantProps<typeof messageVariants>;

  export interface MessageProps extends MessageVariants {
    /** クラス */
    class?: string;
    children?: Snippet<[]>;
  }
</script>

<script lang="ts">
  import { Info, CircleAlert, TriangleAlert, Check } from '@lucide/svelte';
  
  let { variant, class: className = '', children }: MessageProps = $props();

  let messageVariantClass = $derived(messageVariants({
    variant,
    class: className,
  }));

  let messageIconVariantClass = $derived(messageIconVariants({
    variant,
  }));
</script>

<div class={messageVariantClass}>
  <div class={messageIconVariantClass}>
    {#if variant === "error"}
      <CircleAlert size="1rem" />
    {:else if variant === "warning"}
      <TriangleAlert size="1rem" />
    {:else if variant === "success"}
      <Check size="1rem" />
    {:else}
      <Info size="1rem" />
    {/if}
  </div>
  {@render children?.()}
</div>
