<script>
  import TitleSelect from './TitleSelect.svelte'
  import { selectedPost } from '../stores.js'

  const setEditPost = () => {
    return null
  }
  const deletePost = () => {
    return null
  }
</script>

<style>
  .container {
    margin: 20px;
    border: 1px solid gray;
    border-radius: 4px;
    padding: 20px;
  }
  h2 {
    text-align: center;
  }
  .link {
    color: rgb(0, 80, 160);
    cursor: pointer;
  }
</style>

<div class="container">

  <TitleSelect />

  {#if $selectedPost.loading}
    <h2>Loading...</h2>
  {/if}
  {#if $selectedPost.id}
    <h2>{$selectedPost.title}</h2>
    <pre>{$selectedPost.body}</pre>
    <span class="link" on:click={() => setEditPost()}>edit</span>
    <span>|</span>
    <span class="link" on:click={() => deletePost()}>delete</span>
  {/if}
</div>
