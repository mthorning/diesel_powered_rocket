<script>
    import HTTP from './HTTP.svelte';

    let titles = [];
    let selectedTitle;
</script>

<HTTP url="/api/titles" on:data={({ detail }) => titles = detail} />

{#if titles.length}
    <select bind:value={selectedTitle}>
	<option value={undefined}>Select a post</option>
    {#each titles as item (item.id)}
	<option value={item.id}>{item.title}</option>
    {/each}
</select>
{/if}
