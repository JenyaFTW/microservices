<template>
  <div class="h-full">
    <nav class="bg-blue-300 flex flex-row justify-between" v-if="authStore.authToken">
      <div class="flex flex-row">
        <NuxtLink to="/customers" class="px-4 py-2 text-gray-800 hover:bg-blue-400 transition duration-100 ease">Customers</NuxtLink>
        <NuxtLink to="/orders" class="px-4 py-2 text-gray-800 hover:bg-blue-400 transition duration-100 ease">Orders</NuxtLink>
        <NuxtLink to="/warehouse" class="px-4 py-2 text-gray-800 hover:bg-blue-400 transition duration-100 ease">Warehouse</NuxtLink>
      </div>
      <div>
        <button @click="authStore.logout"  class="px-4 py-2 text-gray-800 hover:bg-blue-400 transition duration-100 ease">Logout</button>
      </div>
    </nav>
    <slot />
  </div>
</template>

<script setup>
import { useAuthStore } from '~~/stores/auth';

const authStore = useAuthStore();
</script>

<style>
html, body, #__nuxt {
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
}
</style>
