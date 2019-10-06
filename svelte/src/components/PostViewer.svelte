<script>
  import TitleSelect from './TitleSelect.svelte'

  export let setEditPost
  export let deletePost
  export let posts = []
  let selectedPost
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
  <h2>Blog Posts!</h2>

  <TitleSelect />

  {#if selectedPost}
    <pre>{posts.find(post => post.id === selectedPost).body}</pre>
    <span class="link" on:click={() => setEditPost(selectedPost)}>edit</span>
    <span>|</span>
    <span class="link" on:click={() => deletePost(selectedPost)}>delete</span>
  {/if}
</div>
