<script>
    import BlogForm from './components/BlogForm.svelte';
    import PostViewer from './components/PostViewer.svelte';

    let showForm = false;
    let posts = [
	{ id: 1, title: 'Post one', body: 'This is the first post' },
	{ id: 2, title: 'A follow up', body: 'The difficult second blog post' }
    ];
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
