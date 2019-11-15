<script>
  import axios from 'axios'
  import { onMount } from 'svelte'
  import { titles, selectedPostId } from '../stores'

  onMount(() => {
    axios.get('/api/titles').then(({ data }) => titles.set(data.titles))
  })

  function handleTitleSelection({ target: { value } }) {
    selectedPostId.set(value)
  }
</script>

{#if $titles.length}
  <select on:change={handleTitleSelection}>
    <option value={undefined}>Select a post</option>
    {#each $titles as item (item.id)}
      <option value={item.id}>{item.title}</option>
    {/each}
  </select>
{/if}
