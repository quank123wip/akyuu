<template>
    <div class="app-wrapper">
        <div class="milkdown-wrapper" ref="editor"></div>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, useTemplateRef } from 'vue';
import { Crepe } from '@milkdown/crepe';
import "@milkdown/crepe/theme/common/style.css";
import { math } from "@milkdown/plugin-math";

import "@milkdown/crepe/theme/nord.css";

const editor = useTemplateRef('editor');

onMounted(() => {

    const crepe = new Crepe({
        root: editor.value,
        defaultValue: '',
        featureConfigs: {
            [Crepe.Feature.Placeholder]: {
                text: "Write something..."
            }
        }
    });

    crepe.editor.use(math);

    crepe.create().then(() => {console.log(editor.value)});
});
</script>

<style lang="css">
.app-wrapper {
    display: flex;
}
.milkdown-wrapper {
    display: flex;
    min-height: 100vh;
}
.milkdown {
    flex-grow: 1;
    min-width: 100vw;
    min-height: 100vh;
}
</style>