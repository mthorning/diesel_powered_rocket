<script>
  import { onMount } from 'svelte'
  import { titles, selectedPostId } from '../stores'

  onMount(() => {
    titles.fetch()
  })
  $: console.log($selectedPostId)
</script>

{#if $titles.length}
  <select bind:value={$selectedPostId}>
    <option value={undefined}>Select a post</option>
    {#each $titles as item (item.id)}
      <option value={item.id}>{item.title}</option>
    {/each}
  </select>
{/if}
