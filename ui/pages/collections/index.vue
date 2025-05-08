<template>
    <section class="relative h-full flex">
        <div class="flex flex-col w-full">
            <div class="flex items-center justify-between mb-3">
                <h2 class="flex text-2xl">Collections</h2>
                <FormButton text="+ New collection" @click="isSidebarOpen = !isSidebarOpen" />
            </div>
        </div>
        <SideBar v-model="isSidebarOpen">
            <form @submit.prevent="store" method="post">
                <FormInput v-model="title" placeholder="Collection title" required />
                <div class="py-3 border-t border-gray-200">
                    <FormButton text="+ New Column" class="w-full" />
                </div>
                <FormButton text="Create" type="submit" />
            </form>
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
        unique: true,
        nullable: true,
        min: 8,
        max: 255,
    },
    {
        title: "content",
        type: "TEXT",
        unique: false,
        nullable: true,
    },
    {
        title: "is_active",
        type: "BOOLEAN",
        unique: false,
        nullable: true,
    }
]);


async function store() {
    if (title.value == "") return;
    try {
        const data = await $fetch('/api/create/collection', {
            method: "POST",
            body: {
                "collection": title.value,
                "fields": fields.value
            }
        });
        reset();
        console.log(data);
    } catch (e) {
        console.log(e.data.data.message);
    }
}


function reset() {
    title.value = "";
}

</script>