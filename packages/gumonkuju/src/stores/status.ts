import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export enum Status {
    Loading,
    Ready
}

export const useStatusStore = defineStore('status', () => {
  const status = ref<Status>(Status.Ready)

  const beginLoading = () => {
    status.value = Status.Loading
  }

  const finishLoading = () => {
    status.value = Status.Ready
  }

  return { status, beginLoading, finishLoading }
});

