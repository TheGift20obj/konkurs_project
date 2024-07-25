<template>
  <div class="max-w-md mx-auto bg-white p-8 rounded-lg shadow-lg mt-8">
    <h2 class="text-2xl font-bold mb-6 text-center">Formularz rejestracji</h2>
    <form @submit.prevent="register">
      <div class="mb-4">
        <label class="block text-gray-700 mb-2">Nazwa użytkownika:</label>
        <input
          type="text"
          v-model="username"
          required
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
        />
      </div>
      <div class="mb-4">
        <label class="block text-gray-700 mb-2">Hasło:</label>
        <input
          type="password"
          v-model="password"
          required
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
        />
      </div>
      <div class="mb-4">
        <label for="gmail" class="block text-gray-700 mb-2">Gmail:</label>
        <input
          v-model="gmail"
          type="text"
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"
        />
      </div>
      <div class="flex justify-center">
        <button
          type="submit"
          class="w-72 bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
        >
          Zarejestruj
        </button>
      </div>
    </form>
  </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

export default {
  data() {
    return {
      username: "",
      password: "",
      gmail: ""
    };
  },
  methods: {
    async register() {
      await dzien2_backend.dodaj_uzytkownika(this.username, this.password, "user", this.gmail)
        .then((response) => {
          alert(response);
          this.username = "";
          this.password = "";
          this.gmail = "";
        })
        .catch((error) => {
          console.error('Błąd podczas rejestracji:', error);
          alert('Wystąpił błąd podczas rejestracji.');
        });
    }
  }
};
</script>