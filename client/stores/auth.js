import { defineStore } from 'pinia';

export const useAuthStore = defineStore('auth', () => {
  const initialized = ref(false);
  const authToken = ref(null);
  const user = ref(null);

  const isAuthed = computed(async () => {
    if (!initialized.value) await tryRestore();
    return user.value !== null;
  });

  const fetchUser = async () => {
    const { data } = await useFetch('http://localhost/api/auth/me', {
      headers: {
        'Authentication': `Bearer ${authToken.value}`,
      }
    });
    if (data.value) {
      user.value = data.value;
    }
  };

  const login = async (email, password) => {
    const { data } = await useFetch('http://localhost/api/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: {
        email,
        password,
        confirmPassword: password,
      }
    });

    if (data.value) {
      await localStorage.setItem('authToken', data.value.token);
    }
  };

  const logout = async () => {
    await localStorage.removeItem('authToken');
    window.location = '/';
  };

  const tryRestore = async () => {
    const token = await localStorage.getItem('authToken');
    if (token) {
      authToken.value = token;
      await fetchUser();
    }
    initialized.value = true;
  };

  return { initialized, tryRestore, isAuthed, login, logout, authToken };
});
