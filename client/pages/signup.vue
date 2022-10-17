<template>
  <div class="flex flex-col justify-center items-center space-y-2 h-full">
    <form class="flex flex-col space-y-2">
      <input type="text" v-model="name" id="name" placeholder="Name" class="rounded-full p-2 border border-gray-100" />
      <input type="text" v-model="email" id="email" placeholder="Email" class="rounded-full p-2 border border-gray-100" />
      <input type="password" v-model="password" id="password" class="rounded-full p-2 border border-gray-100" placeholder="Password" />
      <button @click="signup" class="rounded-full p-2 bg-blue-400 font-semibold text-white hover:bg-blue-500 transition duration-100 ease">Sign up</button>
    </form>
    <div class="font-semibold text-gray-500">
      Or <NuxtLink to="/login" class="text-blue-300 hover:text-blue-400 transition duration-100 ease">log in</NuxtLink>
    </div>
  </div>
</template>

<script setup>
import { useAuthStore } from '~~/stores/auth';

const authStore = useAuthStore();
const name = ref('');
const email = ref('');
const password = ref('');

const signup = async e => {
  e.preventDefault();

  await authStore.login(email.value, password.value);
  if (await authStore.isAuthed) {
    window.location = '/';
  }
};
</script>
