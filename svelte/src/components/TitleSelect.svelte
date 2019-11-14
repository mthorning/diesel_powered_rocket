<script>
  import axios from 'axios'
  import { createEventDispatcher, onMount } from 'svelte'
  import { titles, selectedPostId } from '../stores'

  let selectedTitle
  let dispatch = createEventDispatcher()

  onMount(() => {
    axios.get('/api/titles').then(({ data }) => titles.set(data))
  })

  function handleTitleSelection(stuff) {
    console.log(id)
    selectedPostId.set(id)
  }
</script>

{#if titles.length}
  <select onSelect={handleTitleSelection}>
    <option value={undefined}>Select a post</option>
    {#each titles as item (item.id)}
      <option value={item.id}>{item.title}</option>
    {/each}
  </select>
{/if}
