export default defineEventHandler(async (event) => {
    const api = useRuntimeConfig(event).api;
    const result = await $fetch(`${api}/api/setup`);
    return result;
})