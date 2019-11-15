<script>
  import { onDestroy } from 'svelte'
  import TitleSelect from './TitleSelect.svelte'
  import { selectedPostId } from '../stores.js'
  import axios from 'axios'

  // if selectedPostId then we need to call the API to
  // get the post:
  let selectedPost = {}

  const unsubscribe = selectedPostId.subscribe(id => {
    if (id) {
      axios
        .get(`/api/post/${id}`)
        .then(({ data: { post } }) => (selectedPost = post))
    }
  })

  onDestroy(unsubscribe)

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

  {#if Object.keys(selectedPost).length}
    <h2>{selectedPost.title}</h2>
    <pre>{selectedPost.body}</pre>
    <span class="link" on:click={() => setEditPost(selectedPost)}>edit</span>
    <span>|</span>
    <span class="link" on:click={() => deletePost(selectedPost)}>delete</span>
  {/if}
</div>
