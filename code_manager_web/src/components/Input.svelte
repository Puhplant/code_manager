<script lang="ts">
  import cn from 'clsx';
  import { twMerge } from 'tailwind-merge';

  interface InputProps {
    id?: string;
    name?: string;
    type?: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url' | 'search';
    placeholder?: string;
    value?: string;
    required?: boolean;
    disabled?: boolean;
    error?: string;
    autoComplete?: AutoFill;
    class?: string;
    label?: string;
    onInput?: () => void;
    [key: string]: any;
  }

  let { 
    id,
    name,
    type = 'text',
    placeholder = '',
    value = $bindable(''),
    required = false,
    disabled = false,
    error = '',
    autoComplete,
    class: className = '',
    label = '',
    onInput,
    ...props 
  }: InputProps = $props();

  const baseStyles = 'focus:outline-none appearance-none block w-full px-3 py-2 border border-input rounded-md shadow-sm placeholder-muted-foreground focus:ring-ring focus:border-ring sm:text-sm disabled:opacity-50 disabled:cursor-not-allowed bg-background text-foreground';
  
  const errorStyles = 'border-destructive focus:ring-destructive focus:border-destructive';
  
  const classes = twMerge(
    cn(
      baseStyles,
      error ? errorStyles : '',
      className
    )
  );
</script>

<div>
  {#if label || error}
    <label for={id} class="block text-sm font-medium {error ? 'text-destructive' : 'text-foreground'}">
      {error || label}
    </label>
  {/if}
  <div class="mt-1">
    <input
      {id}
      {name}
      {type}
      {placeholder}
      {required}
      {disabled}
      autocomplete={autoComplete}
      bind:value
      oninput={onInput}
      class={classes}
      {...props}
    />
  </div>
</div> 