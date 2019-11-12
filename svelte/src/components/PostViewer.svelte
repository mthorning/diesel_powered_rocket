<script>
  import TitleSelect from './TitleSelect.svelte'

  export let setEditPost
  export let deletePost
  let selectedPost

  function handleSelectedPost({ detail }) {
    console.log(arguments)
    selectedPost = detail
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

  <TitleSelect on:selectedPost={handleSelectedPost} />

  {#if selectedPost}
    <h2>{selectedPost.title}</h2>
    <pre>{selectedPost.body}</pre>
    <span class="link" on:click={() => setEditPost(selectedPost)}>edit</span>
    <span>|</span>
    <span class="link" on:click={() => deletePost(selectedPost)}>delete</span>
  {/if}
</div>
