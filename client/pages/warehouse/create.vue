<template>
  <div class="flex flex-col justify-center items-center">
    <div class="container mx-auto bg-white mt-10 rounded px-4">
      <div class="xl:w-full border-b border-gray-300 py-5">
        <div class="flex w-11/12 mx-auto xl:w-full xl:mx-0 items-center">
          <p class="text-lg text-gray-800 font-bold">Item Information</p>
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
        <button role="button" @click="createItem" aria-label="Save form" class="focus:ring-2 focus:ring-offset-2 focus:ring-indigo-700 bg-indigo-700 focus:outline-none transition duration-150 ease-in-out hover:bg-indigo-600 rounded text-white px-8 py-2 text-sm" type="submit">Create</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import axios from "axios";
import {navigateTo} from "#app";

const config = useRuntimeConfig();
const name = ref(null);
const price = ref(null);

const createItem = async () => {
  const data = {
    name: name.value,
    price: price.value
  };

  const res = await axios.post(`${config.public.api}api/warehouse/create`, data);
  if (res.status === 200) {
    navigateTo('/warehouse');
  }
};
</script>
