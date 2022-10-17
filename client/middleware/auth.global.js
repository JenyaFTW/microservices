import { useAuthStore } from '../stores/auth';

export default defineNuxtRouteMiddleware(async (to, from) => {
  if (to.name == 'login' || to.name == 'signup') return;

  const { isAuthed } = useAuthStore();
  if (!await isAuthed) {
    return navigateTo('/login');
  }
});
