<template>
    <section class="relative h-full flex">
        <div class="flex flex-col w-full">
            <div class="flex items-center justify-between mb-3">
                <h2 class="flex text-2xl">Collections</h2>
                <FormButton text="+ New collection" @click="isSidebarOpen = !isSidebarOpen" />
            </div>
        </div>
        <SideBar v-model="isSidebarOpen">
            <FormInput placeholder="Collection Name" />
            <div class="py-3 border-t border-gray-200">
                <FormButton text="+ New Column" class="w-full" />
            </div>
        </SideBar>
    </section>
</template>

<script setup>
const items = ref(['Backlog', 'Todo', 'In Progress', 'Done']);
const value = ref('Backlog');
const isSidebarOpen = ref(true)
</script>