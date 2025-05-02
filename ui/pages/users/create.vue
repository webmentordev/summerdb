<template>
    <section>
        <form @submit.prevent="store" method="POST">
            <div v-if="user" class="max-w-[400px] w-full m-auto mt-6">
                <Input v-model="name" placeholder="User Name" required />
                <Input v-model="age" placeholder="Age" required />
                <Input v-model="street" placeholder="Address" required />
                <Input v-model="postal_code" placeholder="Postal Code" required />
                <button type="submit" class="py-2 px-3 rounded-md bg-black text-white">Create</button>
            </div>
        </form>
    </section>
</template>

<script setup>
const user = ref(null);

const name = ref('');
const age = ref('');
const street = ref('');
const postal_code = ref('');


const { data } = await useFetch('/api/users');
user.value = data.value;


async function store() {
    const data = await $fetch('/api/create/user', {
        method: "POST",
        body: {
            name: name.value,
            age: age.value,
            street: street.value,
            postal_code: postal_code.value,
        }
    });
    console.log(data);
}
</script>