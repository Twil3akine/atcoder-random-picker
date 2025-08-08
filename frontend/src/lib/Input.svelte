<!--
@component
## 概要
- 1行のテキストを入力する際に使用されるコンポーネントです

## 機能
- 見た目を変更するためのいくつかのスタイル用Propsが追加されています(詳細はPropsセクションを参照)

## Props
- type: inputのtypeを指定できます
- placeholder: プレースホルダーの文言を指定できます
- isError: true の場合エラー時のスタイルを適用します
- readonly: 指定すると読み取り専用の状態です
- disabled: 指定するとグレーアウトされ、クリック不可になります

## Usage
```svelte
<Input type="text" placeholder="プレースホルダー" bind:value />
```
-->

<script module lang="ts">
  import { cva, type VariantProps } from 'class-variance-authority';
  import type { HTMLInputAttributes, HTMLInputTypeAttribute } from 'svelte/elements';

  export const inputVariants = cva(
    'w-full min-h-10 px-2.75 py-1.75 border border-base-stroke-default rounded-md text-base-foreground-default text-sm outline-primary transition-colors placeholder:text-base-foreground-muted hover:bg-base-container-accent/90 active:border-base-stroke-default focus-visible:outline-primary focus-visible:outline-[0.125rem] focus-visible:outline-offset-[0.125rem]',
    {
      variants: {
        /** type属性 */
        type: {
          text: [],
          file: ['text-base-foreground-muted file:text-base-foreground-default file:px-2 file:mr-0'],
        } as Partial<Record<HTMLInputTypeAttribute, string[]>>,
        /** 操作できるかどうか */
        disabled: {
          true: ['opacity-50 cursor-not-allowed'],
          false: ['cursor-text'],
        },
        /** 読み取り専用かどうか */
        readonly: {
          true: ['opacity-50'],
          false: [],
        },
        /** エラーかどうか */
        isError: {
          true: ['border-destructive'],
          false: [],
        },
      },
      compoundVariants: [
        {
          type: 'file',
          readonly: true,
          class: 'cursor-not-allowed',
        },
      ],
      defaultVariants: {
        type: 'text',
        disabled: false,
        readonly: false,
      },
    },
  );

  export type InputVariants = VariantProps<typeof inputVariants>;

  export interface InputProps extends InputVariants, HTMLInputAttributes {
    /** クラス */
    class?: string;
  }
</script>

<script lang="ts">
  let {
    isError = false,
    class: className = '',
    value = $bindable(''),
    ...inputAttributes
  }: InputProps = $props();

  let inputVariantClass = $derived(inputVariants({
    type: inputAttributes.type,
    disabled: inputAttributes.disabled,
    readonly: inputAttributes.readonly,
    class: className,
    isError,
  }));
</script>

<input class={inputVariantClass} {...inputAttributes} bind:value />