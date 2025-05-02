export default defineEventHandler(async (event) => {
    const api = useRuntimeConfig().api;
    const result = await $fetch(`${api}/api/setup`);
    return result;
})