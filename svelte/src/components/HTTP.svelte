<script>
    import { onMount, createEventDispatcher } from 'svelte';

    export let url;
    export let data = {};
    export let method = 'GET';

    let dispatch = createEventDispatcher();

    onMount(() => {
	console.log('data = ', data);
	const options = {
	    method,
	    mode: 'cors',
	    cache: 'no-cache',
	    credentials: 'same-origin',
	    headers: {
	      'Content-Type': 'application/json'
	    },
	    redirect: 'follow', 
	    referrer: 'no-referrer',
	};
	if(Object.keys(data).length) options.data = JSON.stringify(data);
	
	console.log(options);
	fetch(url, options)
	    .then(res => res.json())
	    .then(data => {
		console.log(data);
		dispatch('data', data)
	    });
    });
</script>
