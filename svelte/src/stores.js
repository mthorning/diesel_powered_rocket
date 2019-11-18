import { writable, derived } from 'svelte/store'
import axios from 'axios'

export const selectedPostId = writable(undefined)
export const selectedPost = derived(
  selectedPostId,
  ($selectedPostId, set) => {
    if ($selectedPostId) {
      axios.get(`/api/post/${$selectedPostId}`).then(({ data }) => set(data))
    } else {
      set({})
    }
  },
  { loading: true }
)

export const titles = (function() {
  const store = writable([])
  function fetch() {
    axios.get('/api/titles').then(({ data }) => store.set(data))
  }
  return { ...store, fetch }
})()
