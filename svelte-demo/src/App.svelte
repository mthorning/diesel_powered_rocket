<script>
    import BlogForm from './components/BlogForm.svelte';
    import PostViewer from './components/PostViewer.svelte';

    let showForm = false;
    let posts = [
	{ id: 1, title: 'Post one', body: 'This is the first post' },
	{ id: 2, title: 'A follow up', body: 'The difficult second blog post' }
    ];

    function savePost(formValues) {
	const id = posts.reduce((id, curr) => id <= curr.id ? curr.id + 1 : id, 1);
	posts = [...posts, { id, ...formValues }];
	console.log(posts);
    }
</script>

<style>
	h1 {
		color: purple;
	}
</style>

<h1>Cruddy Blog Site</h1>

{#if showForm}
    <BlogForm {savePost} hideForm={() => showForm = false}/>
{:else}
    <button on:click={() => showForm = true}>Add Post</button>
{/if}

<PostViewer {posts} />
