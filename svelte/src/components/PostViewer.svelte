<script>
    import PostSelector from './PostSelector.svelte';

    export let setEditPost;
    export let deletePost;
    export let posts = [];
    export let selectedPost;
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
	color: rgb(0,80,160);
	cursor: pointer;
    }
</style>

<div class="container">
<h2>Blog Posts</h2>

<PostSelector />

{#if posts.length}
    <select bind:value={selectedPost}>
	<option>Select a post</option>
	{#each posts as post (post.id)}
	    <option value={post.id}>{post.title}</option>
	{/each}
    </select>
{:else}
    <p>No posts.</p>
{/if}

{#if selectedPost}
    <pre>{posts.find(post => post.id === selectedPost).body}</pre>
    <span class="link" on:click={() => setEditPost(selectedPost)}>edit</span>
    <span> | </span>
    <span class="link" on:click={() => deletePost(selectedPost)}>delete</span>
{/if}
</div>

