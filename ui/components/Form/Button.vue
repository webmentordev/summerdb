<template>
    <button class="py-2 px-5 bg-dark text-white font-medium rounded-md cursor-pointer">
        {{ text }}
    </button>
</template>
<script setup>
defineProps(["text"])
</script>