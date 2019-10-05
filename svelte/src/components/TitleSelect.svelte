<script>
    import { onMount } from 'svelte';
    onMount(() => {
	fetch('api/titles')
	    .then(r => r.json())
	    .then(data => console.log(data));
    });
</script>

<p>Test</p>
