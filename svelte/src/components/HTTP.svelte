<script>
    import axios from 'axios';
    import { onMount, createEventDispatcher } from 'svelte';

    export let url;
    export let data = {};
    export let method = 'get';

    let dispatch = createEventDispatcher();

    onMount(() => {
	axios[method](url, data)
	    .then(( { data } ) => {
		dispatch('data', data)
	    });
    });
</script>
