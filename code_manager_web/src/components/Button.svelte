<script lang="ts">
  import cn from 'clsx';
  import { twMerge } from 'tailwind-merge';

  interface ButtonProps {
    variant?: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link';
    size?: 'default' | 'sm' | 'lg' | 'icon';
    disabled?: boolean;
    loading?: boolean;
    class?: string;
    children?: any;
    [key: string]: any;
  }

  let { 
    variant = 'default', 
    size = 'default', 
    disabled = false, 
    loading = false,
    class: className = '',
    children,
    ...props 
  }: ButtonProps = $props();

  let waveData = $state<{ x: number; y: number; active: boolean }>({ x: 0, y: 0, active: false });

  const variantStyles: Record<string, string> = {
    default: 'bg-primary text-primary-foreground hover:bg-primary/90 shadow-lg shadow-primary/25 backdrop-blur-sm bg-gradient-to-br from-primary via-primary to-primary/95',
    destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
    outline: 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
    secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
    ghost: 'hover:bg-accent hover:text-accent-foreground',
    link: 'text-primary underline-offset-4 hover:underline'
  };

  const sizeStyles: Record<string, string> = {
    default: 'h-10 px-4 py-2',
    sm: 'h-9 rounded-md px-3',
    lg: 'h-11 rounded-md px-8',
    icon: 'h-10 w-10'
  };

  const baseStyles = 'cursor-pointer inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 relative overflow-hidden';
  
  const classes = twMerge(
    cn(
      baseStyles,
      variantStyles[variant],
      sizeStyles[size],
      className
    )
  );

  function handleClick(event: MouseEvent) {
    if (disabled || loading) return;
    
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;
    
    waveData = { x, y, active: true };
    
    setTimeout(() => {
      waveData.active = false;
    }, 600);
  }
</script>

<button
  class={classes}
  disabled={disabled || loading}
  onclick={handleClick}
  {...props}
>
  {#if waveData.active}
    <div 
      class="absolute pointer-events-none rounded-full bg-white/30 animate-wave"
      style="left: {waveData.x}px; top: {waveData.y}px; transform: translate(-50%, -50%);"
    ></div>
  {/if}
  {#if loading}
    <div class="absolute inset-0 overflow-hidden">
      <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent animate-loading-wave"></div>
    </div>
  {/if}
  {@render children?.()}
</button>


<style>

@keyframes wave {
  0% {
    width: 0;
    height: 0;
    opacity: 0.8;
  }
  100% {
    width: 300px;
    height: 300px;
    opacity: 0;
  }
}

@keyframes loading-wave {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.animate-wave {
  animation: wave 0.6s ease-out forwards;
}

.animate-loading-wave {
  animation: loading-wave 1.5s ease-in-out infinite;
}
</style>
