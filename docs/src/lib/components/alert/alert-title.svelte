<script lang="ts">
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';
  import { Title } from '$components/ui/alert';

  interface Props {
    children: Snippet;
    class?: string;
  }

  const { class: className, children }: Props = $props();
</script>

<Title class={cn('pb-2', className)}>
  {@render children()}
</Title>
