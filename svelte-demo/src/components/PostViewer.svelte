<script>
    export let posts = [];

    let selectedPost;
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
</style>

<div class="container">
<h2>Blog Posts</h2>
{#if posts.length}
    <select bind:value={selectedPost}>
	{#each posts as post (post.id)}
	    <option value={post.id}>{post.title}</option>
	{/each}
    </select>
{:else}
    <p>No posts.</p>
{/if}

{#if selectedPost}
    <pre>{posts.find(post => post.id === selectedPost).body}</pre>
{/if}
</div>
