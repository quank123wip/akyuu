import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

const useErrorStore = defineStore('error', () => {
  const errorMessage = ref<string | null>(null)

  const setErrorMessage = (message: string) => {
    errorMessage.value = message
  }
  const clearErrorMessage = () => {
    errorMessage.value = null
  }
  return { errorMessage, setErrorMessage, clearErrorMessage }
})

export default useErrorStore;
