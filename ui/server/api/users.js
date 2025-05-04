export default defineEventHandler(async (event) => {
    const api = useRuntimeConfig(event).api;
    const result = await $fetch(`${api}/api/super-users`);
    return result;
})