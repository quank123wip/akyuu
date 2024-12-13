<template>
    <div class="error-page">
        <h1>🚧Error🚧</h1>
        <h3>Sorry, but something was wrong...</h3>
        <span class="error-message">{{ error }}</span>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import useErrorStore from '@/stores/error';
const errorStore = useErrorStore();
const error = computed(() => errorStore.errorMessage);
</script>

<style>
.error-page {
    background: linear-gradient(90deg, rgba(244,212,255,1) 0%, rgba(255,167,123,1) 100%);
    flex-direction: column;
    min-width: 100%;
    min-height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    color: white;
    font-size: 2rem;
    text-align: center;
    overflow: hidden;
}
</style>