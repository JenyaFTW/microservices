<template>
  <div v-if="doesntExist" class="lg:px-24 lg:py-24 md:py-20 md:px-44 px-4 py-24 items-center flex justify-center flex-col-reverse lg:flex-row md:gap-28 gap-16">
    <div class="xl:pt-24 w-full xl:w-1/2 relative pb-12 lg:pb-0">
      <div class="relative">
        <div class="absolute">
          <div class="">
            <h1 class="my-2 text-gray-800 dark:text-indigo-600 font-bold text-2xl">
              This item does not exist
            </h1>
            <p class="my-2 text-gray-800 dark:text-indigo-600">Sorry about that! Go back to the warehouse page to view all warehouse items.</p>
            <button @click="navigateTo('/warehouse')" class="sm:w-full lg:w-auto my-2  rounded md py-4 px-8 text-center bg-indigo-600 text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-700 focus:ring-opacity-50">Take me there!</button>
          </div>
        </div>
        <div>
          <img src="https://i.ibb.co/G9DC8S0/404-2.png" />
        </div>
      </div>
    </div>
    <div>
      <img src="https://i.ibb.co/ck1SGFJ/Group.png" />
    </div>
  </div>
  <div v-else class="flex flex-col justify-center items-center">
    <div class="container mx-auto bg-white mt-10 rounded px-4">
      <div class="xl:w-full border-b border-gray-300 py-5">
        <div class="flex w-11/12 mx-auto xl:w-full xl:mx-0 items-center">
          <p class="text-lg text-gray-800 font-bold">Item #{{ route.params.id }}</p>
          <div class="ml-2 cursor-pointer text-gray-600">
            <img src="https://tuk-cdn.s3.amazonaws.com/can-uploader/simple_form-svg4.svg" alt="info">
            <img class="hidden" src="https://tuk-cdn.s3.amazonaws.com/can-uploader/simple_form-svg4dark.svg" alt="info">
          </div>
        </div>
      </div>
      <div class="mx-auto pt-4">
        <div class="container mx-auto">
          <form class="my-6 w-11/12 mx-auto xl:w-full xl:mx-0">
            <div class="xl:w-1/4 lg:w-1/2 md:w-1/2 flex flex-col mb-6">
              <label for="Name" class="pb-2 text-sm font-bold text-gray-800">Name</label>
              <input tabindex="0" type="text" id="Name" name="name" v-model="name" required class="border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" placeholder="" />
            </div>
            <div class="xl:w-1/4 lg:w-1/2 md:w-1/2 flex flex-col mb-6">
              <label for="Price" class="pb-2 text-sm font-bold text-gray-800">Price $</label>
              <input tabindex="0" type="number" id="Price" name="price" v-model="price" required class="border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" placeholder="" />
            </div>
          </form>
        </div>
      </div>
    </div>
    <div class="container mx-auto w-11/12 xl:w-full">
      <div class="w-full py-4 sm:px-0 bg-white flex justify-end font-medium">
        <button role="button" @click="navigateTo('/warehouse')" aria-label="cancel form" class="bg-gray-200 focus:outline-none transition duration-150 ease-in-out hover:bg-gray-300 rounded text-indigo-600 px-6 py-2 text-sm mr-4 focus:ring-2 focus:ring-offset-2 focus:ring-gray-700">Back</button>
        <button role="button" @click="saveItem" aria-label="Save form" class="focus:ring-2 focus:ring-offset-2 focus:ring-indigo-700 bg-indigo-700 focus:outline-none transition duration-150 ease-in-out hover:bg-indigo-600 rounded text-white px-8 py-2 text-sm" type="submit">Save</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import axios from "axios";
import {navigateTo} from "#app";

const config = useRuntimeConfig();
const doesntExist = ref(false);
const route = useRoute();
const name = ref(null);
const price = ref(null);

onMounted(async () => {
  const { data } = await axios.get(`${config.public.api}api/warehouse/get/${route.params.id}`);
  if (!data) doesntExist.value = true;
  name.value = data.name;
  price.value = data.price;
});

const saveItem = async () => {
  const data = {
    id: route.params.id,
    name: name.value,
    price: price.value
  };

  const res = await axios.post(`${config.public.api}api/warehouse/update/${route.params.id}`, data);
  if (res.status === 200) {
    navigateTo('/warehouse');
  }
};
</script>
