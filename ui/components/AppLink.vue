<template>
    <nuxt-link :to="link"
        class="p-3 rounded-xl flex items-center border border-white/10 w-fit text-slate-200 mb-3 relative group">
        <Icon :name="icon" class="text-2xl" />
        <span
            class="bg-black absolute text-white max-w-[200px] min-w-full p-1 px-2 rounded-lg left-[55px] text-sm hidden group-hover:block">{{
                text
            }}</span>
    </nuxt-link>
</template>

<script setup>
defineProps(['icon', 'text', 'link'])
</script>