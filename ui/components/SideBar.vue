<template>
    <section v-if="isOpen" class="max-w-lg shadow-lg w-full p-4 bg-white h-full rounded-lg ml-4">
        <div class="flex justify-end w-full mb-2">
            <button @click="closeSidebar">
                <Icon name="iconamoon:close-light" class="text-2xl" />
            </button>
        </div>
        <slot />
    </section>
</template>

<script setup>
const props = defineProps({
    modelValue: {
        type: Boolean,
        default: false
    }
})
const emit = defineEmits(['update:modelValue'])

const isOpen = computed({
    get: () => props.modelValue,
    set: (value) => emit('update:modelValue', value)
})

function closeSidebar() {
    isOpen.value = false
}
</script>