<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const newTodo = ref("");
const todoList = ref<string[]>([]);

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { newTodo: newTodo.value });
}

async function addTodo() {
  if (newTodo.value.trim() === "") {
    // Prevent adding empty todos
    return;
  }
  todoList.value.push(newTodo.value);
  newTodo.value = ""; // Clear the input field after adding the todo
}
</script>

<template>
  <main class="m-0 h-screen p-10 flex flex-col gap-10 font-pixel">
    <h1 class="text-4xl">To Do List</h1>

    <form class="flex justify-center" @submit.prevent="addTodo">
      <input
        class="mx-5 bg-game-blue text-game-yellow px-4 p-2 border-2 rounded shadow focus:outline-none focus:ring-2 focus:ring-game-yellow"
        v-model="newTodo"
        placeholder="water plants 🪴"
      />
      <button
        class="bg-game-red px-4 py-2 text-black hover:bg-game-green hover:text-black transition-all duration-150 ease-pixel"
        type="submit"
      >
        Add
      </button>
    </form>
    <ul class="flex flex-col gap-2 bg-game-bg p-4">
      <li
        class="flex justify-between items-center text-game-yellow bg-game-blue px-4 p-2 rounded shadow hover:border-b-game-yellow hover:border-2"
        v-for="(todo, index) in todoList"
        :key="index"
      >
        {{ todo }}
        <button
          class="ml-4 bg-game-red px-2 py-1 text-black hover:bg-game-green hover:text-black transition-all duration-150 ease-pixel"
          @click="todoList.splice(index, 1)"
        >
          X
        </button>
      </li>
    </ul>
    <p>{{ greetMsg }}</p>
    <div></div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
/* Center the app horizontally */
#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  align-items: center;
}
.container {
  margin: 0;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }
}
</style>
