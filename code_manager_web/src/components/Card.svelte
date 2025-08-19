<script lang="ts">
  import cn from 'clsx';
  import { twMerge } from 'tailwind-merge';

  interface CardProps {
    class?: string;
    children?: any;
    [key: string]: any;
  }

  let { 
    class: className = '',
    children,
    ...props 
  } = $props();

  const baseStyles = 'bg-card text-card-foreground shadow-lg rounded-lg backdrop-blur-sm';
  
  const classes = twMerge(
    cn(
      baseStyles,
      className
    )
  );
</script>

<div class={classes} {...props}>
  {@render children?.()}
</div> 