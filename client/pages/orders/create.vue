<template>
  <div class="flex flex-col justify-center items-center">
    <div class="container mx-auto bg-white mt-10 rounded px-4">
      <div class="xl:w-full border-b border-gray-300 py-5">
        <div class="flex w-11/12 mx-auto xl:w-full xl:mx-0 items-center">
          <p class="text-lg text-gray-800 font-bold">Order Information</p>
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
              <label for="CustomerId" class="pb-2 text-sm font-bold text-gray-800">Customer ID</label>
              <input tabindex="0" type="number" id="CustomerId" name="customerId" v-model="customerId" required class="border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" placeholder="" />
            </div>
            <div class="xl:w-1/4 lg:w-1/2 md:w-1/2 flex flex-col space-y-2 mb-6">
              <label for="FirstName" class="pb-2 text-sm font-bold text-gray-800">Item IDs</label>
              <div class="flex flex-row space-x-2">
                <input tabindex="0" type="number" ref="itemId" required class="w-full border border-gray-300 pl-3 py-3 shadow-sm bg-transparent rounded text-sm focus:outline-none focus:border-indigo-700 placeholder-gray-500 text-gray-600" />
                <button role="button" @click="addItemId" class="focus:ring-2 focus:ring-offset-2 focus:ring-indigo-700 bg-indigo-700 focus:outline-none transition duration-150 ease-in-out hover:bg-indigo-600 rounded text-white px-4 py-2 text-sm font-medium">Add</button>
              </div>
              <div class="flex flex-row w-full flex-wrap text-gray-600 gap-2">
                <div v-for="(itemId, idx) in itemIds" :key="idx" @click="itemIds.delete(itemId)" class="px-3 py-1 bg-gray-50 rounded-md text-sm font-semibold hover:bg-gray-100 cursor-pointer">
                  {{ itemId }}
                </div>
              </div>
            </div>
          </form>
        </div>
      </div>
    </div>
    <div class="container mx-auto w-11/12 xl:w-full">
      <div class="w-full py-4 sm:px-0 bg-white flex justify-end font-medium">
        <button role="button" @click="navigateTo('/orders')" aria-label="cancel form" class="bg-gray-200 focus:outline-none transition duration-150 ease-in-out hover:bg-gray-300 rounded text-indigo-600 px-6 py-2 text-sm mr-4 focus:ring-2 focus:ring-offset-2 focus:ring-gray-700">Back</button>
        <button role="button" @click="createCustomer" aria-label="Save form" class="focus:ring-2 focus:ring-offset-2 focus:ring-indigo-700 bg-indigo-700 focus:outline-none transition duration-150 ease-in-out hover:bg-indigo-600 rounded text-white px-8 py-2 text-sm" type="submit">Create</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import axios from "axios";
import {navigateTo} from "#app";

const config = useRuntimeConfig();
const customerId = ref(null);
const itemId = ref(null);
const itemIds = ref(new Set());

const addItemId = () => {
  if (itemId.value.value) {
    itemIds.value.add(itemId.value.value);
    itemId.value.value = null;
  }
};

const createCustomer = async () => {
  const data = {
    clientId: customerId.value,
    itemId: Array.from(itemIds.value)
  };

  const res = await axios.post(`${config.public.api}api/orders/create`, data);
  if (res.status === 200) {
    navigateTo('/orders');
  }
};
</script>
