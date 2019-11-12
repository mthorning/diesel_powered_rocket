<script>
  import HTTP from './HTTP.svelte'
  import { createEventDispatcher } from 'svelte'

  let titles = []
  let selectedTitle
  let dispatch = createEventDispatcher()

  function handleTitles({ detail }) {
    titles = detail.titles
  }

  function handlePost({ detail: { post } }) {
    dispatch('selectedPost', post)
    selectedTitle = undefined
  }
</script>

<HTTP url="/api/titles" on:data={handleTitles} />

{#if selectedTitle}
  <HTTP url={`/api/post/${selectedTitle}`} on:data={handlePost} />
{/if}

{#if titles.length}
  <select bind:value={selectedTitle}>
    <option value={undefined}>Select a post</option>
    {#each titles as item (item.id)}
      <option value={item.id}>{item.title}</option>
    {/each}
  </select>
{/if}
