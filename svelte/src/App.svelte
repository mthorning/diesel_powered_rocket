<script>
    import BlogForm from './components/BlogForm.svelte';
    import PostViewer from './components/PostViewer.svelte';

    let showForm = false;

</script>

<style>
	h1 {
		color: purple;
	}
	.app {
	    max-width: 1000px;
	    margin: auto;
	}
</style>

<div class="app">
    <h1>Cruddy Blog Site</h1>

    {#if showForm}
	<BlogForm hideForm={() => showForm = false}/>
    {:else}
	<button on:click={() => showForm = true}>Add Post</button>
    {/if}

    <PostViewer  />
</div>
