<script lang="ts">
  import { Check, ChevronDown } from 'lucide-svelte';
  import { createSelect, melt } from '@melt-ui/svelte';
  import { fade } from 'svelte/transition';
  import cn from 'clsx';
  import { twMerge } from 'tailwind-merge';

  interface SelectOption {
    value: number;
    label: string;
  }

  interface SelectProps {
    options: SelectOption[];
    selected: SelectOption | null;
    onSelectedChange: (option: SelectOption | null) => void;
    placeholder?: string;
    labelText?: string;
  }

  const { options, selected, onSelectedChange, placeholder = '', labelText = '' }: SelectProps = $props();

  const {
    elements: { trigger, menu, option, label },
    states: { open },
  } = createSelect<number>({
    defaultSelected: selected || undefined,
    forceVisible: true,
    onValueChange: ({ next }) => onSelectedChange(next),
    positioning: {
      placement: 'bottom',
      fitViewport: true,
      sameWidth: true,
    },
  });
</script>

<div class="flex flex-col gap-2">
  {#if labelText}
    <label class="block text-sm font-medium text-foreground" use:melt={$label}>{labelText}</label>
  {/if}
  <button
    class={twMerge(
      cn(
        'cursor-pointer',
        'flex h-10 min-w-[220px] items-center justify-between rounded-md bg-card text-card-foreground',
        'px-3 py-2 border border-input shadow-sm transition-colors',
        'hover:bg-accent/20 hover:text-foreground focus:outline-none',
        'focus:ring-2 focus:ring-ring focus:border-ring disabled:opacity-50 disabled:cursor-not-allowed'
      )
    )}
    use:melt={$trigger}
    aria-label="Select option"
  >
    {selected?.label || placeholder}
    <ChevronDown class="size-4 text-muted-foreground" />
  </button> 
  {#if $open}
    <div
      class={twMerge(
        cn(
          'z-50 flex max-h-[300px] flex-col overflow-y-auto rounded-md bg-card',
          'border border-input shadow-lg backdrop-blur-sm p-1'
        )
      )}
      use:melt={$menu}
      transition:fade={{ duration: 150 }}
    >
      {#each options as item}
        <div
          class={twMerge(
            cn(
              'relative cursor-pointer rounded-md py-2 pl-8 pr-4 text-sm transition-colors',
              'text-foreground focus:outline-none',
              'data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed'
            )
          )}
          use:melt={$option({ value: item.value, label: item.label })}
        >
          <div class="absolute left-2 top-1/2 -translate-y-1/2 flex items-center justify-center w-4 h-4">
            {#if selected?.value === item.value}
              <Check class="size-4 text-primary" />
            {/if}
          </div>
          {item.label}
        </div>
      {/each}
    </div>
  {/if}
</div>