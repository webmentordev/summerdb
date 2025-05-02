<template>
    <section>
        <div v-if="version" class="max-w-[400px] w-full m-auto mt-6">
            <div class="flex justify-between p-2 bg-gray-200">
                <span>Actix:</span>
                <span>{{ version.actix }}</span>
            </div>
            <div class="flex justify-between p-2">
                <span>Serde:</span>
                <span>{{ version.serde }}</span>
            </div>
            <div class="flex justify-between p-2 bg-gray-200">
                <span>Rusqlite:</span>
                <span>{{ version.rusqlite }}</span>
            </div>
            <div class="flex justify-between p-2">
                <span>Serde Json:</span>
                <span>{{ version.serde_json }}</span>
            </div>
        </div>
        <div v-else>
            Loading...
        </div>
    </section>
</template>

<script setup>
const version = ref(null);
const { data } = await useFetch('/api/version');
version.value = data.value;
</script>