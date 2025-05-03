<template>
    <nuxt-link :to="link" class="px-3 rounded-xl py-3 pl-4 flex items-center text-slate-200 mb-3">
        <Icon :name="icon" class="text-2xl" />
        <span class="ml-3 font-medium text-md">{{ text }}</span>
    </nuxt-link>
</template>

<script setup>
defineProps(['icon', 'text', 'link'])
</script>