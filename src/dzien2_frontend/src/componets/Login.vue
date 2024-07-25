<script setup>
import DeviceReports from '../componets/DeviceReports.vue'
import Rejestracja from '../componets/Rejestracja.vue'
import UserUI from '../componets/UserUI.vue'
</script>

<template>
  <div class="min-h-screen bg-gray-100 flex flex-col items-center justify-center py-12">
    <div class="max-w-5xl w-full bg-white p-8 rounded-lg shadow-md">
      <h1 class="text-3xl font-bold text-center mb-8">Witaj w aplikacji konkursowej</h1>
      <div v-if="logging">
        <div v-if="!logged">
          <div class="max-w-md mx-auto bg-white p-8 rounded-lg shadow-lg mt-8">
            <h2 class="text-2xl font-bold mb-6 text-center">Formularz logowania</h2>
            <form @submit.prevent="login" class="space-y-4">
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
              <div class="flex justify-center">
                <button
                  type="submit"
                  class="w-72 bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
                >
                  Zaloguj
                </button>
              </div>
            </form>
            <p v-if="errorlog" class="mt-4 text-sm text-red-600">{{ errorlog }}</p>
          </div>
        </div>
        <div v-else>
          <h2 class="text-xl font-semibold mb-4">Zalogowano jako {{ user.username }}</h2>
          <UserUI />
        </div>
        <div v-if="!logged" class="mt-4">
          <div class="flex justify-center">
            <button @click="toggleLogin" class="w-72 bg-gray-300 text-gray-700 py-2 px-4 rounded-md shadow hover:bg-gray-400 focus:outline-none">Zarejestruj się</button>
          </div>
        </div>
      </div>
      <div v-else>
        <div v-if="!logged">
          <Rejestracja />
          <div class="flex justify-center">
            <button @click="toggleLogin" class="w-72 bg-gray-300 text-gray-700 py-2 px-4 rounded-md shadow hover:bg-gray-400 focus:outline-none mt-4">Zaloguj się</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

class User {
    constructor(username, password, gmail, role) {
        this.username = username;
        this.password = password;
        this.gmail = gmail;
        this.role = role;
    }
}

export default {
    data() {
        return {
            username: "",
            password: "",
            logged: false,
            logging: true,
            errorlog: "",
            user: new User("null", "null", "null", "null")
        };
    },
    methods: {
        async login() {
          try {
              const response = await dzien2_backend.zaloguj(this.username, this.password);
              const [isAuthenticated, userOption] = response;

              if (isAuthenticated) {
                  this.logged = true;
                  if (userOption) {
                      this.user = userOption[0];
                  }
              } else {
                  this.errorlog = 'Nieprawidłowa nazwa użytkownika lub hasło';
                  this.user = new User("null", "null", "null", "null");
              }
          } catch (error) {
              console.error("Wystąpił błąd podczas logowania:", error);
              this.errorlog = error;
          }
        },
        async toggleLogin() {
          this.logging = !this.logging;
        },
        async logout() {
            this.logged = false;
        }
    }
}
</script>