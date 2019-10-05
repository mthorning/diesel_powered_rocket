<script>
  import { onMount } from 'svelte'

  onMount(() => {
    fetch('/api/titles')
      .then(res => console.log(res))
      .catch(err => console.error(err))
  })
</script>

<p>Test:</p>
