<template>
    <input :value="props.modelValue" @input="emit('update:modelValue', $event.target.value)"
        class="p-3 border border-gray-300 bg-gray-200 outline-none focus:outline-none w-full mb-3 text-gray-700 text-sm rounded-xl">
</template>

<script setup>
const props = defineProps(['modelValue'])
const emit = defineEmits(['update:modelValue'])
</script>