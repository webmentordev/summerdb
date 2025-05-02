<template>
    <section>
        <span v-if="message">{{ message }}</span>
    </section>
</template>

<script setup>
const message = ref(null);
const { data } = await useFetch('/api/setup');
message.value = data.value.message;
</script>