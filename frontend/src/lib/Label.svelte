<!--
@component
## 概要
- テキスト入力欄や選択肢などのフォーム要素に対して、その内容を明示するために使用されるコンポーネントです

## 機能
- フォーム要素と関連付けることでアクティブ化することができます
- 任意の属性を渡せます

## Props
- disabled: 指定するとグレーアウトされ、クリック不可になります

## Usage
```svelte
<Label for="forName">ラベル</Label>
```
-->
<script module lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import type { Snippet } from 'svelte';
  import type { HTMLLabelAttributes } from 'svelte/elements';

  export const labelVariants = cva('inline-block w-fit leading-tight text-sm text-base-foreground-default', {
    variants: {
      /** 操作できるかどうか */
      disabled: {
        true: ['opacity-50'],
        false: [],
      },
    },
    defaultVariants: {
      disabled: false,
    },
  });

  export type LabelVariants = VariantProps<typeof labelVariants>;

  export interface LabelProps extends LabelVariants, HTMLLabelAttributes {
    /** 必須かどうか */ 
    required?: boolean;
    /** クラス */
    class?: string;
    children?: Snippet<[]>;
  }
</script>

<script lang="ts">
  let { disabled = false, class: className = '', children, required = false, ...labelAttributes }: LabelProps = $props();

  let labelVariantClass = $derived(labelVariants({
    disabled,
    class: className,
  }));
</script>

<label class={labelVariantClass} {...labelAttributes}>
  {@render children?.()}

  {#if required}
    <span class="inline-block bg-destructive text-white text-xs leading-none px-1.5 py-0.5 ml-2">必須</span>
  {/if}
</label>
