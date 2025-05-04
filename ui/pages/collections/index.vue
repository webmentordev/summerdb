<template>
    <section class="relative h-full flex">
        <div class="flex flex-col w-full">
            <div class="flex items-center justify-between mb-3">
                <h2 class="flex text-2xl">Collections</h2>
                <FormButton text="+ New collection" @click="isSidebarOpen = !isSidebarOpen" />
            </div>
        </div>
        <SideBar v-model="isSidebarOpen">
            <FormInput v-model="title" placeholder="Collection title" />
            <div class="py-3 border-t border-gray-200">
                <FormButton text="+ New Column" class="w-full" />
            </div>
            <FormButton text="Create" @click="store" />
        </SideBar>
    </section>
</template>

<script setup>
const isSidebarOpen = ref(true);

const title = ref("");

const fields = ref([
    {
        title: "title",
        type: "VARCHAR",
        max: 255,
    },
    {
        title: "content",
        type: "TEXT"
    },
    {
        title: "is_active",
        type: "BOOLEAN"
    }
]);


async function store() {
    const data = await $fetch('/api/create/collection', {
        method: "POST",
        body: {
            "collection": title.value,
            "fields": fields.value
        }
    });
    reset();
    console.log(data);
}


function reset() {

}

</script>