<script>
  import TitleSelect from './TitleSelect.svelte'
  import { titles, selectedPost, selectedPostId } from '../stores.js'
  import axios from 'axios'

  const setEditPost = () => {
    return null
  }
  const deletePost = async () => {
    await axios.delete(`/api/delete/${$selectedPostId}`)
    titles.fetch()
    selectedPostId.set(undefined)
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
