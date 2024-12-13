<template>
    <div class="app-wrapper relative">
        <div class="milkdown header">
            <div class="header-top">
                <input class="header-input" :disabled="readonly" placeholder="Title here..." v-model="title"></input>
                <div class="top-btn-wrapper">
                    <!---<button class="lock-btn hover:bg-gray-200 rounded-md" @click="lock">
                        <div class="i-line-md-cog-filled hover:i-line-md-cog-filled-loop"></div>
                    </button>-->
                </div>
            </div>
        </div>
        <div class="milkdown-wrapper" ref="editor"></div>
        <div class="status-wrapper">
            <div class="rounded-full pa-2 shadow animated">
                <div class="i-line-md-loading-alt-loop text-xl" v-show="loading"></div>
                <div class="i-line-md-check-all text-xl color-green-600" v-show="!loading"></div>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, useTemplateRef, computed, watch } from 'vue';
import { Crepe } from '@milkdown/crepe';
import { automd } from '@milkdown/plugin-automd';
import "@milkdown/crepe/theme/common/style.css";
// import { math } from "@milkdown/plugin-math";

import { listener, listenerCtx } from '@milkdown/kit/plugin/listener';

import "@milkdown/crepe/theme/nord.css";
import useErrorStore from '@/stores/error';
import router from '@/router';

import { useDebounceFn } from '@vueuse/core'
import { Status, useStatusStore } from '@/stores/status';

const editor = useTemplateRef('editor');

const updatePost = useDebounceFn(() => {
    fetch(`/api/${id}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            title: title.value,
            text: text.value
        })
    }).then((res) => {
        if (res.ok) {
            status.finishLoading();
        }
        else {
            error.setErrorMessage(res.statusText);
            router.push('/error')
        }
    })
}, 1000, { maxWait: 5000 }); // Adjust the delay as needed

const props = defineProps<{
    id?: string
}>();

const id = props.id;

const title = ref('');

const text = ref('');

const readonly = ref(false);

enum PostPermission {
    Public = 'Public',
    Private = 'Private',
    Restricted = 'Restricted',
}

interface PostResponse {
    title: string,
    text: string,
    author?: string,
    author_email?: string
    permission: PostPermission,
    updated_at: Date
    created_at: Date,
}

let crepe;

const status = useStatusStore();

const error = useErrorStore();

const loading = computed(() =>
    status.status === Status.Loading
)

const makeId = (length: number) => {
    let idOptions = '1234567890abcdefghijklmnopqrstuvwxyz0123456789';
    let result = '';

    for (let i = 0; i < length; i++)
        result += idOptions[Math.floor(Math.random() * idOptions.length)];
    return result;
}

onMounted(async () => {

    console.log(id)

    if (!id) {
        router.push('/' + makeId(10));
    }

    status.beginLoading();


    error.clearErrorMessage();

    const postData: PostResponse = await fetch(`/api/${id}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
    }).then(async (val) => {
        if (val.ok)
            return val.json();
        else if (val.status === 404) {
            const newPost = await fetch(`/api/${id}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    title: "",
                    text: "",
                    permission: PostPermission.Public,
                })
            }).then(async (val) => {
                if (val.ok)
                    return {
                        title: "",
                        text: "",
                        permission: PostPermission.Public,
                    };
                else {
                    error.setErrorMessage("An unexpected error occurred.");
                    router.push('/error');
                }
            });
            return newPost;
        } else {
            error.setErrorMessage("An unexpected error occurred.");
            router.push('/error');
        }
    });

    console.log(postData);

    title.value = postData.title;

    text.value = postData.text;

    watch(title, (newTitle, oldTitle) => {
        if (newTitle !== oldTitle) {
            status.beginLoading();
            updatePost();
        }
    })

    crepe = new Crepe({
        root: editor.value,
        defaultValue: postData.text,
        featureConfigs: {
            [Crepe.Feature.Placeholder]: {
                text: "Write something..."
            }
        }
    });

    crepe.editor
        .use(automd)
        .config((ctx) => {
            ctx.get(listenerCtx).markdownUpdated(
                (_, markdown) => {
                    text.value = markdown;
                    status.beginLoading();
                    updatePost();
                }
            );
        })
        .use(listener);

    await crepe.create().then(() => { console.log(editor.value) });
    status.finishLoading();
});
</script>

<style lang="css">
.app-wrapper {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
}

.lock-btn {
    padding: .5rem;
    border: 1px outset #11111122 !important;
}

.header {
    display: flex;
    flex-direction: column;
    padding: 40px 120px 10px 120px;
    font-family: var(--crepe-font-default);
    color: var(--crepe-color-on-background);
    background: var(--crepe-color-background);
}

.header-top {
    display: flex;
    position: relative;
}

.top-btn-wrapper {
    position: absolute;
    right: 0;

}

.header-input {
    border: none;
    background-color: transparent;

    font-size: 48px;
    font-family: var(--crepe-font-title);
    font-weight: 400;
    padding: 2px 0;
}

::placeholder {
    color: color-mix(in srgb, #1B1C1D, transparent 60%);
    ;
}

::-ms-input-placeholder {
    /* Edge 12 -18 */
    color: color-mix(in srgb, #1B1C1D, transparent 60%);
}

.header-input:focus-visible {
    outline: none;
}

.milkdown-wrapper {
    display: flex;
    flex-grow: 1;
}

.milkdown-wrapper>.milkdown {
    flex-grow: 1;
    min-width: 100vw;

}

.milkdown .ProseMirror {
    padding: 20px 120px 20px 120px;
}

.status-wrapper {
    position: absolute;
    bottom: 1rem;
    right: 1rem;
    z-index: 1000;
}
</style>