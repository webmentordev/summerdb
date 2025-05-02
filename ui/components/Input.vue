<template>
    <input :value="props.modelValue"
    @input="emit('update:modelValue', $event.target.value)" class="p-2 border border-gray-300 bg-gray-100 w-full mb-3">
</template>

<script setup>
    const props = defineProps(['modelValue'])
    const emit = defineEmits(['update:modelValue'])
</script>