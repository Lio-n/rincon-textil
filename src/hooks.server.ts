import { redirect, type Handle } from '@sveltejs/kit';
import { invoke } from '@tauri-apps/api/core';

const fetchUserData = async (token: string) => {
  try {
    //   const user = await invoke('get_user', { token });
    const user = { id: '123123', email: 'leonadea@fmaods' };
    return user;
  } catch (error) {
    console.error('Error al obtener datos del usuario:', error);
    throw error;
  }
};

export const handle: Handle = async ({ event, resolve }) => {
  const accessToken = event.cookies.get('access_token');
  const hasToken = accessToken ? true : false;

  if (!hasToken && !event.url.pathname.startsWith('/login')) throw redirect(307, '/login');

  const response = await resolve(event);
  return response;
};
