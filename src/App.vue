<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const newTodo = ref<Todo>({
  id: 0,
  title: "",
  completed: false,
});
const todoList = ref<Todo[]>([]);

//  Tauri Backend Calls

async function fetchTodos() {
  todoList.value = await invoke("get_todos");
}

async function addTodo() {
  if (newTodo.value.title.trim() === "") return;
  await invoke("add_todo", { title: newTodo.value.title });
  newTodo.value.title = "";
  fetchTodos(); // lädt neue Liste von Backend
}

async function deleteTodo(id: number) {
  await invoke("delete_todo", { id });
  fetchTodos(); // lädt neue Liste von Backend
}

async function updateTodo(id: number) {
  let completed =
    todoList.value.find((todo) => todo.id === id)?.completed || false;

  await invoke("update_todo", { id, completed: !completed });
  fetchTodos(); // lädt neue Liste von Backend
}
onMounted(() => {
  fetchTodos();
});
</script>

<template>
  <main class="m-0 h-screen p-10 flex flex-col gap-10 font-pixel">
    <h1 class="text-4xl">To Do List</h1>

    <form class="flex justify-center" @submit.prevent="addTodo">
      <input
        class="mx-5 bg-game-blue text-game-yellow px-4 p-2 border-2 rounded shadow focus:outline-none focus:ring-2 focus:ring-game-yellow"
        v-model="newTodo.title"
        placeholder="learn tauri..."
      />
      <button
        class="bg-game-red px-4 py-2 text-black hover:bg-game-green hover:text-black transition-all duration-150 ease-pixel"
        type="submit"
      >
        Add
      </button>
    </form>
    <ul class="flex flex-col gap-4 bg-game-bg p-4">
      <li
        :class="[
          'flex justify-between items-center px-4 py-2 rounded shadow relative transition-all duration-150 ease-pixel',
          todo.completed
            ? 'bg-gray-800 text-gray-500 line-through opacity-60 select-none'
            : 'bg-game-blue text-game-yellow hover:border-b-game-yellow hover:border-2',
        ]"
        v-for="(todo, index) in todoList"
        :key="index"
      >
        {{ todo.title }}
        <div>
          <button
            :class="[
              'ml-4 bg-game-green px-2 py-1 text-black hover:bg-game-yellow hover:text-black',
              todo.completed ? 'opacity-100 pointer-events-auto' : '',
            ]"
            @click="updateTodo(todo.id)"
          >
            {{ todo.completed ? "↺" : "✓" }}
          </button>
          <button
            class="ml-4 bg-game-red px-2 py-1 text-black hover:bg-game-yellow hover:text-black transition-all duration-150 ease-pixel"
            @click="deleteTodo(todo.id)"
          >
            X
          </button>
        </div>
        <span
          v-if="todo.completed"
          class="absolute right-[-60px] top-[+30px] text-game-green font-pixel animate-xp font-extrabold"
        >
          +50 XP
        </span>
      </li>
      <!-- XP Pop -->
    </ul>
    <p>{{ greetMsg }}</p>
    <footer class="mt-auto p-4 text-center text-sm text-gray-500">
      <p class="">
        Made with <span class="text-red-500">&#60;3</span> by
        <a
          class="text-blue-500 hover:underline"
          href="https://ibhayb.github.io/"
          target="_blank"
          rel="noopener noreferrer"
          >Ibrahim</a
        >
      </p>
      <p>
        Check out the source code on

        <a
          class="text-blue-100 hover:underline"
          href="https://github.com/ibhayb/minimal-todo-app.git"
          target="_blank"
          rel="noopener noreferrer"
          >GitHub</a
        >
      </p>
    </footer>
  </main>
</template>

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
