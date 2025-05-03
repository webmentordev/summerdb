<template>
    <button class="py-2 px-3 bg-dark text-white font-medium rounded-xl cursor-pointer">
        {{ text }}
    </button>
</template>
<script setup>
defineProps(["text"])
</script>