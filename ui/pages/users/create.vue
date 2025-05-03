<template>
    <section>
        <form @submit.prevent="store" method="POST">
            <div v-if="user" class="max-w-[400px] w-full m-auto mt-6">
                <FormInput v-model="name" placeholder="User Name" required />
                <FormInput v-model="age" placeholder="Age" required />
                <FormInput v-model="street" placeholder="Address" required />
                <FormInput v-model="postal_code" placeholder="Postal Code" required />
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
    reset();
    console.log(data);
}


function reset() {
    name.value = ""
    age.value = ""
    street.value = ""
    postal_code.value = ""
}
</script>