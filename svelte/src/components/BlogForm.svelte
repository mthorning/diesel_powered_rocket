<script>
  import { titles } from '../stores.js'
  import axios from 'axios'

  export let hideForm
  export let formValues = {
    title: '',
    body: '',
  }

  async function submitHandler() {
    await axios.post('api/new', formValues)
    titles.fetch()
    hideForm()
  }
</script>

<style>
  div {
    display: flex;
    flex-direction: row;
    align-items: baseline;
    justify-content: space-evenly;
    width: 400px;
  }
  .submit {
    justify-content: flex-end;
  }
</style>

<button on:click={hideForm}>Hide Form</button>

<form on:submit|preventDefault={submitHandler}>
  <div>
    <label>Title</label>
    <input bind:value={formValues.title} />
  </div>
  <div>
    <label>Body</label>
    <textarea bind:value={formValues.body} />
  </div>
  <div class="submit">
    <input type="submit" />
  </div>
</form>
